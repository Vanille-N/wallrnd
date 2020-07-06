use crate::color::Color;
use crate::pos::Pos;

pub struct Scene {
    bg: Color,
    items: Vec<Box<dyn Contains>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            bg: Color(50, 50, 50),
            items: create_items(),
        }
    }

    pub fn color(&self, p: Pos) -> Color {
        for i in &self.items {
            if let Some(c) = i.contains(p) {
                return c;
            }
        }
        return self.bg;
    }
}

pub trait Contains {
    fn contains(&self, p: Pos) -> Option<Color>;
}

pub fn create_items() -> Vec<Box<dyn Contains>> {
    vec![Box::new(Disc {
        center: Pos(500., 400.),
        radius: 100.,
        color: Color(255, 0, 0),
    })]
}

struct Disc {
    center: Pos,
    radius: f64,
    color: Color,
}

impl Contains for Disc {
    fn contains(&self, p: Pos) -> Option<Color> {
        if (self.center - p).dot_self() < self.radius.powi(2) {
            Some(self.color)
        } else {
            None
        }
    }
}
