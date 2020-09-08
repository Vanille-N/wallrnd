use crate::prelude::*;
use crate::shape::*;
use crate::svg::*;
use delaunator as del;
use rand::rngs::ThreadRng;
use std::collections::HashSet;

macro_rules! set {
    { $( $elem:expr ),* } => {
        {
            let mut set = HashSet::new();
            $( set.insert($elem); )*
            set
        }
    }
}

/// Tile the plane with a pattern that can be mapped to a 2D grid.
/// This criterion applies to all tilings used here except Delaunay triangulation.
fn periodic_grid_tiling<F>(f: &Frame, gen: F, idir: Pos, jdir: Pos) -> Vec<(Pos, Path)>
where
    F: Fn(Pos) -> Vec<(Pos, Path)>,
{
    let mut items = Vec::new();
    let center = f.center();
    let mut set = set![center];
    let mut stk = vec![center];
    while let Some(pos) = stk.pop() {
        if f.is_inside(pos) {
            for item in gen(pos) {
                items.push(item);
            }
            for &(i, j) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let p = pos + idir * i + jdir * j;
                if !set.contains(&p) {
                    set.insert(p);
                    stk.push(p);
                }
            }
        }
    }
    items
}

pub fn tile_hexagons(f: &Frame, size: f64, rot: isize) -> Vec<(Pos, Path)> {
    let idir = Pos::polar(rot - 30, (size * 2.) * radians(30).cos());
    let jdir = Pos::polar(rot + 30, (size * 2.) * radians(30).cos());
    let m = Movable::hexagon(size, rot);
    periodic_grid_tiling(f, |p| vec![m.render(p)], idir, jdir)
}

pub fn tile_triangles(f: &Frame, size: f64, rot: isize) -> Vec<(Pos, Path)> {
    let idir = Pos::polar(rot - 30, (size * 2.) * radians(30).cos());
    let jdir = Pos::polar(rot + 30, (size * 2.) * radians(30).cos());
    let adjust = Pos::polar(rot + 60, size * radians(30).sin()) + idir * 0.5;
    let m1 = Movable::triangle(size, rot + 60);
    let m2 = Movable::triangle(size, rot);
    periodic_grid_tiling(f, |p| vec![m1.render(p), m2.render(p + adjust)], idir, jdir)
}

pub fn tile_hybrid_hexagons_triangles(f: &Frame, size: f64, rot: isize) -> Vec<(Pos, Path)> {
    let idir = Pos::polar(rot, size * 2.);
    let jdir = Pos::polar(rot + 60, size * 2.);
    let adjust = Pos::polar(rot + 30, size / radians(30).cos());
    let m = [
        Movable::hexagon(size, rot),
        Movable::triangle(size * radians(30).sin(), rot + 30),
        Movable::triangle(size * radians(30).sin(), rot + 90),
    ];
    periodic_grid_tiling(
        f,
        |p| {
            vec![
                m[0].render(p),
                m[1].render(p + adjust),
                m[2].render(p - adjust),
            ]
        },
        idir,
        jdir,
    )
}

pub fn tile_hybrid_squares_triangles(f: &Frame, size: f64, rot: isize) -> Vec<(Pos, Path)> {
    let a = size / 2_f64.sqrt();
    let b = a * radians(30).tan();
    let c = a / radians(30).cos();
    //
    //  +---------------+,
    //  |            ,' |,'-,
    //  |          x'   | 'c '-,
    //  |        ,'     |   ',  '-,
    //  |       +---a---|--b--+    :-
    //  |               |       ,-'
    //  |               |    ,-'
    //  |               | ,-'
    //  +---------------+'
    //
    let idir = Pos::polar(rot, c + a * 2. + 2. * b) + Pos::polar(rot + 60, c + a * 2. + 2. * b);
    let jdir = Pos::polar(rot, c + a * 2. + 2. * b) + Pos::polar(rot - 60, c + a * 2. + 2. * b);
    let mv = [
        Movable::square(size, rot),
        Movable::square(size, rot + 60),
        Movable::square(size, rot - 60),
        Movable::triangle(c, rot + 60),
        Movable::triangle(c, rot),
        Movable::triangle(c, rot + 90),
        Movable::triangle(c, rot + 30),
    ];
    periodic_grid_tiling(
        f,
        |pos| {
            let mut items = vec![
                mv[4].render(pos + Pos::polar(rot, c + 2. * b + 2. * a)),
                mv[3].render(pos - Pos::polar(rot, c + 2. * b + 2. * a)),
            ];
            for i in 0..6 {
                items.push(mv[3 + (i as usize % 2)].render(pos + Pos::polar(rot + i * 60, c)));
                items.push(mv[i as usize % 3].render(pos + Pos::polar(rot + i * 60, c + b + a)));
                items.push(
                    mv[5 + (i as usize % 2)]
                        .render(pos + Pos::polar(rot + i * 60 + 30, 2. * a + c)),
                );
            }
            items
        },
        idir,
        jdir,
    )
}

pub fn tile_rhombus(f: &Frame, ldiag: f64, sdiag: f64, rot: isize) -> Vec<(Pos, Path)> {
    let idir = Pos::polar(rot, ldiag) + Pos::polar(rot + 90, sdiag);
    let jdir = Pos::polar(rot, -ldiag) + Pos::polar(rot + 90, sdiag);
    let m = Movable::rhombus(ldiag, sdiag, rot);
    periodic_grid_tiling(f, |p| vec![m.render(p)], idir, jdir)
}

/// External crate does the heavy lifting and is an order of magnitude faster than the previously implemented Boyer-Watson algorithm.
/// Only downside is that it requires conversions between position types.
fn fast_triangulate(pts: &[Pos]) -> Vec<(Pos, Pos, Pos)> {
    let points = pts
        .iter()
        .map(|&Pos(x, y)| del::Point { x, y })
        .collect::<Vec<_>>();
    let result = del::triangulate(&points)
        .unwrap()
        .triangles
        .iter()
        .map(|&i| pts[i])
        .collect::<Vec<_>>();
    let mut v = Vec::new();
    for i in 0..result.len() / 3 {
        v.push((result[i * 3], result[i * 3 + 1], result[i * 3 + 2]));
    }
    v
}

pub fn random_delaunay(f: &Frame, rng: &mut ThreadRng, n: usize) -> Vec<(Pos, Path)> {
    let mut pts = Vec::new();
    for _ in 0..n {
        pts.push(Pos::random(f, rng));
    }
    let triangulation = fast_triangulate(&pts);
    triangulation
        .into_iter()
        .map(|(a, b, c)| {
            (
                (a + b + c) * 0.33,
                Path::new(Data::new(a).with_line_to(b).with_line_to(c)),
            )
        })
        .collect::<Vec<_>>()
}

pub fn pentagons_type1(f: &Frame, size: f64, rot: isize) -> Vec<(Pos, Path)> {
    let beta = 80;
    let gamma = 180 - beta;
    let alpha = 110;
    let delta = 110;
    let epsilon = 360 - alpha - delta;
    let sizes = [size * 1.2, size, size * 0.9];
    let angles = [alpha, beta, gamma, delta, epsilon];
    #[rustfmt::skip]
    let mv = [
        Pentagon { sizes, rot, angles }.to_movable(),
        Pentagon { sizes, rot: rot + 180, angles }.to_movable(),
    ];
    let idir = mv[0].vertex(3) - mv[0].vertex(0);
    let jdir = mv[0].vertex(0) - mv[1].vertex(4) + mv[1].vertex(2) - mv[0].vertex(1);
    periodic_grid_tiling(
        f,
        |pos| {
            vec![
                mv[0].render(pos - mv[0].vertex(0) + mv[0].side(4) * 0.5),
                mv[1].render(pos - mv[1].vertex(0) + mv[1].side(4) * 0.5),
            ]
        },
        idir,
        jdir,
    )
}

pub fn pentagons_type2(f: &Frame, size: f64, rot: isize) -> Vec<(Pos, Path)> {
    let alpha = 110;
    let beta = 110;
    let gamma = 100;
    let delta = 180 - beta;
    let epsilon = 150;
    let sizes = [size, size * 1.2, size];
    let angles = [epsilon, delta, gamma, beta, alpha];
    let rangles = [beta, gamma, delta, epsilon, alpha];
    #[rustfmt::skip]
    let mv = [
        Pentagon { sizes, rot, angles }.to_movable(),
        Pentagon { sizes, rot: rot + 180, angles }.to_movable(),
        Pentagon { sizes, rot: rot + 180, angles: rangles }.to_movable(),
        Pentagon { sizes, rot, angles: rangles }.to_movable(),
    ];
    let idir = mv[0].vertex(0) - mv[2].vertex(1) + mv[2].vertex(2) - mv[0].vertex(3);
    let jdir = mv[0].vertex(4) - mv[3].vertex(1) + mv[3].vertex(4) - mv[0].vertex(2);
    periodic_grid_tiling(
        f,
        |pos| {
            vec![
                mv[0].render(pos),
                mv[1].render(pos + mv[0].vertex(2) + mv[3].side(4) - mv[1].vertex(1)),
                mv[2].render(pos + mv[0].vertex(0) - mv[2].vertex(1)),
                mv[3].render(pos + mv[0].vertex(2) - mv[3].vertex(4)),
            ]
        },
        idir,
        jdir,
    )
}

pub fn pentagons_type3(f: &Frame, size: f64, rot: isize) -> Vec<(Pos, Path)> {
    let alpha = 120;
    let gamma = alpha;
    let delta = alpha;
    let beta = 77;
    let epsilon = 180 - beta;
    let p = 0.3;
    let sizes = [size * p, size, size * (1. - p)];
    let angles = [epsilon, delta, gamma, beta, alpha];
    #[rustfmt::skip]
    let mv = [
        Pentagon { sizes, rot, angles }.to_movable(),
        Pentagon { sizes, rot: rot + 120, angles }.to_movable(),
        Pentagon { sizes, rot: rot - 120, angles }.to_movable(),
    ];
    let idir = -mv[0].vertex(4) + mv[0].vertex(2) - mv[2].vertex(2) + mv[2].vertex(4);
    let jdir = -mv[0].vertex(4) + mv[0].vertex(2) - mv[1].vertex(2) + mv[1].vertex(4);
    periodic_grid_tiling(
        f,
        |pos| {
            vec![
                mv[0].render(pos - mv[0].vertex(4)),
                mv[1].render(pos - mv[1].vertex(4)),
                mv[2].render(pos - mv[2].vertex(4)),
            ]
        },
        idir,
        jdir,
    )
}

pub fn pentagons_type4(f: &Frame, size: f64, rot: isize) -> Vec<(Pos, Path)> {
    let alpha = 100;
    let beta = 90;
    let gamma = 130;
    let delta = 90;
    let epsilon = 130;
    let sizes = [size, size, size * 0.9];
    let angles = [alpha, beta, gamma, delta, epsilon];
    #[rustfmt::skip]
    let mv = [
        Pentagon { sizes, rot, angles }.to_movable(),
        Pentagon { sizes, rot: rot + 90, angles }.to_movable(),
        Pentagon { sizes, rot: rot + 180, angles }.to_movable(),
        Pentagon { sizes, rot: rot - 90, angles }.to_movable(),
    ];
    let idir = Pos::polar(0, size * 100.);
    let jdir = Pos::polar(90, size * 100.);
    periodic_grid_tiling(
        f,
        |pos| {
            vec![
                mv[0].render(pos - mv[0].vertex(1)),
                mv[1].render(pos - mv[1].vertex(1)),
                mv[2].render(pos - mv[2].vertex(1)),
                mv[3].render(pos - mv[3].vertex(1)),
            ]
        },
        idir,
        jdir,
    )
}

pub fn pentagons_type5(f: &Frame, size: f64, rot: isize) -> Vec<(Pos, Path)> {
    let alpha = 150;
    let beta = 60;
    let gamma = 120;
    let delta = 90;
    let epsilon = 120;
    let sizes = [size, size, size];
    let angles = [delta, gamma, beta, alpha, epsilon];
    #[rustfmt::skip]
    let mv = [
        Pentagon { sizes, rot, angles }.to_movable(),
        Pentagon { sizes, rot: rot + 60, angles }.to_movable(),
        Pentagon { sizes, rot: rot + 120, angles }.to_movable(),
        Pentagon { sizes, rot: rot + 180, angles }.to_movable(),
        Pentagon { sizes, rot: rot - 120, angles }.to_movable(),
        Pentagon { sizes, rot: rot - 60, angles }.to_movable(),
    ];
    let idir = -mv[1].vertex(2) + mv[1].vertex(4) - mv[5].vertex(4) + mv[5].vertex(1)
        - mv[1].vertex(1)
        + mv[1].vertex(4)
        - mv[3].vertex(4)
        + mv[3].vertex(2);
    let jdir = -mv[1].vertex(2) + mv[1].vertex(4) - mv[5].vertex(4) + mv[5].vertex(1)
        - mv[3].vertex(1)
        + mv[3].vertex(4)
        - mv[5].vertex(4)
        + mv[5].vertex(2);
    periodic_grid_tiling(
        f,
        |pos| {
            vec![
                mv[0].render(pos - mv[0].vertex(2)),
                mv[1].render(pos - mv[1].vertex(2)),
                mv[2].render(pos - mv[2].vertex(2)),
                mv[3].render(pos - mv[3].vertex(2)),
                mv[4].render(pos - mv[4].vertex(2)),
                mv[5].render(pos - mv[5].vertex(2)),
                mv[2].render(pos - mv[0].vertex(2) + mv[0].vertex(4) - mv[2].vertex(4)),
                mv[3].render(pos - mv[1].vertex(2) + mv[1].vertex(4) - mv[3].vertex(4)),
                mv[4].render(pos - mv[2].vertex(2) + mv[2].vertex(4) - mv[4].vertex(4)),
                mv[5].render(pos - mv[3].vertex(2) + mv[3].vertex(4) - mv[5].vertex(4)),
                mv[0].render(pos - mv[4].vertex(2) + mv[4].vertex(4) - mv[0].vertex(4)),
                mv[1].render(pos - mv[5].vertex(2) + mv[5].vertex(4) - mv[1].vertex(4)),
                mv[4].render(pos - mv[0].vertex(2) + mv[0].vertex(4) - mv[4].vertex(4)),
                mv[5].render(pos - mv[1].vertex(2) + mv[1].vertex(4) - mv[5].vertex(4)),
                mv[0].render(pos - mv[2].vertex(2) + mv[2].vertex(4) - mv[0].vertex(4)),
                mv[1].render(pos - mv[3].vertex(2) + mv[3].vertex(4) - mv[1].vertex(4)),
                mv[2].render(pos - mv[4].vertex(2) + mv[4].vertex(4) - mv[2].vertex(4)),
                mv[3].render(pos - mv[5].vertex(2) + mv[5].vertex(4) - mv[3].vertex(4)),
            ]
        },
        idir,
        jdir,
    )
}

struct Pentagon {
    rot: isize,
    sizes: [f64; 3],
    angles: [usize; 5],
}

impl Pentagon {
    fn to_movable(&self) -> Movable {
        assert_eq!(0, {
            let mut s = 0;
            for i in 0..5 {
                s += 180 - self.angles[i];
            }
            s.rem_euclid(360)
        });
        let mut pts = Vec::new();
        pts.push(Pos::zero());
        let mut running_angle = self.rot;
        for i in 0..=2 {
            let latest = pts[i];
            pts.push(latest + Pos::polar(running_angle, self.sizes[i]));
            running_angle += 180 - self.angles[i + 1] as isize;
        }
        let latest = pts[3];
        pts.push(Pos::intersect(
            (Pos::zero(), self.rot + self.angles[0] as isize),
            (latest, running_angle),
        ));
        let mid = pts.iter().fold(Pos::zero(), |acc, item| acc + *item) * 0.2;
        Movable::from(pts.into_iter().map(|p| p - mid).collect::<Vec<_>>())
    }
}
