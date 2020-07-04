use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;
use std::f64::consts::PI;

#[derive(Clone, Copy, Debug)]
pub struct Pos(f64, f64);

impl Pos {
    pub fn into_tuple(self) -> (f64, f64) {
        (self.0, self.1)
    }
}

struct Movable(Vec<Pos>);

struct Hexagon {
    size: f64,
    rot: i32,
}

pub fn polar(a: f64, r: f64) -> Pos {
    Pos(r * a.cos(), r*a.sin())
}

pub fn radians(a: i32) -> f64 {
    (a as f64) * PI / 180.
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

    pub fn hexagon(h: Hexagon) -> Self {
        let mut pts = Vec::new();
        for i in 0..6 {
            pts.push(polar(radians(h.rot + 60*i), h.size))
        }
        Movable(pts)
    }
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

#[derive(Clone, Copy)]
pub struct Frame {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl Frame {
    pub fn into_tuple(self) -> (usize, usize, usize, usize) {
        (self.x, self.y, self.x + self.w, self.y + self.h)
    }

    pub fn center(&self) -> Pos {
        Pos((self.x + self.w / 2) as f64, (self.y + self.h / 2) as f64)
    }

    pub fn is_inside(&self, pos: Pos) -> bool {
        let xerr = (self.w as f64) / 10.;
        let yerr = (self.h as f64) / 10.;
        (self.x as f64 - xerr) < pos.0 && pos.0 < (self.x + self.w) as f64 + xerr
        && (self.y as f64 - yerr) < pos.1 && pos.1 < (self.y + self.h) as f64 + yerr
    }
fn main() {
    let frame = Frame {x: 0, y: 0, w: 1000, h: 600};
    let mut document = Document::new()
        .set("viewBox", frame.into_tuple());

    for elem in frame.hexfill(Hexagon {size: 14., rot: 15}) {
        document = document.add(elem)
    }

    svg::save("image.svg", &document).unwrap();
}
