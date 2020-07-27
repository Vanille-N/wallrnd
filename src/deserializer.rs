use crate::cfg::SceneCfg;
use crate::prelude::*;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::convert::TryInto;
use toml::{map::Map, Value};

/// All config information
#[derive(Deserialize, Default)]
pub struct MetaConfig {
    pub global: Option<ConfigGlobal>,
    pub lines: Option<ConfigLines>,
    pub colors: Option<ConfigColors>,
    pub themes: Option<ConfigThemes>,
    pub shapes: Option<ConfigShapes>,
    pub data: Option<ConfigData>,
    pub entry: Option<Vec<ConfigEntry>>,
}

/// Global options
#[derive(Deserialize, Default)]
pub struct ConfigGlobal {
    pub deviation: Option<usize>,
    pub weight: Option<usize>,
    pub size: Option<f64>,
    pub width: Option<usize>,
    pub height: Option<usize>,
}

/// Lines appearance
#[derive(Deserialize, Default)]
pub struct ConfigLines {
    pub width: Option<f64>,
    pub color: Option<String>,
    pub del_width: Option<f64>,
    pub del_color: Option<String>,
    pub hex_width: Option<f64>,
    pub hex_color: Option<String>,
    pub tri_width: Option<f64>,
    pub tri_color: Option<String>,
    pub rho_width: Option<f64>,
    pub rho_color: Option<String>,
    pub hex_and_tri_width: Option<f64>,
    pub hex_and_tri_color: Option<String>,
    pub squ_and_tri_width: Option<f64>,
    pub squ_and_tri_color: Option<String>,
}

/// Color list
#[derive(Deserialize, Default, Debug)]
pub struct ConfigColors {
    #[serde(flatten)]
    pub list: Map<String, Value>,
}

/// Theme list
#[derive(Deserialize, Default, Debug)]
pub struct ConfigThemes {
    #[serde(flatten)]
    pub list: Map<String, Value>,
}

/// Shapes combination list
#[derive(Deserialize, Default, Debug)]
pub struct ConfigShapes {
    #[serde(flatten)]
    pub list: Map<String, Value>,
}

/// Group together pattern options and tiling options
#[derive(Deserialize, Default, Debug)]
pub struct ConfigData {
    pub patterns: Option<ConfigPatterns>,
    pub tilings: Option<ConfigTilings>,
}

/// Tiling options
#[derive(Deserialize, Default, Debug)]
pub struct ConfigTilings {
    pub size_hex: Option<f64>,
    pub size_tri: Option<f64>,
    pub size_hex_and_tri: Option<f64>,
    pub size_squ_and_tri: Option<f64>,
    pub size_rho: Option<f64>,
    pub nb_delaunay: Option<usize>,
}

/// Pattern options
#[derive(Deserialize, Default, Debug)]
pub struct ConfigPatterns {
    pub nb_free_circles: Option<usize>,
    pub nb_free_spirals: Option<usize>,
    pub nb_free_stripes: Option<usize>,
    pub nb_crossed_stripes: Option<usize>,
    pub nb_parallel_stripes: Option<usize>,
    pub nb_concentric_circles: Option<usize>,
    pub nb_free_triangles: Option<usize>,
    pub nb_parallel_waves: Option<usize>,
    pub nb_parallel_sawteeth: Option<usize>,
    pub var_parallel_stripes: Option<usize>,
    pub var_crossed_stripes: Option<usize>,
    pub width_spiral: Option<f64>,
    pub width_stripe: Option<f64>,
    pub width_wave: Option<f64>,
    pub width_sawtooth: Option<f64>,
}

/// Entry for a single theme/time combination
#[derive(Deserialize, Debug)]
pub struct ConfigEntry {
    pub span: Option<String>,
    pub weight: Option<usize>,
    pub themes: Option<Vec<String>>,
    pub shapes: Option<Vec<String>>,
    pub line_color: Option<String>,
}

impl MetaConfig {
    /// Parse from TOML.
    /// Heavy lifting done by external crates
    pub fn from_string(src: String, verbose: Verbosity) -> Self {
        toml::from_str(src.as_str()).unwrap_or_else(|e| {
            if verbose.warn {
                println!("No valid config file found, picking default settings");
                println!("Message: {}", e);
            }
            MetaConfig::default()
        })
    }

    /// Choose options at random according to configuration
    pub fn pick_cfg(self, rng: &mut ThreadRng, time: usize, verbose: Verbosity) -> SceneCfg {
        // Read default/overriden global options
        let (deviation, weight, size, width, height) = {
            let (deviation, weight, size, width, height);
            match self.global {
                None => {
                    if verbose.info {
                        println!("Default global");
                    }
                    deviation = DEVIATION;
                    weight = WEIGHT;
                    size = SIZE;
                    width = WIDTH;
                    height = HEIGHT;
                }
                Some(g) => {
                    match g.deviation {
                        None => {
                            if verbose.info {
                                println!("Default global.deviation");
                            }
                            deviation = DEVIATION;
                        }
                        Some(d) => {
                            deviation = d.try_into().unwrap_or_else(|_| {
                                if verbose.warn {
                                    println!("Unreadable global.deviation");
                                }
                                DEVIATION
                            })
                        }
                    }
                    match g.weight {
                        None => {
                            if verbose.info {
                                println!("Default global.weight");
                            }
                            weight = WEIGHT;
                        }
                        Some(w) => {
                            weight = w.try_into().unwrap_or_else(|_| {
                                if verbose.warn {
                                    println!("Unreadable global.weight");
                                }
                                WEIGHT
                            })
                        }
                    }
                    match g.size {
                        None => {
                            if verbose.info {
                                println!("Default global.size");
                            }
                            size = SIZE;
                        }
                        Some(s) => {
                            size = s;
                        }
                    }
                    match g.width {
                        None => {
                            if verbose.info {
                                println!("Default global.width");
                            }
                            width = WIDTH;
                        }
                        Some(w) => {
                            width = w;
                        }
                    }
                    match g.height {
                        None => {
                            if verbose.info {
                                println!("Default global.height");
                            }
                            height = HEIGHT;
                        }
                        Some(s) => {
                            height = s;
                        }
                    }
                }
            }
            if verbose.details {
                println!("Global settings:
    Deviation   (color)    {}
    Weight      (color)    {}
    Size        (tiles)    {}
    Width       (image)    {}
    Height      (image)    {}", deviation, weight, size, width, height);
            }
            (deviation, weight, size, width, height)
        };

        // Get list of named colors
        let colors = {
            let mut colors = HashMap::new();
            if let Some(ConfigColors { list }) = self.colors {
                for name in list.keys() {
                    match color_from_value(&list[name], &colors) {
                        Ok(c) => {
                            if verbose.details {
                                println!("Added new color to list: '{} = {}'", &name, &c);
                            }
                            colors.insert(name.clone(), c);
                        }
                        Err(s) => {
                            if verbose.warn {
                                println!("{}", s);
                            }
                        }
                    }
                }
            }
            colors
        };

        // Get list of named themes
        let mut themes = {
            let mut themes = HashMap::new();
            if let Some(ConfigThemes { list }) = self.themes {
                for name in list.keys() {
                    match theme_from_value(&list[name], &colors, &themes, verbose) {
                        Ok(th) => {
                            if verbose.details {
                                println!("Added new theme to list: '{}'", &name);
                            }
                            themes.insert(name.clone(), th);
                        }
                        Err(s) => {
                            if verbose.warn {
                                println!("{}", s);
                            }
                        }
                    }
                }
            }
            themes
        };

        // List of allowed shape combinations
        let shapes = {
            let mut shapes = HashMap::new();
            if let Some(ConfigShapes { list }) = self.shapes {
                for name in list.keys() {
                    if verbose.details {
                        println!("Added new shapes to list: '{}'", &name);
                    }
                    shapes.insert(name.clone(), shapes_from_value(&list[name], &shapes));
                }
            }
            shapes
        };

        let (theme, shape, line_color_override) = choose_theme_shapes(rng, &self.entry, time);
        if verbose.info {
            println!("Chosen theme: '{}'", &theme);
        }

        let (tiling, pattern) = match shapes.get(&shape) {
            None => (Tiling::choose(rng), Pattern::choose(rng)),
            Some(t) => (
                t.1.choose(rng).unwrap_or_else(|| Tiling::choose(rng)),
                t.0.choose(rng).unwrap_or_else(|| Pattern::choose(rng)),
            ),
        };
        if verbose.info {
            println!("Pattern '{:?}' and tiling '{:?}' chosen from shapes '{}'", pattern, tiling, shape);
        }

        // Get pattern-specific information according to picked shapes
        let (nb_pattern, var_stripes, width_pattern) = {
            let nb_pattern;
            let (mut var_stripes, mut width_pattern) = (0, 0.0);
            if let Some(ConfigData {
                patterns: Some(p),
                tilings: _,
            }) = &self.data
            {
                match pattern {
                    Pattern::FreeCircles => {
                        nb_pattern = p.nb_free_circles.unwrap_or(NB_FREE_CIRCLES) as i32
                    }
                    Pattern::FreeTriangles => {
                        nb_pattern = p.nb_free_triangles.unwrap_or(NB_FREE_TRIANGLES) as i32
                    }
                    Pattern::FreeStripes => {
                        nb_pattern = p.nb_free_stripes.unwrap_or(NB_FREE_STRIPES) as i32;
                        width_pattern = p.width_stripe.unwrap_or(WIDTH_STRIPE) as f64;
                    }
                    Pattern::FreeSpirals => {
                        nb_pattern = p.nb_free_spirals.unwrap_or(NB_FREE_SPIRALS) as i32;
                        width_pattern = p.width_spiral.unwrap_or(WIDTH_SPIRAL);
                    }
                    Pattern::ConcentricCircles => {
                        nb_pattern = p.nb_concentric_circles.unwrap_or(NB_CONCENTRIC_CIRCLES) as i32
                    }
                    Pattern::ParallelStripes => {
                        nb_pattern = p.nb_parallel_stripes.unwrap_or(NB_PARALLEL_STRIPES) as i32;
                        var_stripes = p.var_parallel_stripes.unwrap_or(VAR_PARALLEL_STRIPES) as i32;
                    }
                    Pattern::CrossedStripes => {
                        nb_pattern = p.nb_crossed_stripes.unwrap_or(NB_CROSSED_STRIPES) as i32;
                        var_stripes = p.var_crossed_stripes.unwrap_or(VAR_CROSSED_STRIPES) as i32;
                    }
                    Pattern::ParallelWaves => {
                        nb_pattern = p.nb_parallel_waves.unwrap_or(NB_PARALLEL_WAVES) as i32;
                        width_pattern = p.width_wave.unwrap_or(WIDTH_WAVE);
                    }
                    Pattern::ParallelSawteeth => {
                        nb_pattern = p.nb_parallel_sawteeth.unwrap_or(NB_PARALLEL_SAWTEETH) as i32;
                        width_pattern = p.width_sawtooth.unwrap_or(WIDTH_SAWTOOTH);
                    }
                }
            } else {
                match pattern {
                    Pattern::FreeCircles => nb_pattern = NB_FREE_CIRCLES as i32,
                    Pattern::FreeTriangles => nb_pattern = NB_FREE_TRIANGLES as i32,
                    Pattern::FreeStripes => {
                        nb_pattern = NB_FREE_STRIPES as i32;
                        width_pattern = WIDTH_STRIPE;
                    }
                    Pattern::FreeSpirals => {
                        nb_pattern = NB_FREE_SPIRALS as i32;
                        width_pattern = WIDTH_SPIRAL;
                    }
                    Pattern::ConcentricCircles => nb_pattern = NB_CONCENTRIC_CIRCLES as i32,
                    Pattern::ParallelStripes => {
                        nb_pattern = NB_PARALLEL_STRIPES as i32;
                        var_stripes = VAR_PARALLEL_STRIPES as i32;
                    }
                    Pattern::CrossedStripes => {
                        nb_pattern = NB_CROSSED_STRIPES as i32;
                        var_stripes = VAR_CROSSED_STRIPES as i32;
                    }
                    Pattern::ParallelWaves => {
                        nb_pattern = NB_PARALLEL_WAVES as i32;
                        width_pattern = WIDTH_WAVE;
                    }
                    Pattern::ParallelSawteeth => {
                        nb_pattern = NB_PARALLEL_SAWTEETH as i32;
                        width_pattern = WIDTH_SAWTOOTH;
                    }
                }
            }
            if verbose.details {
                println!("Number of patterns: {}
Variability of stripes orientation: {}
Width of pattern: {}", nb_pattern, var_stripes, width_pattern);
            }
            (nb_pattern, var_stripes, width_pattern)
        };

        if themes.is_empty() {
            if verbose.warn {
                println!("No themes available. Populating with random theme");
            }
            if colors.is_empty() {
                if verbose.warn {
                    println!("No colors available. Populating with random color");
                }
                themes.insert(
                    String::from("-default-"),
                    Chooser::new(vec![((Color::random(rng), -1, -1), 1)]),
                );
            } else {
                themes.insert(
                    String::from("-default-"),
                    Chooser::new(vec![(
                        (
                            *colors
                                .get(*colors.keys().collect::<Vec<_>>().choose(rng).unwrap())
                                .unwrap(),
                            -1,
                            -1,
                        ),
                        1,
                    )]),
                );
            }
        }

        // Get tiling-specific options according to picked shapes
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
                    Tiling::Rhombus => (t.size_rho.unwrap_or(size), 0),
                    Tiling::Delaunay => (0.0, t.nb_delaunay.unwrap_or(NB_DELAUNAY) as i32),
                }
            } else {
                match tiling {
                    Tiling::Hexagons => (size, 0),
                    Tiling::Triangles => (size, 0),
                    Tiling::HexagonsAndTriangles => (size, 0),
                    Tiling::SquaresAndTriangles => (size, 0),
                    Tiling::Rhombus => (size, 0),
                    Tiling::Delaunay => (0.0, NB_DELAUNAY as i32),
                }
            }
        };
        if verbose.details {
            println!("Tiling size: {}
Delaunay triangles count: {}", size_tiling, nb_delaunay);
}
        let (line_width, line_color_default) = {
            if let Some(lines) = self.lines {
                lines.get_settings(tiling, &colors)
            } else {
                (LINE_WIDTH, LINE_COLOR)
            }
        };
        if verbose.details {
            println!("Line width: {}
Line color: {}", line_width, line_color_default);
}

        SceneCfg {
            deviation,
            weight,
            theme: themes
                .get(&theme)
                .unwrap_or_else(|| {
                    themes
                        .get(*themes.keys().collect::<Vec<_>>().choose(rng).unwrap())
                        .unwrap()
                })
                .clone(),
            frame: Frame {
                x: 0,
                y: 0,
                w: width,
                h: height,
            },
            tiling,
            line_width,
            line_color: color_from_value(&Value::String(line_color_override), &colors)
                .unwrap_or_else(|_| {
                    color_from_value(&Value::String(line_color_default.to_string()), &colors)
                        .unwrap_or(Color(0, 0, 0))
                }),
            pattern,
            nb_pattern,
            var_stripes,
            nb_delaunay,
            size_tiling,
            width_pattern,
        }
    }
}

/// Parse a color code: decimal (0-255) or hex (00-FF)
fn color_from_value(val: &Value, dict: &HashMap<String, Color>) -> Result<Color, String> {
    match val {
        Value::String(s) => {
            if let Some(color) = dict.get(s.as_str()) {
                return Ok(*color);
            }
            if s.len() == 7 && &s[0..1] == "#" {
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
                    val
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
            val
        )),
    }
}

fn theme_item_from_value(val: &Value, dict: &HashMap<String, Color>, verbose: Verbosity) -> (Color, usize, isize, isize) {
    let warn_invalid = |x| {
        if verbose.warn {
            println!(
                "Invalid item ({:?})
Provide one of:
- a named color (\"blue\")
- a hex code (\"#0000FF\")
- any of the above along with an integer ponderation (\"<COLOR> xPONDERATION\")
- any of the above along with a variability override (\"<COLOR> ~VAR\")
- any of the above along with a weight override (\"<COLOR> !WEIGHT\")
Note that the format [<R>, <G>, <B>] is not accepted here",
                x
            );
        }
    };
    match val {
        Value::String(s) => {
            let mut c = Color(0, 0, 0);
            let mut p = 1;
            let mut var = -1;
            let mut w = -1;
            for item in s.split(' ') {
                if item.is_empty() {
                    continue;
                }
                if &item[0..1] == "x" {
                    p = item[1..].parse().unwrap_or_else(|_| {
                        println!("Not a valid ponderation: {}", &item[1..]);
                        1
                    });
                } else if &item[0..1] == "~" {
                    var = item[1..]
                        .parse::<usize>()
                        .map(|x| x as isize)
                        .unwrap_or_else(|_| {
                            println!("Not a valid variability: {}", &item[1..]);
                            -1
                        });
                } else if &item[0..1] == "!" {
                    w = item[1..]
                        .parse::<usize>()
                        .map(|x| x as isize)
                        .unwrap_or_else(|_| {
                            println!("Not a valid weight: {}", &item[1..]);
                            -1
                        });
                } else {
                    match color_from_value(&Value::String(item.to_string()), dict) {
                        Ok(color) => c = color,
                        Err(e) => {
                            warn_invalid(e);
                        }
                    }
                }
            }
            (c, p, var, w)
        }
        val => {
            warn_invalid(val.to_string());
            (Color(0, 0, 0), 1, -1, -1)
        }
    }
}

/// Read group of colors as a theme
fn theme_from_value(
    v: &Value,
    colors: &HashMap<String, Color>,
    themes: &HashMap<String, Chooser<(Color, isize, isize)>>,
    verbose: Verbosity,
) -> Result<Chooser<(Color, isize, isize)>, String> {
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
                let (color, ponderation, var, weight) = theme_item_from_value(x, colors, verbose);
                items.push(((color, var, weight), ponderation));
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
    val: &Value,
    shapes: &HashMap<String, (Chooser<Pattern>, Chooser<Tiling>)>,
) -> (Chooser<Pattern>, Chooser<Tiling>) {
    let mut tilings = Chooser::new(vec![]);
    let mut patterns = Chooser::new(vec![]);
    match val {
        Value::Array(arr) => {
            for x in arr {
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
        _ => println!("{} is not an array of shapes.", val),
    }
    (patterns, tilings)
}

/// Read shape from one of its names
fn add_shape(s: &str, w: usize, tilings: &mut Chooser<Tiling>, patterns: &mut Chooser<Pattern>) {
    match &s[..] {
        "H" | "hex." | "hexagons" => tilings.push(Tiling::Hexagons, w),
        "T" | "tri." | "triangles" => tilings.push(Tiling::Triangles, w),
        "H&T" | "hex.&tri." | "hexagons&squares" => tilings.push(Tiling::HexagonsAndTriangles, w),
        "S&T" | "squ.&tri." | "squares&triangles" => tilings.push(Tiling::SquaresAndTriangles, w),
        "R" | "rho." | "rhombus" => tilings.push(Tiling::Rhombus, w),
        "D" | "del." | "delaunay" => tilings.push(Tiling::Delaunay, w),
        "FC" | "f-cir." | "free-circles" => patterns.push(Pattern::FreeCircles, w),
        "FT" | "f-tri." | "free-triangles" => patterns.push(Pattern::FreeTriangles, w),
        "FR" | "f-str." | "free-stripes" => patterns.push(Pattern::FreeStripes, w),
        "FP" | "f-spi." | "free-spirals" => patterns.push(Pattern::FreeSpirals, w),
        "CC" | "c-cir." | "concentric-circles" => patterns.push(Pattern::ConcentricCircles, w),
        "PS" | "p-str." | "parallel-stripes" => patterns.push(Pattern::ParallelStripes, w),
        "CS" | "c-str." | "crossed-stripes" => patterns.push(Pattern::CrossedStripes, w),
        "PW" | "p-wav." | "parallel-waves" => patterns.push(Pattern::ParallelWaves, w),
        "PT" | "p-saw." | "parallel-sawteeth" => patterns.push(Pattern::ParallelSawteeth, w),
        _ => println!("{} is not recognized as a shape", s),
    }
}

fn choose_theme_shapes(
    rng: &mut ThreadRng,
    entry: &Option<Vec<ConfigEntry>>,
    time: usize,
) -> (String, String, String) {
    match entry {
        None => (String::from(""), String::from(""), String::from("")),
        Some(v) => {
            let mut valid = Chooser::new(vec![]);
            for e in v {
                let markers = e
                    .span
                    .as_ref()
                    .unwrap_or(&"-".to_string())
                    .split('-')
                    .map(String::from)
                    .collect::<Vec<_>>();
                let start = markers
                    .get(0)
                    .as_ref()
                    .unwrap_or(&&String::from("0"))
                    .parse::<usize>()
                    .unwrap_or(0);
                let end = markers
                    .get(1)
                    .as_ref()
                    .unwrap_or(&&String::from("2400"))
                    .parse::<usize>()
                    .unwrap_or(2400);
                if start <= time && time <= end {
                    valid.push(e, e.weight.unwrap_or(1));
                }
            }
            match valid.choose(rng) {
                None => (String::from(""), String::from(""), String::from("")),
                Some(chosen_entry) => {
                    let chosen_theme = match &chosen_entry.themes {
                        None => String::from(""),
                        Some(th) => th
                            .choose(rng)
                            .map(String::from)
                            .unwrap_or_else(|| String::from("")),
                    };
                    let chosen_shapes = match &chosen_entry.shapes {
                        None => String::from(""),
                        Some(sh) => sh
                            .choose(rng)
                            .map(String::from)
                            .unwrap_or_else(|| String::from("")),
                    };
                    let line_color = chosen_entry
                        .line_color
                        .as_ref()
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| String::from(""));
                    (chosen_theme, chosen_shapes, line_color)
                }
            }
        }
    }
}

impl ConfigLines {
    fn get_settings(&self, tiling: Tiling, colors: &HashMap<String, Color>) -> (f64, Color) {
        let (w, c) = match tiling {
            Tiling::Hexagons => (self.hex_width, &self.hex_color),
            Tiling::Triangles => (self.tri_width, &self.tri_color),
            Tiling::HexagonsAndTriangles => (self.hex_and_tri_width, &self.hex_and_tri_color),
            Tiling::SquaresAndTriangles => (self.squ_and_tri_width, &self.squ_and_tri_color),
            Tiling::Rhombus => (self.rho_width, &self.rho_color),
            Tiling::Delaunay => (self.del_width, &self.del_color),
        };
        (
            w.unwrap_or_else(|| self.width.unwrap_or(LINE_WIDTH)),
            match c {
                Some(c) => color_from_value(&Value::String(c.to_string()), colors).ok(),
                None => None,
            }
            .unwrap_or_else(|| {
                match &self.color {
                    Some(color) => color_from_value(&Value::String(color.to_string()), colors).ok(),
                    None => None,
                }
                .unwrap_or(LINE_COLOR)
            }),
        )
    }
}

const DEVIATION: i32 = 20;
const WEIGHT: i32 = 40;
const SIZE: f64 = 15.;
const WIDTH: usize = 1000;
const HEIGHT: usize = 600;
const NB_FREE_CIRCLES: usize = 10;
const NB_FREE_TRIANGLES: usize = 15;
const NB_FREE_STRIPES: usize = 7;
const NB_PARALLEL_STRIPES: usize = 15;
const NB_CONCENTRIC_CIRCLES: usize = 5;
const NB_CROSSED_STRIPES: usize = 10;
const NB_FREE_SPIRALS: usize = 3;
const NB_PARALLEL_WAVES: usize = 15;
const NB_PARALLEL_SAWTEETH: usize = 15;
const VAR_PARALLEL_STRIPES: usize = 15;
const VAR_CROSSED_STRIPES: usize = 10;
const WIDTH_SPIRAL: f64 = 0.3;
const WIDTH_STRIPE: f64 = 0.1;
const WIDTH_WAVE: f64 = 0.3;
const WIDTH_SAWTOOTH: f64 = 0.3;
const NB_DELAUNAY: usize = 1000;
const LINE_WIDTH: f64 = 1.0;
const LINE_COLOR: Color = Color(0, 0, 0);
