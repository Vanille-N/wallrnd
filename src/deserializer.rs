use crate::cfg::*;
use crate::color::{Chooser, Color};
use crate::tesselation::Frame;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::convert::TryInto;
use toml::{map::Map, Value, value::Datetime};

#[derive(Deserialize, Default)]
pub struct MetaConfig {
    global: Option<ConfigGlobal>,
    colors: Option<ConfigColors>,
    themes: Option<ConfigThemes>,
    shapes: Option<ConfigShapes>,
    data: Option<ConfigData>,
    entry: Option<Vec<ConfigEntry>>,
}

#[derive(Deserialize, Default)]
struct ConfigGlobal {
    deviation: Option<usize>,
    weight: Option<usize>,
    size: Option<f64>,
    width: Option<usize>,
    height: Option<usize>,
}

#[derive(Deserialize, Default, Debug)]
struct ConfigColors {
    #[serde(flatten)]
    list: Map<String, Value>,
}

#[derive(Deserialize, Default, Debug)]
struct ConfigThemes {
    #[serde(flatten)]
    list: Map<String, Value>,
}

#[derive(Deserialize, Default, Debug)]
struct ConfigShapes {
    #[serde(flatten)]
    list: Map<String, Value>,
}

#[derive(Deserialize, Default, Debug)]
struct ConfigData {
    patterns: Option<ConfigPatterns>,
    tilings: Option<ConfigTilings>,
}

#[derive(Deserialize, Default, Debug)]
struct ConfigTilings {
    size_hex: Option<f64>,
    size_tri: Option<f64>,
    size_hex_and_tri: Option<f64>,
    size_squ_and_tri: Option<f64>,
    nb_del: Option<usize>,
}

#[derive(Deserialize, Default, Debug)]
struct ConfigPatterns {
    nb_f_cir: Option<usize>,
    nb_f_spi: Option<usize>,
    nb_f_str: Option<usize>,
    nb_c_str: Option<usize>,
    nb_p_str: Option<usize>,
    nb_c_cir: Option<usize>,
    nb_f_tri: Option<usize>,
    var_p_str: Option<usize>,
    var_c_str: Option<usize>,
    width_spi: Option<f64>,
    width_str: Option<f64>,
}

#[derive(Deserialize, Debug)]
struct ConfigEntry {
    start: Option<String>,
    end: Option<String>,
    weight: Option<usize>,
    themes: Vec<String>,
    shapes: Vec<String>,
}

impl MetaConfig {
    pub fn from_string(src: String) -> Self {
        toml::from_str(src.as_str()).unwrap_or_else(|e| {
            println!("No valid config file found, picking default settings");
            println!("Message: {}", e);
            MetaConfig::default()
        })
    }

    pub fn pick_cfg(self, rng: &mut ThreadRng, time: usize) -> SceneCfg {
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
                }
                Some(g) => {
                    match g.deviation {
                        None => {
                            println!("Default global.deviation");
                            deviation = DEVIATION;
                        }
                        Some(d) => {
                            deviation = d.try_into().unwrap_or_else(|_| {
                                println!("Unreadable global.deviation");
                                DEVIATION
                            })
                        }
                    }
                    match g.weight {
                        None => {
                            println!("Default global.weight");
                            weight = WEIGHT;
                        }
                        Some(w) => {
                            weight = w.try_into().unwrap_or_else(|_| {
                                println!("Unreadable global.weight");
                                WEIGHT
                            })
                        }
                    }
                    match g.size {
                        None => {
                            println!("Default global.size");
                            size = SIZE;
                        }
                        Some(s) => {
                            size = s.try_into().unwrap_or_else(|_| {
                                println!("Unreadable global.size");
                                SIZE
                            })
                        }
                    }
                    match g.width {
                        None => {
                            println!("Default global.width");
                            width = WIDTH;
                        }
                        Some(w) => {
                            width = w.try_into().unwrap_or_else(|_| {
                                println!("Unreadable global.width");
                                WIDTH
                            })
                        }
                    }
                    match g.height {
                        None => {
                            println!("Default global.height");
                            height = HEIGHT;
                        }
                        Some(s) => {
                            height = s.try_into().unwrap_or_else(|_| {
                                println!("Unreadable global.height");
                                HEIGHT
                            })
                        }
                    }
                }
            }
            (deviation, weight, size, width, height)
        };

        let colors = {
            let mut colors = HashMap::new();
            if let Some(ConfigColors { list }) = self.colors {
                for name in list.keys() {
                    match color_from_value(&list[name], &colors) {
                        Ok(c) => {
                            colors.insert(name.clone(), c);
                            ()
                        }
                        Err(s) => println!("{}", s),
                    }
                }
            }
            colors
        };

        let themes = {
            let mut themes = HashMap::new();
            if let Some(ConfigThemes { list }) = self.themes {
                for name in list.keys() {
                    match theme_from_value(&list[name], &colors, &themes) {
                        Ok(th) => {
                            themes.insert(name.clone(), th);
                            ()
                        }
                        Err(s) => println!("{}", s),
                    }
                }
            }
            themes
        };

        let shapes = {
            let mut shapes = HashMap::new();
            if let Some(ConfigShapes { list }) = self.shapes {
                for name in list.keys() {
                    shapes.insert(name.clone(), shapes_from_value(&list[name], &shapes));
                }
            }
            shapes
        };

        let (theme, shape) = choose_theme_shapes(rng, &self.entry, time);
        let (tiling, pattern) = match shapes.get(&shape) {
            None => (Tiling::choose(rng), Pattern::choose(rng)),
            Some(t) => (
                t.1.choose(rng).unwrap_or_else(|| Tiling::choose(rng)),
                t.0.choose(rng).unwrap_or_else(|| Pattern::choose(rng)),
            )
        };

        let (nb_pattern, var_stripes, width_pattern) = {
            let nb_pattern;
            let (mut var_stripes, mut width_pattern) = (0, 0.0);
            if let Some(ConfigData {
                patterns: Some(p),
                tilings: _,
            }) = &self.data
            {
                match pattern {
                    Pattern::FreeCircles => nb_pattern = p.nb_f_cir.unwrap_or(NBFCIR) as i32,
                    Pattern::FreeTriangles => nb_pattern = p.nb_f_tri.unwrap_or(NBFTRI) as i32,
                    Pattern::FreeStripes => {
                        nb_pattern = p.nb_f_str.unwrap_or(NBFSTR) as i32;
                        width_pattern = p.width_str.unwrap_or(WSTR) as f64;
                    }
                    Pattern::FreeSpirals => {
                        nb_pattern = p.nb_f_spi.unwrap_or(NBFSPI) as i32;
                        width_pattern = p.width_spi.unwrap_or(WSPI);
                    }
                    Pattern::ConcentricCircles => nb_pattern = p.nb_c_cir.unwrap_or(NBCCIR) as i32,
                    Pattern::ParallelStripes => {
                        nb_pattern = p.nb_p_str.unwrap_or(NBPSTR) as i32;
                        var_stripes = p.var_p_str.unwrap_or(VARPSTR) as i32;
                    }
                    Pattern::CrossedStripes => {
                        nb_pattern = p.nb_c_str.unwrap_or(NBCSTR) as i32;
                        var_stripes = p.var_c_str.unwrap_or(VARCSTR) as i32;
                    }
                }
            } else {
                match pattern {
                    Pattern::FreeCircles => nb_pattern = NBFCIR as i32,
                    Pattern::FreeTriangles => nb_pattern = NBFTRI as i32,
                    Pattern::FreeStripes => {
                        nb_pattern = NBFSTR as i32;
                        width_pattern = WSTR;
                    }
                    Pattern::FreeSpirals => {
                        nb_pattern = NBFSPI as i32;
                        width_pattern = WSPI;
                    }
                    Pattern::ConcentricCircles => nb_pattern = NBCCIR as i32,
                    Pattern::ParallelStripes => {
                        nb_pattern = NBPSTR as i32;
                        var_stripes = VARPSTR as i32;
                    }
                    Pattern::CrossedStripes => {
                        nb_pattern = NBCSTR as i32;
                        var_stripes = VARCSTR as i32;
                    }
                }
            }
            (nb_pattern, var_stripes, width_pattern)
        };

        let (size_tiling, nb_delaunay) = {
            if let Some(ConfigData {
                patterns: _,
                tilings: Some(t),
            }) = self.data
            {
                match tiling {
                    Tiling::Hexagons => (t.size_hex.unwrap_or(size), 0),
                    Tiling::Triangles => (t.size_tri.unwrap_or(size), 0),
                    Tiling::HexagonsAndTriangles => (t.size_hex_and_tri.unwrap_or(size), 0),
                    Tiling::SquaresAndTriangles => (t.size_squ_and_tri.unwrap_or(size), 0),
                    Tiling::Delaunay => (0.0, t.nb_del.unwrap_or(NBDEL) as i32),
                }
            } else {
                match tiling {
                    Tiling::Hexagons => (size, 0),
                    Tiling::Triangles => (size, 0),
                    Tiling::HexagonsAndTriangles => (size, 0),
                    Tiling::SquaresAndTriangles => (size, 0),
                    Tiling::Delaunay => (0.0, NBDEL as i32),
                }
            }
        };
        println!("<{}>", theme);

        SceneCfg {
            deviation,
            weight,
            theme: themes.get(&theme).unwrap_or_else(|| themes.get(themes.keys().collect::<Vec<_>>().choose(rng).unwrap().clone()).unwrap()).clone(),
            frame: Frame {
                x: 0,
                y: 0,
                w: width,
                h: height,
            },
            tiling,
            pattern,
            nb_pattern,
            var_stripes,
            nb_delaunay,
            size_tiling,
            width_pattern,
        }
    }
}

fn color_from_value(v: &Value, dict: &HashMap<String, Color>) -> Result<Color, String> {
    match v {
        Value::String(s) => {
            if let Some(c) = dict.get(s.as_str()) {
                return Ok(*c);
            }
            if &s[0..1] == "#" && s.len() == 7 {
                let r = i32::from_str_radix(&s[1..3], 16);
                let g = i32::from_str_radix(&s[3..5], 16);
                let b = i32::from_str_radix(&s[5..7], 16);
                match (r, g, b) {
                    (Ok(r), Ok(g), Ok(b)) => Ok(Color(r as i32, g as i32, b as i32)),
                    _ => Err(format!(
                        "{:?} is not a valid color format.\nUse [0, 0, 255] or \"#0000FF\"",
                        s
                    )),
                }
            } else {
                Err(format!(
                    "{:?} is not a valid color format.\nUse [0, 0, 255] or \"#0000FF\"",
                    s
                ))
            }
        }
        Value::Array(a) => {
            if a.len() != 3 {
                return Err(format!(
                    "{:?} is not a valid color format.\nUse [0, 0, 255] or \"#0000FF\"",
                    v
                ));
            }
            match &a[0..3] {
                [Value::Integer(r), Value::Integer(g), Value::Integer(b)] => {
                    Ok(Color(*r as i32, *g as i32, *b as i32))
                }
                _ => Err(format!(
                    "{:?} is not a valid color format.\nUse [0, 0, 255] or \"#0000FF\"",
                    a
                )),
            }
        }
        _ => Err(format!(
            "{:?} is not a valid color format.\nUse [0, 0, 255] or \"#0000FF\"",
            v
        )),
    }
}

fn theme_item_from_value(
    v: &Value,
    dict: &HashMap<String, Color>,
) -> Result<(Color, usize), String> {
    match v {
        Value::String(_) => {
            match color_from_value(&v, dict) {
                Ok(c) => Ok((c, 1)),
                Err(e) => Err(format!("{} or provide a named color.", e)),
            }
        },
        Value::Array(a) => {
            if a.len() == 2 {
                match &a[0..2] {
                    [x, Value::Integer(w)] if *w >= 0 => {
                        match color_from_value(x, dict) {
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

fn theme_from_value(
    v: &Value,
    colors: &HashMap<String, Color>,
    themes: &HashMap<String, Chooser<Color>>,
) -> Result<Chooser<Color>, String> {
    if let Ok(i) = theme_item_from_value(v, colors) {
        return Ok(Chooser::new(vec![i]));
    }
    let mut items = Vec::new();
    if let Value::String(s) = v {
        if let Some(th) = themes.get(s) {
            items = th.extract();
        }
    }
    match v {
        Value::Array(a) => {
            for x in a {
                if let Value::String(s) = x {
                    if let Some(th) = themes.get(s) {
                        items.append(&mut th.extract());
                        continue;
                    }
                }
                match theme_item_from_value(x, colors) {
                    Ok(i) => items.push(i),
                    Err(e) => println!("{}", e),
                }
            }
            Ok(Chooser::new(items))
        }
        _ => Err(format!(
            "{:?} is not a valid theme.
Provide a theme item or an array of theme items",
            v
        )),
    }
}

fn shapes_from_value(
    v: &Value,
    shapes: &HashMap<String, (Chooser<Pattern>, Chooser<Tiling>)>,
) -> (Chooser<Pattern>, Chooser<Tiling>) {
    let mut tilings = Chooser::new(vec![]);
    let mut patterns = Chooser::new(vec![]);
    match v {
        Value::Array(a) => {
            for x in a {
                match x {
                    Value::String(s) => {
                        if let Some(sh) = shapes.get(s) {
                            let (p, t) = sh;
                            tilings.append(t.extract());
                            patterns.append(p.extract());
                        } else {
                            add_shape(&s[..], 1, &mut tilings, &mut patterns);
                        }
                    }
                    Value::Array(a) => {
                        if a.len() == 2 {
                            match &a[..] {
                                [Value::String(s), Value::Integer(w)] if *w > 0 => {
                                    add_shape(&s[..], *w as usize, &mut tilings, &mut patterns)
                                }
                                _ => println!("{} is not a valid shape.", x),
                            }
                        } else {
                            println!("{} is not a valid shape.", x);
                        }
                    }
                    _ => println!("{} is not a valid shape.", x),
                }
            }
        }
        _ => println!("{} is not an array of shapes.", v),
    }
    (patterns, tilings)
}

fn add_shape(s: &str, w: usize, tilings: &mut Chooser<Tiling>, patterns: &mut Chooser<Pattern>) {
    match &s[..] {
        "H" | "hex." | "hexagons" => tilings.push(Tiling::Hexagons, w),
        "T" | "tri." | "triangles" => tilings.push(Tiling::Triangles, w),
        "H&T" | "hex.&tri." | "hexagons&squares" => tilings.push(Tiling::HexagonsAndTriangles, w),
        "S&T" | "squ.&tri." | "squares&triangles" => tilings.push(Tiling::SquaresAndTriangles, w),
        "D" | "del." | "delaunay" => tilings.push(Tiling::Delaunay, w),
        "FC" | "f-cir." | "free-circles" => patterns.push(Pattern::FreeCircles, w),
        "FT" | "f-tri." | "free-triangles" => patterns.push(Pattern::FreeTriangles, w),
        "FR" | "f-str." | "free-stripes" => patterns.push(Pattern::FreeStripes, w),
        "FP" | "f-spi." | "free-spirals" => patterns.push(Pattern::FreeSpirals, w),
        "CC" | "c-cir." | "concentric-circles" => patterns.push(Pattern::ConcentricCircles, w),
        "PS" | "p-str." | "parallel-stripes" => patterns.push(Pattern::ParallelStripes, w),
        "CS" | "c-str." | "crossed-stripes" => patterns.push(Pattern::CrossedStripes, w),
        _ => println!("{} is not recognized as a shape", s),
    }
}

fn choose_theme_shapes(rng: &mut ThreadRng, entry: &Option<Vec<ConfigEntry>>, time: usize) -> (String, String) {
    match entry {
        None => (String::from(""), String::from("")),
        Some(v) => {
            let mut valid = Chooser::new(vec![]);
            for e in v {
                let start = usize_of_time(&e.start);
                let end = usize_of_time(&e.end);
                if start <= time && time <= end {
                    valid.push(e, e.weight.unwrap_or(1));
                }
            }
            match valid.choose(rng) {
                None => (String::from(""), String::from("")),
                Some(chosen_entry) => {
                    let chosen_theme = chosen_entry.themes.choose(rng).map(|s| String::from(s)).unwrap_or(String::from(""));
                    let chosen_shapes = chosen_entry.shapes.choose(rng).map(|s| String::from(s)).unwrap_or(String::from(""));
                    (chosen_theme, chosen_shapes)
                },
            }
        }
    }
}

fn usize_of_time(t: &Option<String>) -> usize {
    t.as_ref().unwrap_or(&String::from("0")).parse::<usize>().unwrap_or(0)
}



const DEVIATION: i32 = 20;
const WEIGHT: i32 = 40;
const SIZE: f64 = 15.;
const WIDTH: usize = 1000;
const HEIGHT: usize = 600;
const NBFCIR: usize = 10;
const NBFTRI: usize = 15;
const NBFSTR: usize = 7;
const NBPSTR: usize = 15;
const NBCCIR: usize = 5;
const NBCSTR: usize = 10;
const NBFSPI: usize = 3;
const VARPSTR: usize = 15;
const VARCSTR: usize = 10;
const WSPI: f64 = 0.3;
const WSTR: f64 = 0.1;
const NBDEL: usize = 1000;
