use serde_derive::Deserialize;
use serde::de;
use std::fmt;
use toml::{map::Map, Value};
use crate::cfg::*;
use crate::color::Color;
use crate::tesselation::Frame;
use rand::rngs::ThreadRng;
use std::convert::TryInto;
use std::collections::HashMap;

#[derive(Deserialize, Default)]
pub struct MetaConfig {
    global: Option<ConfigGlobal>,
    colors: Option<ConfigColors>,
}

#[derive(Deserialize, Default)]
struct ConfigGlobal {
    deviation: Option<usize>,
    weight: Option<usize>,
    size: Option<f64>,
    width: Option<usize>,
    height: Option<usize>,
    colors: Option<Map<String, Value>>,
}

#[derive(Deserialize, Default, Debug)]
struct ConfigColors {
    #[serde(flatten)]
    list: Map<String, Value>,
}


impl MetaConfig {
    pub fn from_string(src: String) -> Self {
        toml::from_str(src.as_str()).unwrap_or_else(|e| {
            println!("No valid config file found, picking default settings");
            println!("Message: {}", e);
            MetaConfig::default()
        })
    }

    pub fn pick_cfg(self, rng: &mut ThreadRng) -> SceneCfg {
        let (deviation, weight, size, width, height) = {
            let (deviation, weight, size, width, height);
            match self.global {
                None => {
                    println!("Default global");
                    deviation = DEVIATION;
                    weight = WEIGHT;
                    size = SIZE;
                    width = WIDTH;
                    height = HEIGHT;
                },
                Some(g) => {
                    match g.deviation {
                        None => {
                            println!("Default global.deviation");
                            deviation = DEVIATION;
                        },
                        Some(d) => deviation = d.try_into().unwrap_or_else(|_| {
                            println!("Unreadable global.deviation");
                            DEVIATION
                        }),
                    }
                    match g.weight {
                        None => {
                            println!("Default global.weight");
                            weight = WEIGHT;
                        },
                        Some(w) => weight = w.try_into().unwrap_or_else(|_| {
                            println!("Unreadable global.weight");
                            WEIGHT
                        }),
                    }
                    match g.size {
                        None => {
                            println!("Default global.size");
                            size = SIZE;
                        },
                        Some(s) => size = s.try_into().unwrap_or_else(|_| {
                            println!("Unreadable global.size");
                            SIZE
                        }),
                    }
                    match g.width {
                        None => {
                            println!("Default global.width");
                            width = WIDTH;
                        },
                        Some(w) => width = w.try_into().unwrap_or_else(|_| {
                            println!("Unreadable global.width");
                            WIDTH
                        }),
                    }
                    match g.height {
                        None => {
                            println!("Default global.height");
                            height = HEIGHT;
                        },
                        Some(s) => height = s.try_into().unwrap_or_else(|_| {
                            println!("Unreadable global.height");
                            HEIGHT
                        }),
                    }
                },
            }
            (deviation, weight, size, width, height)
        };

        println!("{:?}", self.colors);
        let mut colors = HashMap::new();
        if let Some(ConfigColors { list }) = self.colors {
            for name in list.keys() {
                match color_from_value(&list[name]) {
                    Ok(c) => {
                        colors.insert(name.clone(), c);
                        ()
                    },
                    Err(s) => println!("{}", s),
                }
            }
        }


        SceneCfg {
            deviation,
            weight,
            themes: vec![Color(50, 50, 50), Color(100, 0, 0), Color(0, 100, 0)],
            frame: Frame {
                x: 0,
                y: 0,
                w: width,
                h: height,
            },
            tiling: Tiling::SquaresAndTriangles,
            pattern: Pattern::FreeSpirals,
            nb_concentric_circles: 5,
            nb_free_circles: 10,
            nb_free_spirals: 3,
            nb_free_stripes: 10,
            nb_free_triangles: 10,
            nb_crossed_stripes: 7,
            nb_parallel_stripes: 15,
            var_parallel_stripes: 10,
            delaunay_count: 1000,
            tiling_size: 10.,
            stripe_width: 0.1,
            spiral_width: 0.3,
        }
    }
}

fn color_from_value(v: &Value) -> Result<Color, String> {
    match &v {
        Value::String(s) => {
            if &s[0..1] == "#" && s.len() == 7 {
                let r = i32::from_str_radix(&s[1..3], 16);
                let g = i32::from_str_radix(&s[3..5], 16);
                let b = i32::from_str_radix(&s[5..7], 16);
                match (r, g, b) {
                    (Ok(r), Ok(g), Ok(b)) => Ok(Color(r as i32, g as i32, b as i32)),
                    _ => Err(format!("{:?} is not a valid color format.\nUse [0, 0, 255] or \"#0000FF\"", s)),
                }
            } else {
                Err(format!("{:?} is not a valid color format.\nUse [0, 0, 255] or \"#0000FF\"", s))
            }
        },
        Value::Array(a) => {
            if a.len() != 3 {
                return Err(format!("{:?} is not a valid color format.\nUse [0, 0, 255] or \"#0000FF\"", v))
            }
            match &a[0..3] {
                [Value::Integer(r), Value::Integer(g), Value::Integer(b)] => Ok(Color(*r as i32, *g as i32, *b as i32)),
                _ => Err(format!("{:?} is not a valid color format.\nUse [0, 0, 255] or \"#0000FF\"", a))
fn theme_item_from_value(
    v: &Value,
    dict: &HashMap<String, Color>,
) -> Result<(Color, usize), String> {
    match v {
        Value::String(s) => {
            match color_from_value(&v, dict) {
                Ok(c) => Ok((c, 1)),
                Err(e) => Err(format!("{} or provide a named color.", e)),
            }
        },
        Value::Array(a) => {
            if a.len() == 2 {
                match &a[0..2] {
                    [v, Value::Integer(w)] if *w >= 0 => {
                        match color_from_value(&a[0], dict) {
                            Ok(c) => Ok((c, *w as usize)),
                            Err(e) => Err(format!("{} or provide a named color.", e)),
                        }
                    },
                    _ => Err(format!("{} is not a valid theme item.
Provide one of:
    - a named color (\"blue\")
    - a hex code (\"#0000FF\")
    - an rgb tuple ([0, 0, 255])
    - or any of the above along with an integer ponderation ([<COLOR>, 100])", &v)),
                }
            } else {
                match color_from_value(&v, dict) {
                    Ok(c) => Ok((c, 1)),
                    Err(e) => Err(format!("{} or provide a named color.", e)),
                }
            }
        }
        _ => Err(format!("{:?} is not a valid color format.\nUse [0, 0, 255] or \"#0000FF\" or provide a named color", v)),
    }
}

fn theme_from_value(v: &Value, dict: &HashMap<String, Color>) -> Result<Theme, String> {
    if let Ok(i) = theme_item_from_value(v, dict) {
        return Ok(Theme::new(vec![i]));
    }
    match v {
        Value::Array(a) => {
            let mut v = Vec::new();
            for x in a {
                match theme_item_from_value(x, dict) {
                    Ok(i) => v.push(i),
                    Err(e) => println!("{}", e),
                }
            }
            Ok(Theme::new(v))
        }
        _ => Err(format!("{:?} is not a valid theme.
Provide a theme item or an array of theme items", v)),
    }
}

const DEVIATION: i32 = 20;
const WEIGHT: i32 = 40;
const SIZE: f64 = 15.;
const WIDTH: usize = 1000;
const HEIGHT: usize = 600;
