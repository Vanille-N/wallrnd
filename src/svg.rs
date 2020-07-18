use crate::prelude::*;

pub struct Path {
    stroke_width: f64,
    stroke_color: Color,
    fill_color: Color,
    data: Data,
}

pub struct Data(Vec<Pos>);

pub struct Document {
    frame: Frame,
    items: Vec<Path>,
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

    pub fn save(&self, dest: String) -> Result<(), ()> {
        println!("{}", &self);
        Ok(())
    }
}

