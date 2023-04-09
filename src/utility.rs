#![feature(convert_float_to_int)]
use bytes::*;
use libm::modf;
use math::*;
use std::cmp::max;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::FloatToInt;
use std::f64;
use std::f64::consts::PI;
use std::fmt;
use std::string::String;

pub fn min_max_float64_slice(v: &[f64]) -> (f64, f64) {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;

    if v.is_empty() {
        panic!("Empty slice");
    }

    for &e in v {
        if e < min {
            min = e;
        }
        if e > max {
            max = e;
        }
    }
    (min, max)
}

pub fn round(new_input: f64) -> f64 {
    if new_input.is_nan() {
        return f64::NAN;
    }
    let mut sign = 1.0;
    let mut abs_input = new_input;
    if new_input < 0.0 {
        sign = -1.0;
        abs_input *= -1.0;
    }

    let (integer, decimal) = modf(abs_input);
    let rounded = if decimal >= 0.5 {
        f64::ceil(abs_input)
    } else {
        f64::floor(abs_input)
    };
    rounded * sign
}

pub fn linear_interpolate(before: f64, after: f64, at_point: f64) -> f64 {
    before + (after - before) * at_point
}

pub fn interpolate_array(data: &[f64], fit_count: usize) -> Vec<f64> {
    let mut interpolated_data = Vec::new();

    let spring_factor = (data.len() - 1) as f64 / (fit_count - 1) as f64;
    interpolated_data.push(data[0]);

    for i in 1..fit_count - 1 {
        let spring = i as f64 * spring_factor;
        let before = f64::floor(spring);
        let after = f64::ceil(spring);
        let at_point = spring - before;
        interpolated_data.push(linear_interpolate(
            data[before as usize],
            data[after as usize],
            at_point,
        ));
    }
    interpolated_data.push(data[data.len() - 1]);
    interpolated_data
}

#[cfg(windows)]
pub fn clear() {
    let _ = std::process::Command::new("cmd")
        .arg("/c")
        .arg("cls")
        .status();
}

#[cfg(not(windows))]
pub fn clear() {
    print!("\x1B[2J\x1B[H");
}
