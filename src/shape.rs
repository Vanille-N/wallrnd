use crate::pos::*;
use svg::node::element::path::Data;
use svg::node::element::Path;

pub struct Movable(Vec<Pos>);

#[derive(Clone, Copy, Debug)]
pub struct Shape {
    pub size: f64,
    pub rot: i32,
}

impl Shape {
    pub fn rotate(mut self, a: i32) -> Self {
        self.rot += a;
        self
    }

    pub fn redim(mut self, s: f64) -> Self {
        self.size *= s;
        self
    }
}

impl Movable {
    pub fn render(&self, reference: Pos) -> Path {
        let mut data = Data::new();
        data = data.move_to((reference + self.0[0]).into_tuple());
        for p in self.0.iter().skip(1) {
            data = data.line_to((reference + *p).into_tuple());
        }
        let data = data.close();
        Path::new()
            .set("fill", "lawngreen")
            .set("stroke", "blue")
            .set("stroke-width", 1)
            .set("d", data)
    }

    pub fn hexagon(x: Shape) -> Self {
        let mut pts = Vec::new();
        for i in 0..6 {
            pts.push(polar(radians(x.rot + 60 * i), x.size))
        }
        Movable(pts)
    }

    pub fn triangle(x: Shape) -> Self {
        let mut pts = Vec::new();
        for i in 0..3 {
            pts.push(polar(radians(x.rot + 120 * i), x.size))
        }
        Movable(pts)
    }
}
