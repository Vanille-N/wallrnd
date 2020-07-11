use serde_derive::Deserialize;
use toml::{map::Map, Value};

#[derive(Deserialize, Default)]
pub struct MetaConfig {
    pub global: Option<ConfigGlobal>,
    pub colors: Option<ConfigColors>,
    pub themes: Option<ConfigThemes>,
    pub shapes: Option<ConfigShapes>,
    pub data: Option<ConfigData>,
    pub entry: Option<Vec<ConfigEntry>>,
}

#[derive(Deserialize, Default)]
pub struct ConfigGlobal {
    pub deviation: Option<usize>,
    pub weight: Option<usize>,
    pub size: Option<f64>,
    pub width: Option<usize>,
    pub height: Option<usize>,
}

#[derive(Deserialize, Default, Debug)]
pub struct ConfigColors {
    #[serde(flatten)]
    pub list: Map<String, Value>,
}

#[derive(Deserialize, Default, Debug)]
pub struct ConfigThemes {
    #[serde(flatten)]
    pub list: Map<String, Value>,
}

#[derive(Deserialize, Default, Debug)]
pub struct ConfigShapes {
    #[serde(flatten)]
    pub list: Map<String, Value>,
}

#[derive(Deserialize, Default, Debug)]
pub struct ConfigData {
    pub patterns: Option<ConfigPatterns>,
    pub tilings: Option<ConfigTilings>,
}

#[derive(Deserialize, Default, Debug)]
pub struct ConfigTilings {
    pub size_hex: Option<f64>,
    pub size_tri: Option<f64>,
    pub size_hex_and_tri: Option<f64>,
    pub size_squ_and_tri: Option<f64>,
    pub nb_del: Option<usize>,
}

#[derive(Deserialize, Default, Debug)]
pub struct ConfigPatterns {
    pub nb_f_cir: Option<usize>,
    pub nb_f_spi: Option<usize>,
    pub nb_f_str: Option<usize>,
    pub nb_c_str: Option<usize>,
    pub nb_p_str: Option<usize>,
    pub nb_c_cir: Option<usize>,
    pub nb_f_tri: Option<usize>,
    pub var_p_str: Option<usize>,
    pub var_c_str: Option<usize>,
    pub width_spi: Option<f64>,
    pub width_str: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct ConfigEntry {
    pub start: Option<String>,
    pub end: Option<String>,
    pub weight: Option<usize>,
    pub themes: Vec<String>,
    pub shapes: Vec<String>,
}
