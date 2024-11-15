use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use chrono::Local;
use dns_lookup::lookup_addr;
use etherparse::{LaxPacketHeaders, LinkHeader, NetHeaders, TransportHeader};
use pcap::{Address, Device};

use crate::mmdb::asn::get_asn;
use crate::mmdb::country::get_country;
use crate::mmdb::types::mmdb_reader::MmdbReader;
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::host::Host;
use crate::networking::types::icmp_type::{IcmpType, IcmpTypeV4, IcmpTypeV6};
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::networking::types::my_device::MyDevice;
use crate::networking::types::packet_filters_fields::PacketFiltersFields;
use crate::networking::types::service::Service;
use crate::networking::types::service_query::ServiceQuery;
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::networking::types::traffic_type::TrafficType;
use crate::utils::formatted_strings::get_domain_from_r_dns;
use crate::IpVersion::{IPv4, IPv6};
use crate::{InfoTraffic, IpVersion, Protocol};

include!(concat!(env!("OUT_DIR"), "/services.rs"));

/// Calls methods to analyze link, network, and transport headers.
/// Returns the relevant collected information.
pub fn analyze_headers(
    headers: LaxPacketHeaders,
    mac_addresses: &mut (Option<String>, Option<String>),
    exchanged_bytes: &mut u128,
    icmp_type: &mut IcmpType,
    packet_filters_fields: &mut PacketFiltersFields,
) -> Option<AddressPortPair> {
    analyze_link_header(
        headers.link,
        &mut mac_addresses.0,
        &mut mac_addresses.1,
        exchanged_bytes,
    );

    if !analyze_network_header(
        headers.net,
        exchanged_bytes,
        &mut packet_filters_fields.ip_version,
        &mut packet_filters_fields.source,
        &mut packet_filters_fields.dest,
    ) {
        return None;
    }

    if !analyze_transport_header(
        headers.transport,
        &mut packet_filters_fields.sport,
        &mut packet_filters_fields.dport,
        &mut packet_filters_fields.protocol,
        icmp_type,
    ) {
        return None;
    }

    Some(AddressPortPair::new(
        packet_filters_fields.source.to_string(),
        packet_filters_fields.sport,
        packet_filters_fields.dest.to_string(),
        packet_filters_fields.dport,
        packet_filters_fields.protocol,
    ))
}

/// This function analyzes the data link layer header passed as parameter and updates variables
/// passed by reference on the basis of the packet header content.
/// Returns false if packet has to be skipped.
fn analyze_link_header(
    link_header: Option<LinkHeader>,
    mac_address1: &mut Option<String>,
    mac_address2: &mut Option<String>,
    exchanged_bytes: &mut u128,
) {
    if let Some(LinkHeader::Ethernet2(header)) = link_header {
        *exchanged_bytes += 14;
        *mac_address1 = Some(mac_from_dec_to_hex(header.source));
        *mac_address2 = Some(mac_from_dec_to_hex(header.destination));
    } else {
        *mac_address1 = None;
        *mac_address2 = None;
    }
}

/// This function analyzes the network layer header passed as parameter and updates variables
/// passed by reference on the basis of the packet header content.
/// Returns false if packet has to be skipped.
fn analyze_network_header(
    network_header: Option<NetHeaders>,
    exchanged_bytes: &mut u128,
    network_protocol: &mut IpVersion,
    address1: &mut IpAddr,
    address2: &mut IpAddr,
) -> bool {
    match network_header {
        Some(NetHeaders::Ipv4(ipv4header, _)) => {
            *network_protocol = IpVersion::IPv4;
            *address1 = IpAddr::from(ipv4header.source);
            *address2 = IpAddr::from(ipv4header.destination);
            *exchanged_bytes += u128::from(ipv4header.total_len);
            true
        }
        Some(NetHeaders::Ipv6(ipv6header, _)) => {
            *network_protocol = IpVersion::IPv6;
            *address1 = IpAddr::from(ipv6header.source);
            *address2 = IpAddr::from(ipv6header.destination);
            *exchanged_bytes += u128::from(40 + ipv6header.payload_length);
            true
        }
        _ => false,
    }
}

/// This function analyzes the transport layer header passed as parameter and updates variables
/// passed by reference on the basis of the packet header content.
/// Returns false if packet has to be skipped.
fn analyze_transport_header(
    transport_header: Option<TransportHeader>,
    port1: &mut Option<u16>,
    port2: &mut Option<u16>,
    protocol: &mut Protocol,
    icmp_type: &mut IcmpType,
) -> bool {
    match transport_header {
        Some(TransportHeader::Udp(udp_header)) => {
            *port1 = Some(udp_header.source_port);
            *port2 = Some(udp_header.destination_port);
            *protocol = Protocol::UDP;
            true
        }
        Some(TransportHeader::Tcp(tcp_header)) => {
            *port1 = Some(tcp_header.source_port);
            *port2 = Some(tcp_header.destination_port);
            *protocol = Protocol::TCP;
            true
        }
        Some(TransportHeader::Icmpv4(icmpv4_header)) => {
            *port1 = None;
            *port2 = None;
            *protocol = Protocol::ICMP;
            *icmp_type = IcmpTypeV4::from_etherparse(&icmpv4_header.icmp_type);
            true
        }
        Some(TransportHeader::Icmpv6(icmpv6_header)) => {
            *port1 = None;
            *port2 = None;
            *protocol = Protocol::ICMP;
            *icmp_type = IcmpTypeV6::from_etherparse(&icmpv6_header.icmp_type);
            true
        }
        _ => false,
    }
}

pub fn get_service(key: &AddressPortPair, traffic_direction: TrafficDirection) -> Service {
    if key.port1.is_none() || key.port2.is_none() || key.protocol == Protocol::ICMP {
        return Service::NotApplicable;
    }

    // to return the service associated with the highest score:
    // score = service_is_some * (port_is_well_known + bonus_direction)
    // service_is_some: 1 if some, 0 if unknown
    // port_is_well_known: 3 if well known, 1 if not
    // bonus_direction: +1 assigned to remote port
    let compute_service_score = |service: &Service, port: u16, bonus_direction: bool| {
        let service_is_some = u8::from(matches!(service, Service::Name(_)));
        let port_is_well_known = if port < 1024 { 3 } else { 1 };
        let bonus_direction = u8::from(bonus_direction);
        service_is_some * (port_is_well_known + bonus_direction)
    };

    let port1 = key.port1.unwrap();
    let port2 = key.port2.unwrap();

    let unknown = Service::Unknown;
    let service1 = SERVICES
        .get(&ServiceQuery(port1, key.protocol))
        .unwrap_or(&unknown);
    let service2 = SERVICES
        .get(&ServiceQuery(port2, key.protocol))
        .unwrap_or(&unknown);

    let score1 = compute_service_score(
        service1,
        port1,
        traffic_direction.ne(&TrafficDirection::Outgoing),
    );
    let score2 = compute_service_score(
        service2,
        port2,
        traffic_direction.eq(&TrafficDirection::Outgoing),
    );

    if score1 > score2 {
        *service1
    } else {
        *service2
    }
}

/// Function to insert the source and destination of a packet into the shared map containing the analyzed traffic.
pub fn modify_or_insert_in_map(
    info_traffic_mutex: &Arc<Mutex<InfoTraffic>>,
    key: &AddressPortPair,
    my_device: &MyDevice,
    mac_addresses: (Option<String>, Option<String>),
    icmp_type: IcmpType,
    exchanged_bytes: u128,
) -> InfoAddressPortPair {
    let now = Local::now();
    let mut traffic_direction = TrafficDirection::default();
    let mut service = Service::Unknown;

    if !info_traffic_mutex.lock().unwrap().map.contains_key(key) {
        // first occurrence of key

        // update device addresses
        let mut my_interface_addresses = Vec::new();
        for dev in Device::list().expect("Error retrieving device list\r\n") {
            if dev.name.eq(&my_device.name) {
                let mut my_interface_addresses_mutex = my_device.addresses.lock().unwrap();
                my_interface_addresses_mutex.clone_from(&dev.addresses);
                drop(my_interface_addresses_mutex);
                my_interface_addresses = dev.addresses;
                break;
            }
        }
        // determine traffic direction
        let source_ip = &key.address1;
        let destination_ip = &key.address2;
        traffic_direction = get_traffic_direction(
            source_ip,
            destination_ip,
            key.port1,
            key.port2,
            &my_interface_addresses,
        );
        // determine upper layer service
        service = get_service(key, traffic_direction);
    };

    let mut info_traffic = info_traffic_mutex
        .lock()
        .expect("Error acquiring mutex\n\r");

    let new_info: InfoAddressPortPair = info_traffic
        .map
        .entry(key.clone())
        .and_modify(|info| {
            info.transmitted_bytes += exchanged_bytes;
            info.transmitted_packets += 1;
            info.final_timestamp = now;
            if key.protocol.eq(&Protocol::ICMP) {
                info.icmp_types
                    .entry(icmp_type)
                    .and_modify(|n| *n += 1)
                    .or_insert(1);
            }
        })
        .or_insert_with(|| InfoAddressPortPair {
            mac_address1: mac_addresses.0,
            mac_address2: mac_addresses.1,
            transmitted_bytes: exchanged_bytes,
            transmitted_packets: 1,
            initial_timestamp: now,
            final_timestamp: now,
            service,
            traffic_direction,
            icmp_types: if key.protocol.eq(&Protocol::ICMP) {
                HashMap::from([(icmp_type, 1)])
            } else {
                HashMap::new()
            },
        })
        .clone();

    if let Some(host_info) = info_traffic
        .addresses_resolved
        .get(&get_address_to_lookup(key, new_info.traffic_direction))
        .cloned()
    {
        if info_traffic.favorite_hosts.contains(&host_info.1) {
            info_traffic.favorites_last_interval.insert(host_info.1);
        }
    }

    new_info
}

pub fn reverse_dns_lookup(
    info_traffic: &Arc<Mutex<InfoTraffic>>,
    key: &AddressPortPair,
    traffic_direction: TrafficDirection,
    my_device: &MyDevice,
    country_db_reader: &Arc<MmdbReader>,
    asn_db_reader: &Arc<MmdbReader>,
) {
    let address_to_lookup = get_address_to_lookup(key, traffic_direction);
    let my_interface_addresses = my_device.addresses.lock().unwrap().clone();

    // perform rDNS lookup
    let lookup_result = lookup_addr(&address_to_lookup.parse().unwrap());

    // get new host info and build the new host
    let traffic_type = get_traffic_type(
        &address_to_lookup,
        &my_interface_addresses,
        traffic_direction,
    );
    let is_loopback = is_loopback(&address_to_lookup);
    let is_local = is_local_connection(&address_to_lookup, &my_interface_addresses);
    let country = get_country(&address_to_lookup, country_db_reader);
    let asn = get_asn(&address_to_lookup, asn_db_reader);
    let r_dns = if let Ok(result) = lookup_result {
        if result.is_empty() {
            address_to_lookup.clone()
        } else {
            result
        }
    } else {
        address_to_lookup.clone()
    };
    let new_host = Host {
        domain: get_domain_from_r_dns(r_dns.clone()),
        asn,
        country,
    };

    let mut info_traffic_lock = info_traffic.lock().unwrap();
    // collect the data exchanged from the same address so far and remove the address from the collection of addresses waiting a rDNS
    let other_data = info_traffic_lock
        .addresses_waiting_resolution
        .remove(&address_to_lookup)
        .unwrap_or_default();
    // insert the newly resolved host in the collections, with the data it exchanged so far
    info_traffic_lock
        .addresses_resolved
        .insert(address_to_lookup, (r_dns, new_host.clone()));
    info_traffic_lock
        .hosts
        .entry(new_host.clone())
        .and_modify(|data_info_host| {
            data_info_host.data_info += other_data;
        })
        .or_insert_with(|| DataInfoHost {
            data_info: other_data,
            is_favorite: false,
            is_loopback,
            is_local,
            traffic_type,
        });
    // check if the newly resolved host was featured in the favorites (possible in case of already existing host)
    if info_traffic_lock.favorite_hosts.contains(&new_host) {
        info_traffic_lock.favorites_last_interval.insert(new_host);
    }

    drop(info_traffic_lock);
}

/// Returns the traffic direction observed (incoming or outgoing)
fn get_traffic_direction(
    source_ip: &String,
    destination_ip: &String,
    source_port: Option<u16>,
    dest_port: Option<u16>,
    my_interface_addresses: &[Address],
) -> TrafficDirection {
    let my_interface_addresses_string: Vec<String> = my_interface_addresses
        .iter()
        .map(|address| address.addr.to_string())
        .collect();

    // first let's handle TCP and UDP loopback
    if is_loopback(source_ip) && is_loopback(destination_ip) {
        if let (Some(sport), Some(dport)) = (source_port, dest_port) {
            return if sport > dport {
                TrafficDirection::Outgoing
            } else {
                TrafficDirection::Incoming
            };
        }
    }

    if my_interface_addresses_string.contains(source_ip) {
        // source is local
        TrafficDirection::Outgoing
    } else if source_ip.ne("0.0.0.0") && source_ip.ne("::") {
        // source not local and different from 0.0.0.0 and different from ::
        TrafficDirection::Incoming
    } else if !my_interface_addresses_string.contains(destination_ip) {
        // source is 0.0.0.0 or :: (local not yet assigned an IP) and destination is not local
        TrafficDirection::Outgoing
    } else {
        TrafficDirection::Incoming
    }
}

/// Returns the traffic type observed (unicast, multicast or broadcast)
/// It refers to the remote host
pub fn get_traffic_type(
    destination_ip: &str,
    my_interface_addresses: &[Address],
    traffic_direction: TrafficDirection,
) -> TrafficType {
    if traffic_direction.eq(&TrafficDirection::Outgoing) {
        if is_multicast_address(destination_ip) {
            TrafficType::Multicast
        } else if is_broadcast_address(destination_ip, my_interface_addresses) {
            TrafficType::Broadcast
        } else {
            TrafficType::Unicast
        }
    } else {
        TrafficType::Unicast
    }
}

/// Determines if the input address is a multicast address or not.
///
/// # Arguments
///
/// * `address` - string representing an IPv4 or IPv6 network address.
fn is_multicast_address(address: &str) -> bool {
    let mut ret_val = false;
    if address.contains(':') {
        //IPv6 address
        if address.starts_with("ff") {
            ret_val = true;
        }
    } else {
        //IPv4 address
        let first_group = address
            .split('.')
            .next()
            .unwrap()
            .to_string()
            .parse::<u8>()
            .unwrap();
        if (224..=239).contains(&first_group) {
            ret_val = true;
        }
    }
    ret_val
}

/// Determines if the input address is a broadcast address or not.
///
/// # Arguments
///
/// * `address` - string representing an IPv4 or IPv6 network address.
fn is_broadcast_address(address: &str, my_interface_addresses: &[Address]) -> bool {
    if address.eq("255.255.255.255") {
        return true;
    }
    // check if directed broadcast
    let my_broadcast_addresses: Vec<String> = my_interface_addresses
        .iter()
        .map(|address| {
            address
                .broadcast_addr
                .unwrap_or_else(|| "255.255.255.255".parse().unwrap())
                .to_string()
        })
        .collect();
    if my_broadcast_addresses.contains(&address.to_string()) {
        return true;
    }
    false
}

fn is_loopback(address_to_lookup: &str) -> bool {
    IpAddr::from_str(address_to_lookup)
        .unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED))
        .is_loopback()
}

/// Determines if the connection is local
pub fn is_local_connection(address_to_lookup: &str, my_interface_addresses: &Vec<Address>) -> bool {
    let mut ret_val = false;

    let address_to_lookup_type = if address_to_lookup.contains(':') {
        IPv6
    } else {
        IPv4
    };

    for address in my_interface_addresses {
        match address.addr {
            IpAddr::V4(local_addr) if address_to_lookup_type.eq(&IPv4) => {
                // check if the two IPv4 addresses are in the same subnet
                let address_to_lookup_parsed: Ipv4Addr = address_to_lookup
                    .parse()
                    .unwrap_or_else(|_| Ipv4Addr::from(0));
                // remote is link local?
                if address_to_lookup_parsed.is_link_local() {
                    ret_val = true;
                }
                // is the same subnet?
                else if let Some(IpAddr::V4(netmask)) = address.netmask {
                    let mut local_subnet = Vec::new();
                    let mut remote_subnet = Vec::new();
                    let netmask_digits = netmask.octets();
                    let local_addr_digits = local_addr.octets();
                    let remote_addr_digits = address_to_lookup_parsed.octets();
                    for (i, netmask_digit) in netmask_digits.iter().enumerate() {
                        local_subnet.push(netmask_digit & local_addr_digits[i]);
                        remote_subnet.push(netmask_digit & remote_addr_digits[i]);
                    }
                    if local_subnet == remote_subnet {
                        ret_val = true;
                    }
                }
            }
            IpAddr::V6(local_addr) if address_to_lookup_type.eq(&IPv6) => {
                // check if the two IPv6 addresses are in the same subnet
                let address_to_lookup_parsed: Ipv6Addr = address_to_lookup
                    .parse()
                    .unwrap_or_else(|_| Ipv6Addr::from(0));
                // remote is link local?
                if address_to_lookup.starts_with("fe80") {
                    ret_val = true;
                }
                // is the same subnet?
                else if let Some(IpAddr::V6(netmask)) = address.netmask {
                    let mut local_subnet = Vec::new();
                    let mut remote_subnet = Vec::new();
                    let netmask_digits = netmask.octets();
                    let local_addr_digits = local_addr.octets();
                    let remote_addr_digits = address_to_lookup_parsed.octets();
                    for (i, netmask_digit) in netmask_digits.iter().enumerate() {
                        local_subnet.push(netmask_digit & local_addr_digits[i]);
                        remote_subnet.push(netmask_digit & remote_addr_digits[i]);
                    }
                    if local_subnet == remote_subnet {
                        ret_val = true;
                    }
                }
            }
            _ => {}
        }
    }

    ret_val
}

/// Determines if the address passed as parameter belong to the chosen adapter
pub fn is_my_address(local_address: &String, my_interface_addresses: &Vec<Address>) -> bool {
    for address in my_interface_addresses {
        if address.addr.to_string().eq(local_address) {
            return true;
        }
    }
    is_loopback(local_address)
}

/// Converts a MAC address in its hexadecimal form
fn mac_from_dec_to_hex(mac_dec: [u8; 6]) -> String {
    let mut mac_hex = String::new();
    for n in &mac_dec {
        mac_hex.push_str(&format!("{n:02x}:"));
    }
    mac_hex.pop();
    mac_hex
}

pub fn get_address_to_lookup(key: &AddressPortPair, traffic_direction: TrafficDirection) -> String {
    match traffic_direction {
        TrafficDirection::Outgoing => key.address2.clone(),
        TrafficDirection::Incoming => key.address1.clone(),
    }
}
