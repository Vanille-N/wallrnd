use crate::color::Color;
use crate::pos::Pos;

pub struct Scene {
    bg: Color,
    items: Vec<Box<dyn Contains>>,
}

impl Scene {
    pub fn new() -> Self {
        unimplemented!()
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
