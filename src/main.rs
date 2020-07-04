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
            .set("fill", "none")
            .set("stroke", "black")
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

impl From<Pos> for (f64, f64) {
    fn from(p: Pos) -> (f64, f64) {
        (p.0, p.1)
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


fn main() {
    let mut document = Document::new()
        .set("viewBox", (0, 0, 100, 100));

    let h = Movable::hexagon(Hexagon {size: 14., rot: 60});
    let document = document.add(h.render(Pos(20., 20.)));

    svg::save("image.svg", &document).unwrap();
}
