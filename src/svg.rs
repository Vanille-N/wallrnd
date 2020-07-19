use crate::prelude::*;
use std::fmt;
use std::io::Write;

pub struct Path {
    pub stroke_width: f64,
    pub stroke_color: Color,
    pub fill_color: Color,
    pub data: Data,
}

pub struct Data(pub Vec<Pos>);

pub struct Document {
    pub frame: Frame,
    pub items: Vec<Path>,
}

impl Data {
    pub fn new(pos: Pos) -> Self {
        Self(vec![pos])
    }

    pub fn line_to(&mut self, pos: Pos) {
        self.0.push(pos);
    }

    pub fn with_line_to(mut self, pos: Pos) -> Self {
        self.0.push(pos);
        self
    }
}

impl Path {
    pub fn new(d: Data) -> Self {
        Self {
            stroke_width: 0.0,
            stroke_color: Color(0, 0, 0),
            fill_color: Color(255, 255, 255),
            data: d,
        }
    }

    pub fn with_fill_color(mut self, c: Color) -> Self {
        self.fill_color = c;
        self
    }

    pub fn with_stroke_color(mut self, c: Color) -> Self {
        self.stroke_color = c;
        self
    }

    pub fn with_stroke_width(mut self, w: f64) -> Self {
        self.stroke_width = w;
        self
    }
}

impl Document {
    pub fn new(frame: Frame) -> Self {
        Self {
            frame,
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, path: Path) {
        self.items.push(path);
    }

    pub fn save(&self, dest: String) -> std::io::Result<()> {
        let mut buffer = std::fs::File::create(dest)?;
        buffer.write_all(&format!("{}", &self).into_bytes())
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<path d=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\" />",
            self.data, self.fill_color, self.stroke_color, self.stroke_width
        )
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.0.is_empty() {
            let Pos(x, y) = self.0[0];
            write!(f, "M{},{} ", x, y)?;
        }
        for Pos(x, y) in self.0.iter().skip(1) {
            write!(f, "L{},{} ", x, y)?;
        }
        write!(f, "z")
    }
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x1, y1, x2, y2) = self.frame.into_tuple();
        let src = String::from("http://www.w3.org/2000/svg");
        writeln!(
            f,
            "<svg viewBox=\"{} {} {} {}\" xmlns=\"{}\">",
            x1, y1, x2, y2, src
        )?;
        for p in self.items.iter() {
            writeln!(f, "{}", p)?;
        }
        write!(f, "</svg>")
    }
}
