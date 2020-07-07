use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Copy, Debug)]
pub struct Color(pub i32, pub i32, pub i32);

impl Color {
    pub fn to_string(self) -> String {
        let c = self.validate();
        format!("rgb({},{},{})", c.0, c.1, c.2)
    }

    fn validate(mut self) -> Self {
        self.0 = self.0.min(255).max(0);
        self.1 = self.1.min(255).max(0);
        self.2 = self.2.min(255).max(0);
        self
    }

    pub fn variate(mut self, rng: &mut ThreadRng, amount: i32) -> Self {
        self.0 = self.0 + rng.gen_range(-amount, amount);
        self.1 = self.1 + rng.gen_range(-amount, amount);
        self.2 = self.2 + rng.gen_range(-amount, amount);
        self
    }

    pub fn meanpoint(mut self, th: Self, weight: i32) -> Self {
        self.0 = (self.0 * weight + th.0 * (100 - weight)) / 100;
        self.1 = (self.1 * weight + th.1 * (100 - weight)) / 100;
        self.2 = (self.2 * weight + th.2 * (100 - weight)) / 100;
        self
    }

    pub fn random(rng: &mut ThreadRng) -> Self {
        Self(
            rng.gen_range(0, 255),
            rng.gen_range(0, 255),
            rng.gen_range(0, 255),
        )
    }
}

#[derive(Debug)]
pub struct Theme(usize, Vec<(Color, usize)>);

impl Theme {
    pub fn default() -> Self {
        Theme(1, vec![(Color(0, 0, 0), 1)])
    }

    pub fn new(mut v: Vec<(Color, usize)>) -> Self {
        let mut sum = 0;
        for (_, w) in &mut v {
            sum += *w;
            *w = sum;
        }
        if !v.is_empty() {
            Self(sum.max(1), v)
        } else {
            Self::default()
        }
    }

    pub fn choose_color(&self, rng: &mut ThreadRng) -> Color {
        let choice = rng.gen_range(0, self.0);
        self.dichotomy(choice, 0, self.1.len())
    }

    fn dichotomy(&self, target: usize, inf: usize, sup: usize) -> Color {
        if inf == sup || inf + 1 == sup {
            self.1[inf].0
        } else {
            let mid = (sup + inf) / 2;
            if self.1[mid].1 > target {
                self.dichotomy(target, inf, mid)
            } else {
                self.dichotomy(target, mid, sup)
            }
        }
    }
}
