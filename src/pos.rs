use std::f64::consts::PI;

#[derive(Clone, Copy, Debug)]
pub struct Pos(pub f64, pub f64);

impl Pos {
    pub fn into_tuple(self) -> (f64, f64) {
        (self.0, self.1)
    }
}

pub fn polar(a: f64, r: f64) -> Pos {
    Pos(r * a.cos(), r * a.sin())
}

pub fn radians(a: i32) -> f64 {
    (a as f64) * PI / 180.
}

impl std::ops::Add<(f64, f64)> for Pos {
    type Output = Self;
    fn add(self, (x, y): (f64, f64)) -> Self::Output {
        Pos(self.0 + x, self.1 + y)
    }
}
impl std::ops::Add<Pos> for Pos {
    type Output = Self;
    fn add(self, Pos(x, y): Pos) -> Self::Output {
        Pos(self.0 + x, self.1 + y)
    }
}
impl std::ops::Mul<isize> for Pos {
    type Output = Self;
    fn mul(self, x: isize) -> Self::Output {
        Pos(self.0 * x as f64, self.1 * x as f64)
    }
}
impl std::ops::Mul<f64> for Pos {
    type Output = Self;
    fn mul(self, x: f64) -> Self::Output {
        Pos(self.0 * x as f64, self.1 * x as f64)
    }
}
