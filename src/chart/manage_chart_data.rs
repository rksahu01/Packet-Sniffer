use splines::{Interpolation, Key, Spline};

use crate::{RunTimeData, TrafficChart};

/// This function is invoked every second by the application subscription
///
/// It updates data (packets and bytes per second) to be displayed in the chart of gui run page
pub fn update_charts_data(runtime_data: &mut RunTimeData, traffic_chart: &mut TrafficChart) {
    #[allow(clippy::cast_precision_loss)]
    let tot_seconds = traffic_chart.ticks as f32;
    traffic_chart.ticks += 1;

    #[allow(clippy::cast_precision_loss)]
    let out_bytes_entry =
        -1.0 * (runtime_data.tot_out_bytes - runtime_data.tot_out_bytes_prev) as f32;
    #[allow(clippy::cast_precision_loss)]
    let in_bytes_entry = (runtime_data.tot_in_bytes - runtime_data.tot_in_bytes_prev) as f32;
    #[allow(clippy::cast_precision_loss)]
    let out_packets_entry =
        -1.0 * (runtime_data.tot_out_packets - runtime_data.tot_out_packets_prev) as f32;
    #[allow(clippy::cast_precision_loss)]
    let in_packets_entry = (runtime_data.tot_in_packets - runtime_data.tot_in_packets_prev) as f32;

    let out_bytes_key = Key::new(tot_seconds, out_bytes_entry, Interpolation::Cosine);
    let in_bytes_key = Key::new(tot_seconds, in_bytes_entry, Interpolation::Cosine);
    let out_packets_key = Key::new(tot_seconds, out_packets_entry, Interpolation::Cosine);
    let in_packets_key = Key::new(tot_seconds, in_packets_entry, Interpolation::Cosine);

    // update sent bytes traffic data
    update_spline(&mut traffic_chart.out_bytes, out_bytes_key);
    traffic_chart.min_bytes = get_min(&traffic_chart.out_bytes);
    runtime_data.tot_out_bytes_prev = runtime_data.tot_out_bytes;

    // update received bytes traffic data
    update_spline(&mut traffic_chart.in_bytes, in_bytes_key);
    traffic_chart.max_bytes = get_max(&traffic_chart.in_bytes);
    runtime_data.tot_in_bytes_prev = runtime_data.tot_in_bytes;

    // update sent packets traffic data
    update_spline(&mut traffic_chart.out_packets, out_packets_key);
    traffic_chart.min_packets = get_min(&traffic_chart.out_packets);
    runtime_data.tot_out_packets_prev = runtime_data.tot_out_packets;

    // update received packets traffic data
    update_spline(&mut traffic_chart.in_packets, in_packets_key);
    traffic_chart.max_packets = get_max(&traffic_chart.in_packets);
    runtime_data.tot_in_packets_prev = runtime_data.tot_in_packets;
}

fn update_spline(spline: &mut Spline<f32, f32>, new_key: Key<f32, f32>) {
    if spline.len() >= 30 {
        spline.remove(0);
    }
    spline.add(new_key);
}

/// Finds the minimum y value to be displayed in chart.
fn get_min(spline: &Spline<f32, f32>) -> f32 {
    let mut min = 0.0;
    for key in spline {
        if key.value < min {
            min = key.value;
        }
    }
    min
}

/// Finds the maximum y value to be displayed in chart.
fn get_max(spline: &Spline<f32, f32>) -> f32 {
    let mut max = 0.0;
    for key in spline {
        if key.value > max {
            max = key.value;
        }
    }
    max
}
