use rand::{rngs::ThreadRng, Rng};

struct SaltItem {
    color: Color,
    likeliness: f64,
    variability: usize,
}

pub struct Salt(Vec<SaltItem>);

impl SaltItem {
    fn sample(&self, rng: &mut ThreadRng) -> Option<Color> {
        if rng.gen::<f64>() < self.likeliness {
            Some(self.color.variate(self.variability))
        } else {
            None
        }
    }
}

impl Salt {
    pub fn sample(&self, rng: &mut ThreadRng) -> Option<Color> {
        for item in self.0.iter() {
            if let Some(c) = item.sample(rng) {
                return Some(c);
            }
        }
        None
    }
}
