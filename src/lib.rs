pub mod cfg;
pub mod chooser;
pub mod color;
pub mod deserializer;
pub mod frame;
pub mod log;
pub mod paint;
pub mod pos;
pub mod salt;
pub mod scene;
pub mod shape;
pub mod svg;
pub mod tesselate;

pub mod prelude {
    pub use super::Verbosity;
    use super::*;
    pub use cfg::{Pattern, Tiling};
    pub use chooser::Chooser;
    pub use color::Color;
    pub use frame::Frame;
    pub use pos::{radians, Pos};
    pub use salt::{Salt, SaltItem};

    use std::collections::HashMap;
    pub type ColorList = HashMap<String, Color>;
    pub type ThemeList = HashMap<String, Chooser<ThemeItem>>;

    #[derive(Clone, Debug)]
    pub struct ThemeItem(pub Color, pub Option<usize>, pub Option<usize>, pub Salt);
}

#[derive(Clone, Copy, Default)]
pub struct Verbosity {
    pub info: bool,
    pub warn: bool,
    pub prog: bool,
    pub details: bool,
}

impl Verbosity {
    pub fn from(s: &str) -> Self {
        let mut v = Verbosity::default();
        for option in s.chars() {
            match option {
                'A' => {
                    v.info = true;
                    v.warn = true;
                    v.prog = true;
                    v.details = true;
                }
                'I' => v.info = true,
                'W' => v.warn = true,
                'P' => v.prog = true,
                'D' => v.details = true,
                c => println!(
                    "Unknown verbosity option '{}', use one or more of 'IWPDA'",
                    c
                ),
            }
        }
        v
    }
}
