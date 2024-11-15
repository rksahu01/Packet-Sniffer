use std::cmp::min;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};

use iced::alignment::Horizontal;
use iced::widget::{lazy, vertical_space, Column, Container, Row, Rule, Space, Text};
use iced::{Alignment, Font, Length};

use crate::chart::types::chart_type::ChartType;
use crate::configs::types::config_settings::ConfigSettings;
use crate::countries::country_utils::get_flag_tooltip;
use crate::gui::styles::style_constants::FONT_SIZE_FOOTER;
use crate::gui::styles::types::style_type::StyleType;
use crate::gui::types::message::Message;
use crate::gui::types::sniffer::Sniffer;
use crate::networking::types::host::{Host, ThumbnailHost};
use crate::networking::types::info_traffic::InfoTraffic;
use crate::report::get_report_entries::{get_host_entries, get_service_entries};
use crate::report::types::sort_type::SortType;
use crate::translations::types::language::Language;

const MAX_ENTRIES: usize = 4;
const MAX_CHARS_HOST: usize = 26;
const MAX_CHARS_SERVICE: usize = 13;

/// Computes the body of the thumbnail view
pub fn thumbnail_page(sniffer: &Sniffer) -> Container<Message, StyleType> {
    let ConfigSettings { style, .. } = sniffer.configs.lock().unwrap().settings;
    let font = style.get_extension().font;

    let filtered = sniffer.runtime_data.tot_out_packets + sniffer.runtime_data.tot_in_packets;

    if filtered == 0 {
        return Container::new(
            Column::new()
                .push(vertical_space())
                .push(Text::new(&sniffer.waiting).font(font).size(50))
                .push(Space::with_height(Length::FillPortion(2))),
        )
        .width(Length::Fill)
        .align_x(Horizontal::Center);
    }

    let info_traffic = sniffer.info_traffic.clone();
    let chart_type = sniffer.traffic_chart.chart_type;

    let lazy_report = lazy(filtered, move |_| {
        Row::new()
            .padding([5, 0])
            .height(Length::Fill)
            .align_items(Alignment::Start)
            .push(host_col(&info_traffic, chart_type, font))
            .push(Rule::vertical(10))
            .push(service_col(&info_traffic, chart_type, font))
    });

    let content = Column::new()
        .push(Container::new(sniffer.traffic_chart.view()).height(Length::Fill))
        .push(lazy_report);

    Container::new(content)
}

fn host_col(
    info_traffic: &Arc<Mutex<InfoTraffic>>,
    chart_type: ChartType,
    font: Font,
) -> Column<'static, Message, StyleType> {
    let mut host_col = Column::new()
        .padding([0, 5])
        .spacing(3)
        .width(Length::FillPortion(2));
    let hosts = get_host_entries(info_traffic, chart_type, SortType::Neutral);
    let mut thumbnail_hosts = Vec::new();

    for (host, data_info_host) in &hosts {
        let text = host_text(host);
        let country = host.country;
        let thumbnail_host = ThumbnailHost {
            country,
            text: text.clone(),
        };

        if thumbnail_hosts.contains(&thumbnail_host) {
            continue;
        }

        thumbnail_hosts.push(thumbnail_host);

        let flag = get_flag_tooltip(country, data_info_host, Language::default(), font, true);
        let host_row = Row::new()
            .align_items(Alignment::Center)
            .spacing(5)
            .push(flag)
            .push(Text::new(text).font(font).size(FONT_SIZE_FOOTER));
        host_col = host_col.push(host_row);

        if thumbnail_hosts.len() >= MAX_ENTRIES {
            break;
        }
    }

    host_col
}

fn service_col(
    info_traffic: &Arc<Mutex<InfoTraffic>>,
    chart_type: ChartType,
    font: Font,
) -> Column<'static, Message, StyleType> {
    let mut service_col = Column::new().padding([0, 5]).spacing(3).width(Length::Fill);
    let services = get_service_entries(info_traffic, chart_type, SortType::Neutral);
    let n_entry = min(services.len(), MAX_ENTRIES);
    for (service, _) in services.get(..n_entry).unwrap_or_default() {
        service_col = service_col.push(
            Text::new(clip_text(&service.to_string(), MAX_CHARS_SERVICE))
                .font(font)
                .size(FONT_SIZE_FOOTER),
        );
    }
    service_col
}

fn host_text(host: &Host) -> String {
    let domain = &host.domain;
    let asn = &host.asn.name;

    let text = if asn.is_empty() || (!domain.trim().is_empty() && domain.parse::<IpAddr>().is_err())
    {
        domain
    } else {
        asn
    };

    clip_text(text, MAX_CHARS_HOST)
}

fn clip_text(text: &str, max_chars: usize) -> String {
    let text = text.trim();
    let chars = text.chars().collect::<Vec<char>>();
    let tot_len = chars.len();
    let slice_len = min(max_chars, tot_len);

    let suspensions = if tot_len > max_chars { "…" } else { "" };
    let slice = if tot_len > max_chars {
        &chars[..slice_len - 2]
    } else {
        &chars[..slice_len]
    }
    .iter()
    .collect::<String>();

    [slice.trim(), suspensions].concat()
}
