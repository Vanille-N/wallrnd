use crate::paint::*;
use crate::prelude::*;
use crate::scene::*;
use crate::tesselate::*;
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};
use crate::svg::*;

/// General information on a scene
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
    pub line_width: f64,
    pub line_color: Color,
}

/// A trait to box scene items and make them generic.
/// Spares us from a few lines of repeated code.
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
    /// Select a random color for a scene item.
    /// The actual color will depend on the Chooser<Color> with which it is mixed.
    pub fn choose_color(&self, rng: &mut ThreadRng) -> ColorItem {
        ColorItem {
            shade: Color::random(rng),
            deviation: self.deviation,
            weight: self.weight,
            theme: self.theme.choose(rng).unwrap_or(Color(0, 0, 0)),
        }
    }

    /// Match pattern to function that generates it
    pub fn create_items(&self, rng: &mut ThreadRng) -> Vec<Box<dyn Contains>> {
        match self.pattern {
            Pattern::FreeCircles => create_free_circles(rng, &self).dynamic(),
            Pattern::FreeTriangles => create_free_triangles(rng, &self).dynamic(),
            Pattern::FreeStripes => create_free_stripes(rng, &self).dynamic(),
            Pattern::FreeSpirals => create_free_spirals(rng, &self).dynamic(),
            Pattern::ConcentricCircles => create_concentric_circles(rng, &self).dynamic(),
            Pattern::ParallelStripes => create_parallel_stripes(rng, &self).dynamic(),
            Pattern::CrossedStripes => create_crossed_stripes(rng, &self).dynamic(),
            Pattern::ParallelWaves => create_waves(rng, &self).dynamic(),
        }
    }

    /// Math tiling to function that generates it
    pub fn make_tiling(&self, rng: &mut ThreadRng) -> Vec<(Pos, Path)> {
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

/// Available patterns, open to additions
#[derive(Debug, Clone, Copy)]
pub enum Pattern {
    FreeCircles,
    FreeTriangles,
    FreeStripes,
    FreeSpirals,
    ConcentricCircles,
    ParallelStripes,
    CrossedStripes,
    ParallelWaves,
}

impl Pattern {
    /// Pick a random pattern (fallback if no other pattern choosing method is specified)
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
            ParallelWaves,
        ]
        .choose(rng)
        .unwrap()
    }
}

///Available tilings, open to additions
#[derive(Debug, Clone, Copy)]
pub enum Tiling {
    Hexagons,
    Triangles,
    HexagonsAndTriangles,
    SquaresAndTriangles,
    Delaunay,
}

impl Tiling {
    /// Pick a random tiling (fallback if no other tiling choosing method is specified)
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
