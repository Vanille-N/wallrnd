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

    pub fn theme(mut self, th: Self) -> Self {
        self.0 = (self.0 + th.0) / 2;
        self.1 = (self.1 + th.1) / 2;
        self.2 = (self.2 + th.2) / 2;
        self
    }
}
