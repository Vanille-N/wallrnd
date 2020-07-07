use crate::color::Color;
use crate::tesselation::*;
use crate::scene::*;
use rand::{rngs::ThreadRng, seq::SliceRandom};

pub struct SceneCfg {
    pub themes: Vec<Color>,
    pub weight: i32,
    pub deviation: i32,
    pub frame: Frame,
    pub pattern: Pattern,
    pub tiling: Tiling,
}

impl SceneCfg {
    pub fn choose_color(&self, rng: &mut ThreadRng) -> ColorItem {
        ColorItem {
            shade: Color::random(rng),
            deviation: self.deviation,
            weight: self.weight,
            theme: *self.themes.choose(rng).unwrap(),
        }
    }

    pub fn create_items(&self, rng: &mut ThreadRng) -> Vec<Box<dyn Contains>> {
        let mut v = Vec::new();
        for i in 0..10 {
            let c = self.choose_color(rng);
            v.push(Disc::random(rng, &self.frame, c, i as f64/10.));
        }
        v.sort();
        v.into_iter().map(|d| Box::new(d) as Box<dyn Contains>).collect::<Vec<_>>()
    }
}

pub enum Pattern {
    ConcentricCircles,
    FreeCircles,
    FreeTriangles,
    FreeStripes,
    FreeSpirals
}

pub enum Tiling {
    Squares,
    Hexagons,
    HexagonsAndTriangles,
    SquaresAndTriangles,
    Delaunay,
}
