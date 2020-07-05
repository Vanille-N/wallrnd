struct Movable(Vec<Pos>);

#[derive(Clone, Copy, Debug)]
struct Hexagon {
    size: f64,
    rot: i32,
}

#[derive(Clone, Copy, Debug)]
struct Triangle {
    size: f64,
    rot: i32,
}

impl Triangle {
    fn rotate(mut self, a: i32) -> Self {
        self.rot += a;
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

    pub fn hexagon(h: Hexagon) -> Self {
        let mut pts = Vec::new();
        for i in 0..6 {
            pts.push(polar(radians(h.rot + 60 * i), h.size))
        }
        Movable(pts)
    }

    pub fn triangle(t: Triangle) -> Self {
        let mut pts = Vec::new();
        for i in 0..3 {
            pts.push(polar(radians(t.rot + 120 * i), t.size))
        }
        Movable(pts)
    }
}
