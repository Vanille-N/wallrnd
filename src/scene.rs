use crate::color::Color;
use crate::pos::Pos;
use rand::rngs::ThreadRng;

pub struct Scene {
    bg: ColorItem,
    items: Vec<Box<dyn Contains>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            bg: ColorItem {
                shade: Color(50, 50,50),
                deviation: 20,
                theme: Color(0, 100, 100),
                weight: 20,
            },
            items: create_items(),
        }
    }

    pub fn color(&self, p: Pos, rng: &mut ThreadRng) -> Color {
        for i in &self.items {
            if let Some(c) = i.contains(p, rng) {
                return c;
            }
        }
        return self.bg.sample(rng);
    }
}

pub trait Contains {
    fn contains(&self, p: Pos, rng: &mut ThreadRng) -> Option<Color>;
}

pub fn create_items() -> Vec<Box<dyn Contains>> {
    vec![Box::new(Disc {
        center: Pos(500., 400.),
        radius: 100.,
        color: ColorItem {
            shade: Color(255, 0, 0),
            deviation: 20,
            theme: Color(0, 100, 0),
            weight: 20,
        },
    })]
}

struct ColorItem {
    shade: Color,
    deviation: i32,
    theme: Color,
    weight: i32,
}

impl ColorItem {
    fn sample(&self, rng: &mut ThreadRng) -> Color {
        self.shade.variate(rng, self.deviation).meanpoint(self.theme, self.weight)
    }
}


struct Disc {
    center: Pos,
    radius: f64,
    color: ColorItem,
}

impl Contains for Disc {
    fn contains(&self, p: Pos, rng: &mut ThreadRng) -> Option<Color> {
        if (self.center - p).dot_self() < self.radius.powi(2) {
            Some(self.color.sample(rng))
        } else {
            None
        }
    }
}
