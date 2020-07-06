use crate::pos::*;
use crate::shape::*;
use std::collections::{HashSet, HashMap};
use svg::node::element::{Path, path::Data};
use rand::rngs::ThreadRng;

macro_rules! set {
    { $( $elem:expr ),* } => {
        {
            let mut set = HashSet::new();
            $( set.insert($elem); )*
            set
        }
    }
}

#[derive(Clone, Copy)]
pub struct Frame {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

fn periodic_grid_tiling<F>(f: &Frame, gen: F, idir: Pos, jdir: Pos) -> Vec<(Pos, Path)>
where
    F: Fn(Pos) -> Vec<(Pos, Path)>,
{
    let mut v = Vec::new();
    let center = f.center();
    let mut set = set![center];
    let mut stk = vec![center];
    while let Some(pos) = stk.pop() {
        if f.is_inside(pos) {
            for item in gen(pos) {
                v.push(item);
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
    v
}

impl Frame {
    pub fn into_tuple(self) -> (usize, usize, usize, usize) {
        (self.x, self.y, self.x + self.w, self.y + self.h)
    }

    pub fn center(&self) -> Pos {
        Pos((self.x + self.w / 2) as f64, (self.y + self.h / 2) as f64)
    }

    pub fn is_inside(&self, pos: Pos) -> bool {
        let xerr = (self.w as f64) / 10.;
        let yerr = (self.h as f64) / 10.;
        (self.x as f64 - xerr) < pos.0
            && pos.0 < (self.x + self.w) as f64 + xerr
            && (self.y as f64 - yerr) < pos.1
            && pos.1 < (self.y + self.h) as f64 + yerr
    }
}

pub fn tile_hexagons(f: &Frame, size: f64, rot: i32) -> Vec<(Pos, Path)> {
    let idir = polar(radians(rot - 30), (size * 2.) * radians(30).cos());
    let jdir = polar(radians(rot + 30), (size * 2.) * radians(30).cos());
    let m = Movable::hexagon(size, rot);
    periodic_grid_tiling(f, |p| vec![m.render(p)], idir, jdir)
}

pub fn tile_triangles(f: &Frame, size: f64, rot: i32) -> Vec<(Pos, Path)> {
    let idir = polar(radians(rot - 30), (size * 2.) * radians(30).cos());
    let jdir = polar(radians(rot + 30), (size * 2.) * radians(30).cos());
    let adjust = polar(radians(rot + 60), size * radians(30).sin()) + idir * 0.5;
    let m1 = Movable::triangle(size, rot + 60);
    let m2 = Movable::triangle(size, rot);
    periodic_grid_tiling(f, |p| vec![m1.render(p), m2.render(p + adjust)], idir, jdir)
}

pub fn tile_hybrid_hexagons_triangles(f: &Frame, size: f64, rot: i32) -> Vec<(Pos, Path)> {
    let idir = polar(radians(rot), size * 2.);
    let jdir = polar(radians(rot + 60), size * 2.);
    let adjust = polar(radians(rot + 30), size / radians(30).cos());
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
        polar(radians(rot), c + a * 2. + 2. * b) + polar(radians(rot + 60), c + a * 2. + 2. * b);
    let jdir =
        polar(radians(rot), c + a * 2. + 2. * b) + polar(radians(rot - 60), c + a * 2. + 2. * b);
    let m = [
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
        |p| {
            let mut v = vec![
                m[4].render(p + polar(radians(rot), c + 2. * b + 2. * a)),
                m[3].render(p - polar(radians(rot), c + 2. * b + 2. * a)),
            ];
            for i in 0..6 {
                v.push(m[3 + (i as usize % 2)].render(p + polar(radians(rot + i * 60), c)));
                v.push(m[i as usize % 3].render(p + polar(radians(rot + i * 60), c + b + a)));
                v.push(
                    m[5 + (i as usize % 2)]
                        .render(p + polar(radians(rot + i * 60 + 30), 2. * a + c)),
                );
            }
            v
        },
        idir,
        jdir,
    )
}

fn boyer_watson(pts: &[Pos]) -> Vec<[Pos]> {
fn circumcircle((pa, pb, pc): (Pos, Pos, Pos)) -> (Pos, f64) {
    let Pos(a, b) = pb - pa;
    let Pos(c, d) = pc - pa;
    let e = a * (pa.0 + pb.0) + b * (pa.1 + pb.1);
    let f = c * (pa.0 + pc.0) + d * (pa.1 + pc.1);
    let g = 2. * (a * (pc.1 - pb.1) - b * (pc.0 - pb.0));
    if g.abs() < 0.00001 {
        let minx = pa.0.min(pb.0).min(pc.0);
        let miny = pa.1.min(pb.1).min(pc.1);
        let dx = (pa.0.max(pb.0).max(pc.0) - minx) * 0.5;
        let dy = (pa.1.max(pb.1).max(pc.1) - miny) * 0.5;
        (Pos(minx + dx, miny + dy), dx.powi(2) + dy.powi(2))
    } else {
        let x = (d * e - b * f) / g;
        let y = (a * f - c * e) / g;
        let dx = x - pa.0;
        let dy = y - pa.1;
        (Pos(x, y), dx.powi(2) + dy.powi(2))
    }
}

fn inside_circle((c, r): (Pos, f64), pt: Pos) -> bool {
    (c - pt).dot_self() < r
}

fn encompass_triangle(pts: &[Pos]) -> (Pos, Pos, Pos) {
    let Pos(mut minx, mut miny) = pts[0];
    let Pos(mut maxx, mut maxy) = pts[0];
    for &Pos(x, y) in pts.iter().skip(1) {
        minx = minx.min(x);
        miny = miny.min(y);
        maxx = maxx.max(x);
        maxy = maxy.max(y);
    }
    let a = Pos((minx + maxx)/2., maxy + (maxy - miny) * 2.);
    let b = Pos(maxx + (maxx - minx) * 2., miny - (maxy - miny) * 2.);
    let c = Pos(minx - (maxx - minx) * 2., miny - (maxy - miny) * 2.);
    (a, b, c)
}

    let super_triangle = encompass_triangle(pts);
    let mut triangulation = set![(super_triangle, circumcircle(super_triangle))];
    for pt in pts {
        let mut bad_triangles = vec![];
        for item in &triangulation {
            let (triangle, circumcircle) = item;
            if circumcircle.contains(pt) {
                bad_triangles.push(item);
            }
        }
        let bad_edges = set![];
        for item in &bad_triangles {
            let ((a, b, c), _) = *item;
            for (x, y) in &[(a, b), (a, c), (b, c)] {
                bad_edges.insert((x, y));
                bad_edges.insert((y, x));
            }
        }
        let polygon = vec![];
        for item in bad_triangles {
            let ((a, b, c), _) = *item;
            for edge in &[(a, b), (a, c), (b, c)] {
                if !bad_edges.contains(edge) {
                    polygon.push(edge);
                }
            }
            triangulation.remove(item);
        }
        for (x, y) in polygon {
            triangulation.insert(((x, y, z), circumcircle((x, y, z))));
        }
    }
    triangulation.iter().map(|(t, _)| t).collect::<Vec<_>>()
}
