use crate::pos::*;
use crate::shape::*;
use crate::frame::Frame;
use rand::rngs::ThreadRng;
use std::collections::HashSet;
use svg::node::element::{path::Data, Path};
use delaunator as del;

macro_rules! set {
    { $( $elem:expr ),* } => {
        {
            let mut set = HashSet::new();
            $( set.insert($elem); )*
            set
        }
    }
}

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

pub fn tile_hexagons(f: &Frame, size: f64, rot: i32) -> Vec<(Pos, Path)> {
    let idir = polar(rot - 30, (size * 2.) * radians(30).cos());
    let jdir = polar(rot + 30, (size * 2.) * radians(30).cos());
    let m = Movable::hexagon(size, rot);
    periodic_grid_tiling(f, |p| vec![m.render(p)], idir, jdir)
}

pub fn tile_triangles(f: &Frame, size: f64, rot: i32) -> Vec<(Pos, Path)> {
    let idir = polar(rot - 30, (size * 2.) * radians(30).cos());
    let jdir = polar(rot + 30, (size * 2.) * radians(30).cos());
    let adjust = polar(rot + 60, size * radians(30).sin()) + idir * 0.5;
    let m1 = Movable::triangle(size, rot + 60);
    let m2 = Movable::triangle(size, rot);
    periodic_grid_tiling(f, |p| vec![m1.render(p), m2.render(p + adjust)], idir, jdir)
}

pub fn tile_hybrid_hexagons_triangles(f: &Frame, size: f64, rot: i32) -> Vec<(Pos, Path)> {
    let idir = polar(rot, size * 2.);
    let jdir = polar(rot + 60, size * 2.);
    let adjust = polar(rot + 30, size / radians(30).cos());
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

pub fn tile_hybrid_squares_triangles(f: &Frame, size: f64, rot: i32) -> Vec<(Pos, Path)> {
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
    let idir =
        polar(rot, c + a * 2. + 2. * b) + polar(rot + 60, c + a * 2. + 2. * b);
    let jdir =
        polar(rot, c + a * 2. + 2. * b) + polar(rot - 60, c + a * 2. + 2. * b);
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
                mv[4].render(pos + polar(rot, c + 2. * b + 2. * a)),
                mv[3].render(pos - polar(rot, c + 2. * b + 2. * a)),
            ];
            for i in 0..6 {
                items.push(mv[3 + (i as usize % 2)].render(pos + polar(rot + i * 60, c)));
                items.push(mv[i as usize % 3].render(pos + polar(rot + i * 60, c + b + a)));
                items.push(
                    mv[5 + (i as usize % 2)]
                        .render(pos + polar(rot + i * 60 + 30, 2. * a + c)),
                );
            }
            items
        },
        idir,
        jdir,
    )
}

fn fast_triangulate(pts: &[Pos]) -> Vec<(Pos, Pos, Pos)> {
    let points = pts.iter().map(|&Pos(x, y)| del::Point { x, y }).collect::<Vec<_>>();
    let result = del::triangulate(&points).unwrap().triangles.iter().map(|&i| pts[i]).collect::<Vec<_>>();
    let mut v = Vec::new();
    for i in 0..result.len()/3 {
        v.push((result[i*3], result[i*3+1], result[i*3+2]));
    }
    v
}

pub fn random_delaunay(f: &Frame, rng: &mut ThreadRng, n: i32) -> Vec<(Pos, Path)> {
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
                Path::new().set("stroke-width", 1).set(
                    "d",
                    Data::new()
                        .move_to(a.into_tuple())
                        .line_to(b.into_tuple())
                        .line_to(c.into_tuple())
                        .close(),
                ),
            )
        })
        .collect::<Vec<_>>()
}
