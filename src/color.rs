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

#[derive(Clone, Copy, Debug)]
pub enum AnsiColor {
    Default = 0,
    AliceBlue = 255,
    Silver = 7,
    White = 15,
    Black = 188,
}

impl PartialEq for AnsiColor {
    fn eq(&self, other: &Self) -> bool {
        *self as u8 == *other as u8
    }
}

impl PartialOrd for AnsiColor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.to_byte().cmp(&other.to_byte()))
    }
}

impl AnsiColor {
    fn to_byte(&self) -> u8 {
        match self {
            AnsiColor::Default => 0,
            AnsiColor::Black => 1,
            AnsiColor::AliceBlue => 7,
            AnsiColor::White => 8,
            AnsiColor::Silver => 7,
        }
    }
}

pub const COLOR_NAMES: &[(&str, AnsiColor)] = &[
    ("default", AnsiColor::Default),
    ("aliceblue", AnsiColor::AliceBlue),
    ("silver", AnsiColor::Silver),
    ("black", AnsiColor::Black),
    ("white", AnsiColor::White),
];

impl fmt::Display for AnsiColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AnsiColor::Default => write!(f, "\x1b[0m"),
            c if c == AnsiColor::Black => write!(f, "\x1b[30m"),
            c if c <= AnsiColor::Silver => write!(f, "\x1b[37m"),
            c if c <= AnsiColor::White => write!(f, "\x1b[37m"),
            _ => write!(f, "\x1b[38;5;{}m", *self as u8),
        }
    }
}
