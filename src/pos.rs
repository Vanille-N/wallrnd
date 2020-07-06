use std::cmp::{Eq, PartialEq};
use std::f64::consts::PI;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Mul, Sub};
use rand::{rngs::ThreadRng, Rng};
use crate::tesselation::Frame;

#[derive(Clone, Copy, Debug)]
pub struct Pos(pub f64, pub f64);

impl Pos {
    pub fn into_tuple(self) -> (f64, f64) {
        (self.0, self.1)
    }

    pub fn round(self) -> (i32, i32) {
        (
            (self.0 * 100.).round() as i32,
            (self.1 * 100.).round() as i32,
        )
    }

    pub fn dot_self(self) -> f64 {
        self.0.powi(2) + self.1.powi(2)
    }

    pub fn dot(self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1
    }

    pub fn random(f: &Frame, rng: &mut ThreadRng) -> Self {
        let errx = f.w as f64 / 10.;
        let erry = f.h as f64 / 10.;
        let x = f.x as f64 - errx + rng.gen::<f64>() * f.w as f64 * 1.2;
        let y = f.y as f64 - erry + rng.gen::<f64>() * f.h as f64 * 1.2;
        Self(x, y)
    }
}

pub fn crossprod_sign(a: Pos, b: Pos, c: Pos) -> bool {
    (a.0 - c.0) * (b.1 - c.1) - (b.0 - c.0) * (a.1 - c.1) > 0.
}

pub fn polar(a: f64, r: f64) -> Pos {
    Pos(r * a.cos(), r * a.sin())
}

pub fn radians(a: i32) -> f64 {
    (a as f64) * PI / 180.
}

impl Add<(f64, f64)> for Pos {
    type Output = Self;
    fn add(self, (x, y): (f64, f64)) -> Self::Output {
        Pos(self.0 + x, self.1 + y)
    }
}

impl Add<Pos> for Pos {
    type Output = Self;
    fn add(self, Pos(x, y): Pos) -> Self::Output {
        Pos(self.0 + x, self.1 + y)
    }
}

impl Sub<Pos> for Pos {
    type Output = Self;
    fn sub(self, Pos(x, y): Pos) -> Self::Output {
        Pos(self.0 - x, self.1 - y)
    }
}

impl Mul<isize> for Pos {
    type Output = Self;
    fn mul(self, x: isize) -> Self::Output {
        Pos(self.0 * x as f64, self.1 * x as f64)
    }
}

impl Mul<f64> for Pos {
    type Output = Self;
    fn mul(self, x: f64) -> Self::Output {
        Pos(self.0 * x as f64, self.1 * x as f64)
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.round() == other.round()
    }
}

impl Eq for Pos {}

impl Hash for Pos {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.round().hash(state)
    }
}
