use crate::pos::*;
use svg::node::element::path::Data;
use svg::node::element::Path;

pub struct Movable(Vec<Pos>);

impl Movable {
    pub fn render(&self, reference: Pos) -> (Pos, Path) {
        let mut data = Data::new();
        data = data.move_to((reference + self.0[0]).into_tuple());
        for p in self.0.iter().skip(1) {
            data = data.line_to((reference + *p).into_tuple());
        }
        let data = data.close();
        (reference, Path::new().set("stroke-width", 1).set("d", data))
    }

    pub fn hexagon(size: f64, rot: i32) -> Self {
        let mut pts = Vec::new();
        for i in 0..6 {
            pts.push(polar(rot + 60 * i, size))
        }
        Movable(pts)
    }

    pub fn triangle(size: f64, rot: i32) -> Self {
        let mut pts = Vec::new();
        for i in 0..3 {
            pts.push(polar(rot + 120 * i, size))
        }
        Movable(pts)
    }

    pub fn square(size: f64, rot: i32) -> Self {
        let mut pts = Vec::new();
        for i in 0..4 {
            pts.push(polar(rot + 45 + 90 * i, size))
        }
        Movable(pts)
    }
}
