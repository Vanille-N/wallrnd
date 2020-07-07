use crate::color::Color;
use crate::tesselation::*;
use crate::scene::*;
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};
use crate::pos::{radians, polar, Pos};

pub struct SceneCfg {
    pub themes: Vec<Color>,
    pub weight: i32,
    pub deviation: i32,
    pub frame: Frame,
    pub pattern: Pattern,
    pub tiling: Tiling,
    pub nb_free_circles: i32,
    pub nb_free_spirals: i32,
    pub nb_free_stripes: i32,
    pub nb_free_triangles: i32,
    pub nb_crossed_stripes: i32,
    pub nb_parallel_stripes: i32,
    pub nb_concentric_circles: i32,
    pub var_parallel_stripes: i32,
}

trait Dynamic<C>
where C: Contains + 'static
{
    fn dynamic(self) -> Vec<Box<dyn Contains>>;
}

impl<C> Dynamic<C> for Vec<C>
where C: Contains + 'static
{
    fn dynamic(self) -> Vec<Box<dyn Contains>> {
        self.into_iter().map(|d| Box::new(d) as Box<dyn Contains>).collect::<Vec<_>>()
    }
}

impl SceneCfg {
    pub fn choose_color(&self, rng: &mut ThreadRng) -> ColorItem {
        ColorItem {
            shade: Color::random(rng),
            deviation: self.deviation,
            weight: self.weight,
            theme: *self.themes.choose(rng).unwrap(),
        }
    }

    pub fn create_items(&self, rng: &mut ThreadRng) -> Vec<Box<dyn Contains>> {
        match self.pattern {
            Pattern::FreeCircles => self.create_free_circles(rng).dynamic(),
            Pattern::FreeTriangles => self.create_free_triangles(rng).dynamic(),
            Pattern::FreeStripes => self.create_free_stripes(rng).dynamic(),
            Pattern::FreeSpirals => self.create_free_spirals(rng).dynamic(),
            Pattern::ConcentricCircles => self.create_concentric_circles(rng).dynamic(),
            Pattern::ParallelStripes => self.create_parallel_stripes(rng).dynamic(),
            Pattern::CrossedStripes => self.create_crossed_stripes(rng).dynamic(),
        }
    }

    fn create_free_circles(&self, rng: &mut ThreadRng) -> Vec<Disc> {
        let mut v = Vec::new();
        for i in 0..self.nb_free_circles {
            let c = self.choose_color(rng);
            v.push(Disc::random(rng, &self.frame, c, i as f64/self.nb_free_circles as f64 * 0.5));
        }
        v.sort_by(|a, b| a.radius.partial_cmp(&b.radius).unwrap());
        v
    }

    fn create_free_triangles(&self, rng: &mut ThreadRng) -> Vec<Triangle> {
        let mut v = Vec::new();
        for i in 0..self.nb_free_triangles {
            let c = self.choose_color(rng);
            v.push(Disc::random(rng, &self.frame, c, i as f64 / self.nb_free_triangles as f64 * 0.7));
        }
        v.sort_by(|a, b| b.radius.partial_cmp(&a.radius).unwrap());
        v.into_iter().map(|d| Triangle::random(rng, d)).collect::<Vec<_>>()
    }

    fn create_free_stripes(&self, rng: &mut ThreadRng) -> Vec<Stripe> {
        let mut v = Vec::new();
        for i in 0..self.nb_free_stripes {
            let c = self.choose_color(rng);
            v.push(Disc::random(rng, &self.frame, c, i as f64/10.));
        }
        unimplemented!()
    }

    fn create_free_spirals(&self, rng: &mut ThreadRng) -> Vec<Spiral> {
        let mut v = Vec::new();
        for i in 0..self.nb_free_spirals {
            let c = self.choose_color(rng);
            v.push(Disc::random(rng, &self.frame, c, i as f64/10.));
        }
        unimplemented!()
    }

    fn create_concentric_circles(&self, rng: &mut ThreadRng) -> Vec<Disc> {
        let mut v = Vec::new();
        for i in 0..self.nb_concentric_circles {
            let c = self.choose_color(rng);
            v.push(Disc::random(rng, &self.frame, c, i as f64/10.));
        }
        unimplemented!()
    }

    fn create_parallel_stripes(&self, rng: &mut ThreadRng) -> Vec<HalfPlane> {
        let mut v = Vec::new();
        let (a, b) = {
            let c = Pos::random(&self.frame, rng);
            let w = self.frame.h + self.frame.w;
            let d = polar(radians(rng.gen_range(0, 360)), w as f64);
            (c + d, c - d)
        };
        for i in 0..self.nb_parallel_stripes {
            let c = self.choose_color(rng);
            let p = i as f64 / self.nb_parallel_stripes as f64;
            v.push(HalfPlane::random(rng, a * (1. - p) + b * p, b, self.var_parallel_stripes, c));
        }
        v
    }

    fn create_crossed_stripes(&self, rng: &mut ThreadRng) -> Vec<HalfPlane> {
        let mut v = Vec::new();
        for i in 0..self.nb_crossed_stripes {
            let c = self.choose_color(rng);
            v.push(Disc::random(rng, &self.frame, c, i as f64/10.));
        }
        unimplemented!()
    }
}

pub enum Pattern {
    FreeCircles,
    FreeTriangles,
    FreeStripes,
    FreeSpirals,
    ConcentricCircles,
    ParallelStripes,
    CrossedStripes,
}

pub enum Tiling {
    Squares,
    Hexagons,
    HexagonsAndTriangles,
    SquaresAndTriangles,
    Delaunay,
}
