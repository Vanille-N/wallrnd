use crate::pos::*;
use crate::shape::*;
use std::collections::HashSet;
use svg::node::element::Path;

#[derive(Clone, Copy)]
pub struct Frame {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
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

pub fn tile_hexagons(f: &Frame, size: f64, rot: i32) -> Vec<Path> {
    let mut v = Vec::new();
    let center = f.center();
    let idir = polar(radians(rot - 30), (size * 2.) * radians(30).cos());
    let jdir = polar(radians(rot + 30), (size * 2.) * radians(30).cos());
    let mut set = HashSet::new();
    let mut stk = Vec::new();
    stk.push(center);
    set.insert(center);
    while !stk.is_empty() {
        let pos = stk.pop().unwrap();
        if f.is_inside(pos) {
            v.push(pos);
            for &(i, j) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let p = pos + idir * i + jdir * j;
                if !set.contains(&p) {
                    set.insert(p);
                    stk.push(p);
                }
            }
        }
    }
    let m = Movable::hexagon(size, rot);
    v.into_iter().map(|p| m.render(p)).collect::<Vec<_>>()
}

pub fn tile_triangles(f: &Frame, size: f64, rot: i32) -> Vec<Path> {
    let mut v = Vec::new();
    let center = f.center();
    let idir = polar(radians(rot - 30), (size * 2.) * radians(30).cos());
    let jdir = polar(radians(rot + 30), (size * 2.) * radians(30).cos());
    let adjust = polar(radians(rot + 60), size * radians(30).sin());
    let mut set = HashSet::new();
    let mut stk = Vec::new();
    stk.push(center);
    set.insert(center);
    while !stk.is_empty() {
        let pos = stk.pop().unwrap();
        if f.is_inside(pos) {
            v.push((pos, false));
            v.push((pos + idir * 0.5 + adjust, true));
            for &(i, j) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let p = pos + idir * i + jdir * j;
                if !set.contains(&p) {
                    set.insert(p);
                    stk.push(p);
                }
            }
        }
    }
    let m1 = Movable::triangle(size, rot);
    let m2 = Movable::triangle(size, rot + 60);
    v.into_iter()
        .map(|(p, b)| (if b { &m1 } else { &m2 }).render(p))
        .collect::<Vec<_>>()
}

pub fn tile_hybrid_hexagons_triangles(f: &Frame, size: f64, rot: i32) -> Vec<Path> {
    let mut v = Vec::new();
    let center = f.center();
    let idir = polar(radians(rot), size * 2.);
    let jdir = polar(radians(rot + 60), size * 2.);
    let adjust = polar(radians(rot + 30), size / radians(30).cos());
    let mut set = HashSet::new();
    let mut stk = Vec::new();
    stk.push(center);
    set.insert(center);
    while !stk.is_empty() {
        let pos = stk.pop().unwrap();
        if f.is_inside(pos) {
            v.push((pos, 0));
            v.push((pos + adjust, 1));
            v.push((pos - adjust, 2));
            for &(i, j) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let p = pos + idir * i + jdir * j;
                if !set.contains(&p) {
                    set.insert(p);
                    stk.push(p);
                }
            }
        }
    }
    let m = [
        Movable::hexagon(size, rot),
        Movable::triangle(size * radians(30).sin(), rot + 30),
        Movable::triangle(size * radians(30).sin(), rot + 90),
    ];
    v.into_iter()
        .map(|(p, i)| m[i].render(p))
        .collect::<Vec<_>>()
}

pub fn tile_hybrid_squares_triangles(f: &Frame, size: f64, rot: i32) -> Vec<Path> {
    let mut v = Vec::new();
    let center = f.center();
    let mut set = HashSet::new();
    let mut stk = Vec::new();
    stk.push(center);
    set.insert(center);
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
    let a = size / 2_f64.sqrt();
    let b = a * radians(30).tan();
    let c = a / radians(30).cos();
    let idir = polar(radians(rot), c + a*2. + 2.*b) + polar(radians(rot + 60), c + a*2. + 2.*b);
    let jdir = polar(radians(rot), c + a*2. + 2.*b) + polar(radians(rot - 60), c + a*2. + 2.*b);
    while !stk.is_empty() {
        let pos = stk.pop().unwrap();
        if f.is_inside(pos) {
            for i in 0..6 {
                v.push((
                    pos + polar(radians(rot + i * 60), c),
                    3 + (i as usize % 2),
                ));
                v.push((
                    pos + polar(radians(rot + i * 60), c + b + a),
                    i as usize % 3,
                ));
                v.push((
                    pos + polar(radians(rot + i * 60 + 30), 2. * a + c),
                    5 + (i as usize % 2),
                ));
            }
            v.push((pos + polar(radians(rot), c + 2. * b + 2. * a), 4));
            v.push((pos - polar(radians(rot), c + 2. * b + 2. * a), 3));
            for &(i, j) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let p = pos + idir * i + jdir * j;
                if !set.contains(&p) {
                    set.insert(p);
                    stk.push(p);
                }
            }
        }
    }
    let m = [
        Movable::square(size, rot),
        Movable::square(size, rot + 60),
        Movable::square(size, rot - 60),
        Movable::triangle(c, rot + 60),
        Movable::triangle(c, rot),
        Movable::triangle(c, rot + 90),
        Movable::triangle(c, rot + 30),
    ];
    v.into_iter()
        .map(|(p, i)| m[i].render(p))
        .collect::<Vec<_>>()
}
