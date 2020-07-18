use rand::{rngs::ThreadRng, Rng};
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Color(pub i32, pub i32, pub i32);

impl Color {
    /// Ensure that all RGB values are within [[1; 100]]
    fn validate(mut self) -> Self {
        self.0 = self.0.min(255).max(0);
        self.1 = self.1.min(255).max(0);
        self.2 = self.2.min(255).max(0);
        self
    }

    /// Random noise
    pub fn variate(mut self, rng: &mut ThreadRng, amount: i32) -> Self {
        self.0 += rng.gen_range(-amount, amount);
        self.1 += rng.gen_range(-amount, amount);
        self.2 += rng.gen_range(-amount, amount);
        self
    }

    /// Weighted mix with other color
    pub fn meanpoint(mut self, th: Self, weight: i32) -> Self {
        self.0 = (self.0 * weight + th.0 * (100 - weight)) / 100;
        self.1 = (self.1 * weight + th.1 * (100 - weight)) / 100;
        self.2 = (self.2 * weight + th.2 * (100 - weight)) / 100;
        self
    }

    /// Generate color
    pub fn random(rng: &mut ThreadRng) -> Self {
        Self(
            rng.gen_range(0, 255),
            rng.gen_range(0, 255),
            rng.gen_range(0, 255),
        )
    }
}

/// SVG color format: `rgb(<r>,<g>,<b>)`
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = self.validate();
        write!(f, "rgb({},{},{})", c.0, c.1, c.2)
    }
}
