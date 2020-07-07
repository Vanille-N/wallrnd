use crate::color::Color;
use crate::pos::Pos;
use rand::{rngs::ThreadRng, Rng};
use crate::tesselation::Frame;
use crate::cfg::SceneCfg;

pub struct Scene {
    bg: ColorItem,
    items: Vec<Box<dyn Contains>>,
}

impl Scene {
    pub fn new(cfg: &SceneCfg, rng: &mut ThreadRng) -> Self {
        Self {
            bg: cfg.choose_color(rng),
            items: cfg.create_items(rng),
        }
    }

    pub fn color(&self, p: Pos, rng: &mut ThreadRng) -> Color {
        for i in &self.items {
            if let Some(c) = i.contains(p, rng) {
                return c;
            }
        }
        return self.bg.sample(rng);
    }
}

pub trait Contains {
    fn contains(&self, p: Pos, rng: &mut ThreadRng) -> Option<Color>;
}

pub struct ColorItem {
    pub shade: Color,
    pub deviation: i32,
    pub theme: Color,
    pub weight: i32,
}

impl ColorItem {
    pub fn sample(&self, rng: &mut ThreadRng) -> Color {
        self.shade.variate(rng, self.deviation).meanpoint(self.theme, self.weight)
    }
}

pub struct Disc {
    center: Pos,
    radius: f64,
    color: ColorItem,
}

impl Ord for Disc {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.radius.partial_cmp(&other.radius).unwrap()
    }
}

impl PartialOrd for Disc {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Disc {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

impl Eq for Disc {}

impl Disc {
    pub fn random(rng: &mut ThreadRng, f: &Frame, color: ColorItem, size_hint: f64) -> Self {
        let center = Pos::random(f, rng);
        let radius = (rng.gen::<f64>() + 0.1) * (f.h.min(f.w) as f64 * size_hint);
        Self {
            center,
            radius,
            color,
        }
    }
}

impl Contains for Disc {
    fn contains(&self, p: Pos, rng: &mut ThreadRng) -> Option<Color> {
        if (self.center - p).dot_self() < self.radius.powi(2) {
            Some(self.color.sample(rng))
        } else {
            None
        }
    }
}
