use rand::{Rng, rngs::ThreadRng};

#[derive(Clone)]
pub struct Chooser<T: Copy>(usize, Vec<(T, usize)>);

impl<T: Copy> Chooser<T> {
    pub fn default() -> Self {
        Self(1, Vec::new())
    }

    pub fn new(mut v: Vec<(T, usize)>) -> Self {
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

    pub fn choose(&self, rng: &mut ThreadRng) -> Option<T> {
        let choice = rng.gen_range(0, self.0);
        if self.1.is_empty() {
            None
        } else {
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

    pub fn extract(&self) -> Vec<(T, usize)> {
        let mut cpy = self.1.clone();
        let n = cpy.len();
        for i in 1..n {
            cpy[n - i].1 -= cpy[n - i - 1].1;
        }
        cpy
    }

    pub fn push(&mut self, item: T, w: usize) {
        self.0 += w;
        self.1.push((item, self.0));
    }

    pub fn append(&mut self, items: Vec<(T, usize)>) {
        for (item, w) in items {
            self.push(item, w);
        }
    }
}
