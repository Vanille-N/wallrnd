pub mod cfg;
pub mod chooser;
pub mod color;
pub mod deserializer;
pub mod frame;
pub mod paint;
pub mod pos;
pub mod scene;
pub mod shape;
pub mod svg;
pub mod tesselate;
pub mod log;

pub mod prelude {
    use super::*;
    pub use cfg::{Pattern, Tiling};
    pub use chooser::Chooser;
    pub use color::Color;
    pub use frame::Frame;
    pub use pos::{radians, Pos};
    pub use super::Verbosity;
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
                c => println!("Unknown verbosity option '{}', use one or more of 'IWPDA'", c),
            }
        }
        v
    }
}
