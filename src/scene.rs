use crate::color::Color;
use crate::pos::Pos;
use rand::{rngs::ThreadRng, Rng};
use crate::tesselation::Frame;
use crate::cfg::SceneCfg;
use crate::pos::{radians, polar, crossprod_sign};

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
    pub center: Pos,
    pub radius: f64,
    pub color: ColorItem,
}

impl Disc {
    pub fn random(rng: &mut ThreadRng, f: &Frame, color: ColorItem, size_hint: f64) -> Self {
        let center = Pos::random(f, rng);
        let radius = (rng.gen::<f64>() * size_hint + 0.1) * (f.h.min(f.w) as f64);
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

pub struct HalfPlane {
    pub limit: Pos,
    pub reference: Pos,
    pub color: ColorItem,
}

impl HalfPlane {
    pub fn random(rng: &mut ThreadRng, limit: Pos, indic: Pos, var: i32, color: ColorItem) -> Self {
        Self {
            limit,
            reference: indic + polar(radians(rng.gen_range(-var, var)), 100.),
            color,
        }
    }
}

impl Contains for HalfPlane {
    fn contains(&self, p: Pos, rng: &mut ThreadRng) -> Option<Color> {
        unimplemented!()
    }
}

pub struct Triangle {
    pub a: Pos,
    pub b: Pos,
    pub c: Pos,
    pub color: ColorItem,
}

impl Triangle {
    pub fn random(rng: &mut ThreadRng, circ: Disc) -> Self {
        let theta0 = rng.gen_range(0, 360);
        let theta1 = rng.gen_range(80, 150);
        let theta2 = rng.gen_range(80, 150);
        Self {
            a: circ.center + polar(radians(theta0), circ.radius),
            b: circ.center + polar(radians(theta0 + theta1), circ.radius),
            c: circ.center + polar(radians(theta0 + theta1 + theta2), circ.radius),
            color: circ.color,
        }
    }
}

impl Contains for Triangle {
    fn contains(&self, p: Pos, rng: &mut ThreadRng) -> Option<Color> {
        let d1 = crossprod_sign(p, self.a, self.b);
        let d2 = crossprod_sign(p, self.b, self.c);
        let d3 = crossprod_sign(p, self.c, self.a);
        let has_pos = d1 || d2 || d3;
        let has_neg = !(d1 && d2 && d3);
        if !(has_neg && has_pos) {
            Some(self.color.sample(rng))
        } else {
            None
        }
    }
}

pub struct Spiral {
}

impl Spiral {
    pub fn random(rng: &mut ThreadRng, f: &Frame, color: ColorItem, size_hint: f64) -> Self {
        unimplemented!()
    }
}

impl Contains for Spiral {
    fn contains(&self, p: Pos, rng: &mut ThreadRng) -> Option<Color> {
        unimplemented!()
    }
}

pub struct Stripe {
}

impl Stripe {
    pub fn random(rng: &mut ThreadRng, f: &Frame, color: ColorItem, size_hint: f64) -> Self {
        unimplemented!()
    }
}

impl Contains for Stripe {
    fn contains(&self, p: Pos, rng: &mut ThreadRng) -> Option<Color> {
        unimplemented!()
    }
}
