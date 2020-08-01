use rand::{rngs::ThreadRng, Rng};

#[derive(Clone)]
pub struct Chooser<T: Clone>(usize, Vec<(T, usize)>);

impl<T: Clone> Chooser<T> {
    /// Empty Chooser
    pub fn default() -> Self {
        Self(0, Vec::new())
    }

    /// Create Chooser from weighted items
    pub fn new(mut v: Vec<(T, usize)>) -> Self {
        let mut sum = 0;
        for (_, w) in &mut v {
            sum += *w;
            *w = sum;
        }
        if !v.is_empty() {
            Self(sum, v)
        } else {
            Self::default()
        }
    }

    /// Pick a random item (weighted)
    pub fn choose(&self, rng: &mut ThreadRng) -> Option<T> {
        if self.1.is_empty() {
            None
        } else {
            let choice = rng.gen_range(0, self.0);
            Some(self.dichotomy(choice, 0, self.1.len()))
        }
    }

    fn dichotomy(&self, target: usize, inf: usize, sup: usize) -> T {
        if inf == sup {
            self.1[inf].0
        } else if inf + 1 == sup {
            if self.1[inf].1 < target {
                self.1[inf + 1].0
            } else {
                self.1[inf].0
            }
        } else {
            let mid = (sup + inf) / 2;
            if self.1[mid].1 > target {
                self.dichotomy(target, inf, mid)
            } else {
                self.dichotomy(target, mid, sup)
            }
        }
    }

    /// Get items with their weights as a copy
    pub fn extract(&self) -> Vec<(T, usize)> {
        let mut cpy = self.1.clone();
        let n = cpy.len();
        for i in 1..n {
            cpy[n - i].1 -= cpy[n - i - 1].1;
        }
        cpy
    }

    /// Add new item
    pub fn push(&mut self, item: T, w: usize) {
        if w == 0 {
            panic!("In call to Chooser::push, 0 is not a valid weight");
        }
        self.0 += w;
        self.1.push((item, self.0));
    }

    /// Add vector of new items
    pub fn append(&mut self, items: Vec<(T, usize)>) {
        for (item, w) in items {
            self.push(item, w);
        }
    }
}
