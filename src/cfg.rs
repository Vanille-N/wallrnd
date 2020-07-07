use crate::color::{Chooser, Color};
use crate::pos::{polar, radians, Pos};
use crate::scene::*;
use crate::tesselation::*;
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};
use svg::node::element::Path;

pub struct SceneCfg {
    pub theme: Chooser<Color>,
    pub weight: i32,
    pub deviation: i32,
    pub frame: Frame,
    pub pattern: Pattern,
    pub tiling: Tiling,
    pub nb_pattern: i32,
    pub var_stripes: i32,
    pub size_tiling: f64,
    pub nb_delaunay: i32,
    pub width_pattern: f64,
}

trait Dynamic<C>
where
    C: Contains + 'static,
{
    fn dynamic(self) -> Vec<Box<dyn Contains>>;
}

impl<C> Dynamic<C> for Vec<C>
where
    C: Contains + 'static,
{
    fn dynamic(self) -> Vec<Box<dyn Contains>> {
        self.into_iter()
            .map(|d| Box::new(d) as Box<dyn Contains>)
            .collect::<Vec<_>>()
    }
}

impl SceneCfg {
    pub fn choose_color(&self, rng: &mut ThreadRng) -> ColorItem {
        ColorItem {
            shade: Color::random(rng),
            deviation: self.deviation,
            weight: self.weight,
            theme: self.theme.choose(rng).unwrap_or(Color(0, 0, 0)),
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
        for i in 0..self.nb_pattern {
            let c = self.choose_color(rng);
            v.push(Disc::random(
                rng,
                &self.frame,
                c,
                i as f64 / self.nb_pattern as f64 * 0.5,
            ));
        }
        v.sort_by(|a, b| a.radius.partial_cmp(&b.radius).unwrap());
        v
    }

    fn create_free_triangles(&self, rng: &mut ThreadRng) -> Vec<Triangle> {
        let mut v = Vec::new();
        for i in 0..self.nb_pattern {
            let c = self.choose_color(rng);
            v.push(Disc::random(
                rng,
                &self.frame,
                c,
                i as f64 / self.nb_pattern as f64 * 0.7,
            ));
        }
        v.sort_by(|a, b| a.radius.partial_cmp(&b.radius).unwrap());
        v.into_iter()
            .map(|d| Triangle::random(rng, d))
            .collect::<Vec<_>>()
    }

    fn create_free_stripes(&self, rng: &mut ThreadRng) -> Vec<Stripe> {
        let mut v = Vec::new();
        for _ in 0..self.nb_pattern {
            let c = self.choose_color(rng);
            let w = self.width_pattern * self.frame.h as f64 * (rng.gen::<f64>() + 0.5);
            v.push(Stripe::random(rng, &self.frame, c, w));
        }
        v
    }

    fn create_free_spirals(&self, rng: &mut ThreadRng) -> Vec<Spiral> {
        let mut v = Vec::new();
        for _ in 0..self.nb_pattern {
            let c = self.choose_color(rng);
            let w = self.width_pattern * self.frame.h as f64 * (rng.gen::<f64>() + 0.5);
            v.push(Spiral::random(rng, &self.frame, c, w));
        }
        v.sort_by(|a, b| a.width.partial_cmp(&b.width).unwrap());
        v
    }

    fn create_concentric_circles(&self, rng: &mut ThreadRng) -> Vec<Disc> {
        let mut v = Vec::new();
        for i in 0..self.nb_pattern {
            let c = self.choose_color(rng);
            v.push(Disc::random(rng, &self.frame, c, i as f64 / 10.));
        }
        unimplemented!()
    }

    fn create_parallel_stripes(&self, rng: &mut ThreadRng) -> Vec<HalfPlane> {
        let mut v = Vec::new();
        let (a, b, dir) = {
            let c = self.frame.center();
            let w = self.frame.h + self.frame.w;
            let dir = rng.gen_range(0, 360);
            let d = polar(radians(dir), w as f64 / 2.);
            (c + d, c - d, dir)
        };
        for i in 0..self.nb_pattern {
            let c = self.choose_color(rng);
            let p = i as f64 / self.nb_pattern as f64;
            v.push(HalfPlane::random(
                rng,
                a * (1. - p) + b * p,
                180 + dir,
                self.var_stripes,
                c,
            ));
        }
        v
    }

    fn create_crossed_stripes(&self, rng: &mut ThreadRng) -> Vec<HalfPlane> {
        let mut v = Vec::new();
        for i in 0..self.nb_pattern {
            let c = self.choose_color(rng);
            v.push(Disc::random(rng, &self.frame, c, i as f64 / 10.));
        }
        unimplemented!()
    }

    pub fn make_tiling(&self, rng: &mut ThreadRng) -> Vec<(Pos, Path)> {
        use crate::tesselation::*;
        match self.tiling {
            Tiling::Hexagons => tile_hexagons(&self.frame, self.size_tiling, rng.gen_range(0, 360)),
            Tiling::Triangles => {
                tile_triangles(&self.frame, self.size_tiling, rng.gen_range(0, 360))
            }
            Tiling::HexagonsAndTriangles => {
                tile_hybrid_hexagons_triangles(&self.frame, self.size_tiling, rng.gen_range(0, 360))
            }
            Tiling::SquaresAndTriangles => {
                tile_hybrid_squares_triangles(&self.frame, self.size_tiling, rng.gen_range(0, 360))
            }
            Tiling::Delaunay => random_delaunay(&self.frame, rng, self.nb_delaunay),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Pattern {
    FreeCircles,
    FreeTriangles,
    FreeStripes,
    FreeSpirals,
    ConcentricCircles,
    ParallelStripes,
    CrossedStripes,
}

impl Pattern {
    pub fn choose(rng: &mut ThreadRng) -> Self {
        use Pattern::*;
        *vec![
            FreeCircles,
            FreeTriangles,
            FreeStripes,
            FreeSpirals,
            ConcentricCircles,
            ParallelStripes,
            CrossedStripes,
        ]
        .choose(rng)
        .unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Tiling {
    Hexagons,
    Triangles,
    HexagonsAndTriangles,
    SquaresAndTriangles,
    Delaunay,
}

impl Tiling {
    pub fn choose(rng: &mut ThreadRng) -> Self {
        use Tiling::*;
        *vec![
            Hexagons,
            Triangles,
            HexagonsAndTriangles,
            SquaresAndTriangles,
            Delaunay,
        ]
        .choose(rng)
        .unwrap()
    }
}
