use crate::prelude::*;
use crate::svg::*;

/// A generic shape that can be placed at any position according to a given center
pub struct Movable(Vec<Pos>);

impl Movable {
    pub fn render(&self, reference: Pos) -> (Pos, Path) {
        let mut data = Data::new(reference + self.0[0]);
        for p in self.0.iter().skip(1) {
            data.line_to(reference + *p);
        }
        (reference, Path::new(data))
    }

    pub fn hexagon(size: f64, rot: i32) -> Self {
        let mut pts = Vec::new();
        for i in 0..6 {
            pts.push(Pos::polar(rot + 60 * i, size))
        }
        Movable(pts)
    }

    pub fn triangle(size: f64, rot: i32) -> Self {
        let mut pts = Vec::new();
        for i in 0..3 {
            pts.push(Pos::polar(rot + 120 * i, size))
        }
        Movable(pts)
    }

    pub fn square(size: f64, rot: i32) -> Self {
        let mut pts = Vec::new();
        for i in 0..4 {
            pts.push(Pos::polar(rot + 45 + 90 * i, size))
        }
        Movable(pts)
    }
}
