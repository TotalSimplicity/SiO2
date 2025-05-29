#! [allow(dead_code)]
use crate::stockdata::RawStockData;
use crate::averaging;


impl RawStockData {
    /// Function to normalize the current volume against the previous volumes. Previous vols should NOT include current volume.
    pub fn normalize_volume(current_vol: f64, previous_vols: Vec<f64>, lookback_len: i64) -> f64 {
        if previous_vols.len() < lookback_len as usize {
            eprintln!("Not enough previous volumes to normalize against. Returned current vol.");
            return current_vol;
        }
        let previous_vols_slice = &previous_vols[previous_vols.len() - lookback_len as usize..];
        let sma = averaging::simple_moving_average(previous_vols_slice, lookback_len as usize)
            .unwrap_or(current_vol);
        if sma == 0.0 {
            eprintln!("SMA is zero, returning current volume.");
            return current_vol;
        }
        let normalize_ratio = current_vol / sma;
        let normal_val = normalize_function(normalize_ratio);
        return normal_val * 100.0;
    }

    pub fn normalize_price_strength(
        current_item: &RawStockData,
        previous_items: &[RawStockData],
        lookback_len: i64
    ) -> f64 {
        if previous_items.len() < lookback_len as usize {
            eprintln!("Not enough previous items to normalize against. Returning 0.0 for price strength.");
            return 0.0;
        }

        let bar_spread = current_item.close - current_item.open;
        let bar_range = current_item.high - current_item.low;
        let bar_closing_percent = if bar_range != 0.0 {
            2.0 * (current_item.close - current_item.low) / bar_range * 100.0 - 100.0
        } else {
            0.0
        };
        let spread_to_range_ratio = if bar_range != 0.0 {
            bar_spread / bar_range * 100.0
        } else {
            0.0
        };

        let absolute_spread = bar_spread.abs();
        let historical_absolute_spreads: Vec<f64> = previous_items
            .iter()
            .map(|b| (b.close - b.open).abs())
            .collect();
        let relative_spread_normalized = if historical_absolute_spreads.len() >= lookback_len as usize {
            let recent_absolute_spreads = &historical_absolute_spreads[historical_absolute_spreads.len() - lookback_len as usize..];
            let average_bar_spread = match averaging::simple_moving_average(recent_absolute_spreads, lookback_len as usize) {
                Some(avg) => avg,
                None => absolute_spread,
            };
            let spread_ratio = if average_bar_spread != 0.0 { absolute_spread / average_bar_spread } else { 0.0 };
            normalize_function(spread_ratio) * 100.0 * bar_spread.signum()
        } else {
            0.0
        };

        let (bar_closing_2_percent, shift_2_bar_to_range_ratio, relative_shift_normalized_2bar) =
            if previous_items.len() >= 1 {
                let previous_item = &previous_items[previous_items.len() - 1];
                let two_bar_high = current_item.high.max(previous_item.high);
                let two_bar_low = current_item.low.min(previous_item.low);
                let two_bar_range = two_bar_high - two_bar_low;
                let bar_closing_2_percent = if two_bar_range != 0.0 {
                    2.0 * (current_item.close - two_bar_low) / two_bar_range * 100.0 - 100.0
                } else {
                    0.0
                };

                let price_shift = current_item.close - previous_item.close;
                let shift_2_bar_to_range_ratio = if two_bar_range != 0.0 {
                    price_shift / two_bar_range * 100.0
                } else {
                    0.0
                };

                let absolute_price_shift = price_shift.abs();
                let historical_price_shifts: Vec<f64> = previous_items
                    .iter()
                    .map(|b| if let Some(prev) = previous_items.get(previous_items.iter().position(|x| x.timestamp == b.timestamp).unwrap().saturating_sub(1)) {
                        (b.close - prev.close).abs()
                    } else {
                        0.0
                    })
                    .collect();

                let relative_shift_normalized_2bar = if historical_price_shifts.len() >= lookback_len as usize {
                    let recent_price_shifts = &historical_price_shifts[historical_price_shifts.len() - lookback_len as usize..];
                    let average_price_shift = match averaging::simple_moving_average(recent_price_shifts, lookback_len as usize) {
                        Some(avg) => avg,
                        None => absolute_price_shift,
                    };
                    let shift_ratio = if average_price_shift != 0.0 { absolute_price_shift / average_price_shift } else { 0.0 };
                    normalize_function(shift_ratio) * 100.0 * price_shift.signum()
                } else {
                    0.0
                };
                (bar_closing_2_percent, shift_2_bar_to_range_ratio, relative_shift_normalized_2bar)
            } else {
                (0.0, 0.0, 0.0)
            };

        (bar_closing_percent + spread_to_range_ratio + relative_spread_normalized + bar_closing_2_percent + shift_2_bar_to_range_ratio + relative_shift_normalized_2bar) / 6.0
    }

    pub fn calculate_bar_flow(
        current_item: &RawStockData,
        previous_items: &[RawStockData],
        lookback_len: i64
    ) -> f64 {

        let previous_vols: Vec<f64> = previous_items.iter().map(|item| item.volume as f64).collect();

        let vola_n = RawStockData::normalize_volume(
            current_item.volume as f64,
            previous_vols,
            lookback_len
        );

        let pricea_n = RawStockData::normalize_price_strength(
            current_item,
            previous_items,
            lookback_len
        );
        pricea_n * vola_n / 100.0
    }

    

    pub fn calculate_rrof(
        bar_flow_values: &[f64],
        length: usize
    ) -> Option<f64> {
        if bar_flow_values.len() < length || length == 0 {
            return None; // Not enough data
        }

        let mut bullish_flows = Vec::with_capacity(bar_flow_values.len());
        let mut bearish_flows = Vec::with_capacity(bar_flow_values.len());

        for &flow_value in bar_flow_values {
            let bullish = flow_value.max(0.0);
            let bearish = flow_value.min(0.0).abs();
            bullish_flows.push(bullish);
            bearish_flows.push(bearish);
        }

        let recent_bullish_flows = &bullish_flows[bullish_flows.len() - length..];
        let recent_bearish_flows = &bearish_flows[bearish_flows.len() - length..];

        let avg_bullish_flow = averaging::simple_moving_average(recent_bullish_flows, length)?;
        let avg_bearish_flow = averaging::simple_moving_average(recent_bearish_flows, length)?;

        if avg_bearish_flow == 0.0 {
            return Some(0.0); // Avoid division by zero
        }

        let dx = avg_bullish_flow / avg_bearish_flow;
        let rrof = 2.0 * (100.0 - (100.0 / (1.0 + dx))) - 100.0;

        Some(rrof)
    }

    pub fn calculate_rrof_smooth(rrof_values: &[f64], smooth_length: usize) -> Option<f64> {
        if rrof_values.len() < smooth_length || smooth_length == 0 {
            return None;
        }

        let recent_rrof_values = &rrof_values[rrof_values.len() - smooth_length..];
        averaging::simple_moving_average(recent_rrof_values, smooth_length)
    }


    pub fn calculate_signal_line(rrof_smooth_values: &[f64], signal_length: usize) -> Option<f64> {
        if rrof_smooth_values.len() < signal_length || signal_length == 0 {
            return None;
        }

        let recent_rrof_smooth_values = &rrof_smooth_values[rrof_smooth_values.len() - signal_length..];
        averaging::simple_moving_average(recent_rrof_smooth_values, signal_length)
    }

}

fn normalize_function(ratio: f64) -> f64 {
    if ratio > 1.50 {
        1.00
    } else if ratio > 1.20 {
        0.90
    } else if ratio > 1.00 {
        0.80
    } else if ratio > 0.80 {
        0.70
    } else if ratio > 0.60 {
        0.60
    } else if ratio > 0.40 {
        0.50
    } else if ratio > 0.20 {
        0.25
    } else {
        0.1
    }
}

