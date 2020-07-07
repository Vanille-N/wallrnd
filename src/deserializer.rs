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


const DEVIATION: i32 = 20;
const WEIGHT: i32 = 40;
const SIZE: f64 = 15.;
const WIDTH: usize = 1000;
const HEIGHT: usize = 600;
