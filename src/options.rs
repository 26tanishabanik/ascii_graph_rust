#![feature(convert_float_to_int)]
use crate::color;
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

pub struct Config {
    width: i32,
    height: i32,
    offset: i32,
    caption: String,
    precision: u32,
    caption_color: color::AnsiColor,
    axis_color: color::AnsiColor,
    label_color: color::AnsiColor,
    series_colors: Vec<color::AnsiColor>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            width: 150,
            height: 10,
            offset: 3,
            caption: String::new(),
            precision: 2,
            caption_color: color::AnsiColor::White,
            axis_color: color::AnsiColor::White,
            label_color: color::AnsiColor::AliceBlue,
            series_colors: vec![
                color::AnsiColor::AliceBlue,
                color::AnsiColor::Silver,
                color::AnsiColor::White,
                color::AnsiColor::Black,
            ],
        }
    }
}

trait GraphOption {
    fn apply(&self, c: &mut Config);
}

pub struct GraphOptionFunc<F: Fn(&mut Config)> {
    apply_func: F,
}

impl<F: Fn(&mut Config)> GraphOption for GraphOptionFunc<F> {
    fn apply(&self, c: &mut Config) {
        (self.apply_func)(c);
    }
}

pub fn width(w: i32) -> Box<dyn GraphOption> {
    Box::new(GraphOptionFunc {
        apply_func: move |c: &mut Config| {
            if w > 0 {
                c.width = w;
            } else {
                c.width = 0;
            }
        },
    })
}

pub fn configure(mut defaults: Config, options: Vec<Box<dyn GraphOption>>) -> Config {
    for o in options {
        o.apply(&mut defaults);
    }
    defaults
}

pub fn height(h: i32) -> Box<dyn GraphOption> {
    Box::new(GraphOptionFunc {
        apply_func: move |c: &mut Config| {
            if h > 0 {
                c.height = h;
            } else {
                c.height = 0;
            }
        },
    })
}

pub fn offset(o: i32) -> Box<dyn GraphOption> {
    Box::new(GraphOptionFunc {
        apply_func: move |c: &mut Config| c.offset = o,
    })
}

pub fn precision(p: u32) -> Box<dyn GraphOption> {
    Box::new(GraphOptionFunc {
        apply_func: move |c: &mut Config| c.precision = p,
    })
}

pub fn caption(caption: String) -> Box<dyn GraphOption> {
    Box::new(GraphOptionFunc {
        apply_func: move |c: &mut Config| c.caption = caption.trim().to_string(),
    })
}

pub fn caption_color(ac: color::AnsiColor) -> Box<dyn GraphOption> {
    Box::new(GraphOptionFunc {
        apply_func: move |c: &mut Config| c.caption_color = ac,
    })
}

pub fn axis_color(ac: color::AnsiColor) -> Box<dyn GraphOption> {
    Box::new(GraphOptionFunc {
        apply_func: move |c: &mut Config| c.axis_color = ac,
    })
}

pub fn label_color(ac: color::AnsiColor) -> Box<dyn GraphOption> {
    Box::new(GraphOptionFunc {
        apply_func: move |c: &mut Config| c.label_color = ac,
    })
}

pub fn series_colors(ac: Vec<color::AnsiColor>) -> Box<dyn GraphOption> {
    Box::new(GraphOptionFunc {
        apply_func: move |c: &mut Config| c.series_colors = ac.clone(),
    })
}
