#![feature(convert_float_to_int)]
mod color;
use color::*;
mod utility;
use utility::*;
mod options;
use options::*;
use std::cmp::max;
use std::f64;
use std::f64::consts::PI;
use std::string::String;

#[derive(Debug)]
struct Cell {
    text: String,
    color: AnsiColor,
}

struct Config {
    width: i32,
    height: i32,
    offset: i32,
    caption: String,
    precision: u32,
    caption_color: AnsiColor,
    axis_color: AnsiColor,
    label_color: AnsiColor,
    series_colors: Vec<AnsiColor>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            width: 150,
            height: 10,
            offset: 3,
            caption: String::new(),
            precision: 2,
            caption_color: AnsiColor::White,
            axis_color: AnsiColor::White,
            label_color: AnsiColor::AliceBlue,
            series_colors: vec![
                AnsiColor::AliceBlue,
                AnsiColor::Silver,
                AnsiColor::White,
                AnsiColor::Black,
            ],
        }
    }
}

trait GraphOption {
    fn apply(&self, c: &mut Config);
}

struct GraphOptionFunc<F: Fn(&mut Config)> {
    apply_func: F,
}

impl<F: Fn(&mut Config)> GraphOption for GraphOptionFunc<F> {
    fn apply(&self, c: &mut Config) {
        (self.apply_func)(c);
    }
}

fn width(w: i32) -> Box<dyn GraphOption> {
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

fn configure(mut defaults: Config, options: Vec<Box<dyn GraphOption>>) -> Config {
    for o in options {
        o.apply(&mut defaults);
    }
    defaults
}

fn height(h: i32) -> Box<dyn GraphOption> {
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

fn offset(o: i32) -> Box<dyn GraphOption> {
    Box::new(GraphOptionFunc {
        apply_func: move |c: &mut Config| c.offset = o,
    })
}

fn precision(p: u32) -> Box<dyn GraphOption> {
    Box::new(GraphOptionFunc {
        apply_func: move |c: &mut Config| c.precision = p,
    })
}

fn caption(caption: String) -> Box<dyn GraphOption> {
    Box::new(GraphOptionFunc {
        apply_func: move |c: &mut Config| c.caption = caption.trim().to_string(),
    })
}

fn caption_color(ac: AnsiColor) -> Box<dyn GraphOption> {
    Box::new(GraphOptionFunc {
        apply_func: move |c: &mut Config| c.caption_color = ac,
    })
}

fn axis_color(ac: AnsiColor) -> Box<dyn GraphOption> {
    Box::new(GraphOptionFunc {
        apply_func: move |c: &mut Config| c.axis_color = ac,
    })
}

fn label_color(ac: AnsiColor) -> Box<dyn GraphOption> {
    Box::new(GraphOptionFunc {
        apply_func: move |c: &mut Config| c.label_color = ac,
    })
}

fn series_colors(ac: Vec<AnsiColor>) -> Box<dyn GraphOption> {
    Box::new(GraphOptionFunc {
        apply_func: move |c: &mut Config| c.series_colors = ac.clone(),
    })
}

fn plot(series: &[f64], options: Vec<Box<dyn GraphOption>>) -> String {
    let mut data = Vec::new();
    data.push(series.to_vec());
    plot_many(&mut data, options)
}

fn plot_many(data: &mut Vec<Vec<f64>>, options: Vec<Box<dyn GraphOption>>) -> String {
    let (mut minimum, mut maximum) = (f64::INFINITY, f64::NEG_INFINITY);
    let mut config = Config {
        offset: 3,
        precision: 2,
        ..Config::default()
    };
    config = configure(config, options);
    let mut len_max = 0;
    for i in data.iter() {
        let l: i32 = i.len() as i32;
        if l > len_max {
            len_max = l;
        }
    }
    if config.width > 0 {
        for i in 0..data.len() {
            let mut temp_vec = data[i].to_vec();
            for _j in data[i].len()..len_max as usize {
                temp_vec.push(std::f64::NAN);
            }
            data[i] = interpolate_array(&temp_vec, config.width.try_into().unwrap());
        }
        len_max = config.width;
    }
    for i in 0..data.len() {
        let (min, max) = min_max_float64_slice(&data[i]);
        if min < minimum {
            minimum = min;
        }
        if max > maximum {
            maximum = max;
        }
    }
    let interval = (maximum - minimum).abs();
    if config.height <= 0 {
        if interval <= 0.0 {
            config.height =
                (interval * 10.0f64.powi((interval.log10().ceil() as i32).abs())) as i32;
        } else {
            config.height = interval as i32;
        }
    }

    if config.offset <= 0 {
        config.offset = 3;
    }
    let mut ratio = 0.0;
    if interval != 0.0 {
        ratio = config.height as f64 / interval;
    } else {
        ratio = 1.0;
    }

    let min2 = round(minimum * ratio);
    let max2 = round(maximum * ratio);
    let intmin2 = min2 as i32;
    let intmax2 = max2 as i32;
    let rows = (intmax2 - intmin2).abs();
    let width = len_max + config.offset;
    let mut plot: Vec<Vec<Cell>> = Vec::with_capacity(rows as usize + 1);
    for _i in 0..rows + 1 {
        let mut line: Vec<Cell> = Vec::with_capacity(width as usize);
        for _j in 0..width {
            line.push(Cell {
                text: " ".to_string(),
                color: AnsiColor::Default,
            });
        }
        plot.push(line);
    }
    let mut precision = config.precision;
    let mut log_maximum = (maximum.abs().max(minimum.abs())).log10().floor();
    if minimum == 0.0 && maximum == 0.0 {
        log_maximum = -1.0;
    }
    if log_maximum < 0.0 {
        if log_maximum % 1.0 != 0.0 {
            config.precision += log_maximum.abs() as u32;
        } else {
            config.precision += log_maximum.abs() as u32 - 1;
        }
    } else if log_maximum > 2.0 {
        precision = 0;
    }

    let max_num_length = format!("{:.1$}", maximum, precision as usize).len();
    let min_num_length = format!("{:.1$}", minimum, precision as usize).len();
    let max_width = f64::max(max_num_length as f64, min_num_length as f64).floor() as i32;
    for y in intmin2..intmax2 + 1 {
        let magnitude = if rows > 0 {
            maximum - ((y - intmin2) as f64 * interval / rows as f64)
        } else {
            y as f64
        };

        let label = format!(
            "{:>width$.precision$}",
            magnitude,
            width = (max_width + 1) as usize,
            precision = precision as usize
        );
        let w = (y - intmin2) as usize;
        let h = max(config.offset as i32 - label.len() as i32, 0) as usize;

        plot[w][h].text = label;
        plot[w][h].color = config.label_color;
        plot[w][(config.offset - 1) as usize].text = "┤".to_owned();
        plot[w][(config.offset - 1) as usize].color = config.axis_color;
    }

    for i in 0..data.len() {
        let series = &data[i];
        let mut color = AnsiColor::Default;
        if i < config.series_colors.len() {
            color = config.series_colors[i];
        }
        let mut y0 = 0;
        let mut y1 = 0;
        if !series[0].is_nan() {
            y0 = (series[0] * ratio).round() as i32 - (min2 as i32);
            plot[(rows - y0) as usize][(config.offset - 1) as usize].text = "┼".to_string();
            plot[(rows - y0) as usize][(config.offset - 1) as usize].color = config.axis_color;
        }
        for x in 0..series.len() - 1 {
            let d0 = series[x];
            let d1 = series[x + 1];
            if d0.is_nan() && d1.is_nan() {
                continue;
            }
            if d1.is_nan() && !d0.is_nan() {
                y0 = (d0 * ratio).round() as i32 - intmin2;
                plot[(rows - y0) as usize][x + config.offset as usize].text = "╴".to_owned();
                plot[(rows - y0) as usize][x + config.offset as usize].color = color;
                continue;
            }
            if d0.is_nan() && !d1.is_nan() {
                y1 = (d1 * ratio).round() as i32 - intmin2;
                plot[(rows - y1) as usize][x + config.offset as usize].text = "╶".to_string();
                plot[(rows - y1) as usize][x + config.offset as usize].color = color;
                continue;
            }
            y0 = (d0 * ratio).round() as i32 - intmin2;
            y1 = (d1 * ratio).round() as i32 - intmin2;
            if y0 == y1 {
                plot[(rows - y0) as usize][x + config.offset as usize].text = "─".to_owned();
            } else {
                if y0 > y1 {
                    plot[(rows - y1) as usize][x + config.offset as usize].text = "╰".to_owned();
                    plot[(rows - y0) as usize][x + config.offset as usize].text = "╮".to_owned();
                } else {
                    plot[(rows - y1) as usize][x + config.offset as usize].text = "╭".to_owned();
                    plot[(rows - y0) as usize][x + config.offset as usize].text = "╯".to_owned();
                }
                let start = f64::min(y0 as f64, y1 as f64) as i32 + 1;
                let end = f64::max(y0 as f64, y1 as f64) as i32;
                for y in start..end {
                    plot[(rows - y) as usize][x + config.offset as usize].text = "│".to_owned();
                }
            }
            let start = f64::min(y0 as f64, y1 as f64) as i32;
            let end = f64::max(y0 as f64, y1 as f64) as i32;
            for y in start..end {
                plot[(rows - y) as usize][x + config.offset as usize].color = color;
            }
        }
    }

    let mut lines = String::new();
    for (h, horizontal) in plot.iter().enumerate() {
        if h != 0 {
            lines.push('\n');
        }
        let mut last_char_index = 0;
        for i in (0..width).rev() {
            if horizontal[i as usize].text != " " {
                last_char_index = i;
                break;
            }
        }
        let mut c = AnsiColor::Default;
        for v in &horizontal[..(last_char_index as usize) + 1] {
            if v.color != c {
                c = v.color;
                lines.push_str(&c.to_string());
            }
            lines.push_str(&v.text);
        }
        if c != AnsiColor::Default {
            lines.push_str(&AnsiColor::Default.to_string());
        }
    }
    if !config.caption.is_empty() {
        lines.push('\n');
        lines.push_str(&" ".repeat((config.offset + max_width).try_into().unwrap()));
        if config.caption.len() < len_max.try_into().unwrap() {
            lines.push_str(
                &" ".repeat(
                    ((len_max - config.caption.len() as i32) / 2)
                        .try_into()
                        .unwrap(),
                ),
            );
        }
        if config.caption_color != AnsiColor::Default {
            lines.push_str(&config.caption_color.to_string());
        }
        lines.push_str(&config.caption);
        if config.caption_color != AnsiColor::Default {
            lines.push_str(&AnsiColor::Default.to_string());
        }
    }

    lines
}
fn main() {
    let mut data = Vec::new();
    for i in 0..105 {
        data.push(15.0 * (i as f64 * (4.0 * PI / 120.0)).sin());
    }
    let mut height_new: Vec<Box<dyn GraphOption>> = Vec::new();
    height_new.push(height(10));
    let graph = plot(&data, height_new);
    println!("{}", graph);
    // println!("Hello, world!");
}
