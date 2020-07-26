use crate::cfg::SceneCfg;
use crate::pos::crossprod_sign;
use crate::prelude::*;
use rand::{rngs::ThreadRng, Rng};
use std::rc::Rc;

pub struct Scene {
    pub bg: ColorItem,
    pub items: Vec<Rc<dyn Contains>>,
}

impl Scene {
    pub fn new(cfg: &SceneCfg, rng: &mut ThreadRng, verbose: Verbosity) -> Self {
        Self {
            bg: cfg.choose_color(rng),
            items: cfg.create_items(rng, verbose),
        }
    }

    /// Get color of a position depending on objects that were hit
    pub fn color(&self, p: Pos, rng: &mut ThreadRng) -> Color {
        for i in &self.items {
            if let Some(c) = i.contains(p, rng) {
                return c;
            }
        }
        self.bg.sample(rng)
    }
}

/// Trait for anything that can contain a 2D point
pub trait Contains: std::fmt::Display {
    fn contains(&self, p: Pos, rng: &mut ThreadRng) -> Option<Color>;
}

#[derive(Debug, Clone)]
pub struct ColorItem {
    pub shade: Color,
    pub deviation: i32,
    pub theme: Color,
    pub weight: i32,
}

impl ColorItem {
    pub fn sample(&self, rng: &mut ThreadRng) -> Color {
        self.shade
            .meanpoint(self.theme, self.weight)
            .variate(rng, self.deviation)
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct HalfPlane {
    pub limit: Pos,
    pub reference: Pos,
    pub color: ColorItem,
}

impl HalfPlane {
    pub fn random(rng: &mut ThreadRng, limit: Pos, indic: i32, var: i32, color: ColorItem) -> Self {
        Self {
            limit,
            reference: limit + Pos::polar(rng.gen_range(indic - var, indic + var), 100.),
            color,
        }
    }
}

impl Contains for HalfPlane {
    fn contains(&self, p: Pos, rng: &mut ThreadRng) -> Option<Color> {
        let dotprod = (p - self.limit).dot(self.reference - self.limit);
        if dotprod < 0. {
            Some(self.color.sample(rng))
        } else {
            None
        }
    }
}

#[derive(Debug)]
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
            a: circ.center + Pos::polar(theta0, circ.radius),
            b: circ.center + Pos::polar(theta0 + theta1, circ.radius),
            c: circ.center + Pos::polar(theta0 + theta1 + theta2, circ.radius),
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

#[derive(Debug)]
pub struct Spiral {
    pub center: Pos,
    pub width: f64,
    pub color: ColorItem,
}

impl Spiral {
    pub fn random(rng: &mut ThreadRng, f: &Frame, color: ColorItem, width: f64) -> Self {
        Self {
            center: Pos::random(f, rng),
            width,
            color,
        }
    }
}

impl Contains for Spiral {
    fn contains(&self, p: Pos, rng: &mut ThreadRng) -> Option<Color> {
        let Pos(di, dj) = self.center - p;
        let theta = di.atan2(dj);
        let radius = (di.powi(2) + dj.powi(2)).sqrt() + theta / std::f64::consts::PI * self.width;
        if (radius / self.width).floor() as i32 % 2 == 0 {
            Some(self.color.sample(rng))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Stripe {
    pub limit: Pos,
    pub reference: Pos,
    pub color: ColorItem,
}

impl Stripe {
    pub fn random(rng: &mut ThreadRng, f: &Frame, color: ColorItem, width: f64) -> Self {
        let limit = Pos::random(f, rng);
        let reference = limit + Pos::polar(rng.gen_range(0, 360), width);
        Self {
            limit,
            reference,
            color,
        }
    }
}

impl Contains for Stripe {
    fn contains(&self, p: Pos, rng: &mut ThreadRng) -> Option<Color> {
        let dotprod1 = (p - self.limit).dot(self.reference - self.limit);
        let dotprod2 = (p - self.reference).dot(self.limit - self.reference);
        if dotprod1 > 0. && dotprod2 > 0. {
            Some(self.color.sample(rng))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Wave {
    pub limit: Pos,
    pub reference: Pos,
    pub amplitude: f64,
    pub frequency: f64,
    pub color: ColorItem,
}

impl Wave {
    pub fn random(
        _rng: &mut ThreadRng,
        limit: Pos,
        indic: i32,
        width: f64,
        amplitude: f64,
        color: ColorItem,
    ) -> Self {
        Self {
            limit,
            reference: limit + Pos::polar(indic, 100.),
            amplitude,
            frequency: 0.002 / width,
            color,
        }
    }
}

impl Contains for Wave {
    fn contains(&self, p: Pos, rng: &mut ThreadRng) -> Option<Color> {
        let proj = (p - self.limit).project(self.reference - self.limit);
        let nearpt = p - proj;
        let phase = (self.limit - nearpt).norm() * self.frequency;
        if phase.cos() * self.amplitude > (p - self.limit).dot((self.reference - self.limit).unit())
        {
            Some(self.color.sample(rng))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Sawtooth {
    pub limit: Pos,
    pub reference: Pos,
    pub amplitude: f64,
    pub frequency: f64,
    pub color: ColorItem,
}

impl Sawtooth {
    pub fn random(
        _rng: &mut ThreadRng,
        limit: Pos,
        indic: i32,
        width: f64,
        amplitude: f64,
        color: ColorItem,
    ) -> Self {
        Self {
            limit,
            reference: limit + Pos::polar(indic, 100.),
            amplitude,
            frequency: 0.001 / width,
            color,
        }
    }
}

impl Contains for Sawtooth {
    fn contains(&self, p: Pos, rng: &mut ThreadRng) -> Option<Color> {
        let sawtooth = |f: f64| {
            let int = f.floor();
            let frac = f - int;
            match (int as i32).rem_euclid(4) {
                0 => frac,
                1 => 1. - frac,
                2 => -frac,
                3 => frac - 1.,
                _ => unreachable!(),
            }
        };
        let proj = (p - self.limit).project(self.reference - self.limit);
        let nearpt = p - proj;
        let phase = (self.limit - nearpt).norm() * self.frequency;
        if sawtooth(phase) * self.amplitude > (p - self.limit).dot((self.reference - self.limit).unit())
        {
            Some(self.color.sample(rng))
        } else {
            None
        }
    }
}
