use crate::color::Color;
use crate::pos::Pos;
use rand::{rngs::ThreadRng, Rng, seq::SliceRandom};
use crate::tesselation::Frame;

pub struct Scene {
    bg: ColorItem,
    items: Vec<Box<dyn Contains>>,
}

impl Scene {
    pub fn new(cfg: &SceneCfg, rng: &mut ThreadRng) -> Self {
        Self {
            bg: cfg.choose_color(rng),
            items: create_items(cfg, rng),
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

pub fn create_items(cfg: &SceneCfg, rng: &mut ThreadRng) -> Vec<Box<dyn Contains>> {
    vec![Box::new(Disc {
        center: Pos(500., 400.),
        radius: 100.,
        color: cfg.choose_color(rng),
    })]
}

struct ColorItem {
    shade: Color,
    deviation: i32,
    theme: Color,
    weight: i32,
}

impl ColorItem {
    fn sample(&self, rng: &mut ThreadRng) -> Color {
        self.shade.variate(rng, self.deviation).meanpoint(self.theme, self.weight)
    }
}


struct Disc {
    center: Pos,
    radius: f64,
    color: ColorItem,
}

impl Disc {
    fn random(rng: &mut ThreadRng, f: &Frame, color: ColorItem) -> Self {
        let center = Pos::random(f, rng);
        let radius = (rng.gen::<f64>() + 0.1) * (f.h.max(f.w) as f64);
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

pub struct SceneCfg {
    pub themes: Vec<Color>,
    pub weight: i32,
    pub deviation: i32,
}

impl SceneCfg {
    fn choose_color(&self, rng: &mut ThreadRng) -> ColorItem {
        ColorItem {
            shade: Color::random(rng),
            deviation: self.deviation,
            weight: self.weight,
            theme: *self.themes.choose(rng).unwrap(),
        }
    }
}
