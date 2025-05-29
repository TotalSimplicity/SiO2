#! [allow(unused_imports)]
#! [allow(dead_code)]

use average::{
    Mean,
    WeightedMean
};

pub fn simple_moving_average(data: &[f64], length: usize) -> Option<f64> {
    if data.len() < length || length == 0 {
        return None;
    }
    let slice = &data[data.len() - length..];
    Some(slice.iter().sum::<f64>() / length as f64)
}

fn exponential_moving_average(data: &[f64], length: usize) -> Option<f64> {
    if data.is_empty() || length == 0 {
        return None;
    }
    let alpha = 2.0 / (length as f64 + 1.0);
    let mut ema = data[0];
    for &value in data.iter().skip(1) {
        ema = alpha * value + (1.0 - alpha) * ema;
    }
    Some(ema)
}

fn weighted_moving_average(data: &[f64], length: usize) -> Option<f64> {
    if data.len() < length || length == 0 {
        return None;
    }
    let slice = &data[data.len() - length..];
    let mut weighted_sum = 0.0;
    let mut weight_sum = 0.0;
    for (i, &value) in slice.iter().enumerate() {
        let weight = (i + 1) as f64;
        weighted_sum += value * weight;
        weight_sum += weight;
    }
    if weight_sum == 0.0 {
        return None;
    }
    Some(weighted_sum / weight_sum)
}
