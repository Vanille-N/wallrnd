use std::collections::HashSet;

use svg::node::element::Path;
use crate::shapes::*;
use crate::pos::*;

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

pub fn tile_hexagons(f: &Frame, h: Hexagon) -> Vec<Path> {
    let mut v = Vec::new();
    let center = f.center();
    let idir = polar(radians(h.rot - 30), (h.size * 2.) * radians(30).cos());
    let jdir = polar(radians(h.rot + 30), (h.size * 2.) * radians(30).cos());
    let mut set = HashSet::new();
    let mut stk = Vec::new();
    // Init
    stk.push((0, 0));
    set.insert((0, 0));
    while !stk.is_empty() {
        let pos = stk.pop().unwrap();
        let (i0, j0) = pos;
        let realpos = center + idir * i0 + jdir * j0;
        if f.is_inside(realpos) {
            v.push(realpos);
            for (i, j) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let p = (i0 + i, j0 + j);
                if !set.contains(&p) {
                    set.insert(p);
                    stk.push(p);
                }
            }
        }
    }
    let m = Movable::hexagon(h);
    v.into_iter().map(|p| m.render(p)).collect::<Vec<_>>()
}

pub fn tile_triangles(f: &Frame, t: Triangle) -> Vec<Path> {
    let mut v = Vec::new();
    let center = f.center();
    let idir = polar(radians(t.rot - 30), (t.size * 2.) * radians(30).cos());
    let jdir = polar(radians(t.rot + 30), (t.size * 2.) * radians(30).cos());
    let adjust = polar(radians(t.rot + 60), t.size * radians(30).sin());
    let mut set = HashSet::new();
    let mut stk = Vec::new();
    // Init
    stk.push((0, 0));
    set.insert((0, 0));
    while !stk.is_empty() {
        let pos = stk.pop().unwrap();
        let (i0, j0) = pos;
        let realpos = center + idir * i0 + jdir * j0;
        if f.is_inside(realpos) {
            v.push((realpos, false));
            v.push((realpos + idir * 0.5 + adjust, true));
            for (i, j) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let p = (i0 + i, j0 + j);
                if !set.contains(&p) {
                    set.insert(p);
                    stk.push(p);
                }
            }
        }
    }
    let m1 = Movable::triangle(t);
    let m2 = Movable::triangle(t.rotate(60));
    v.into_iter().map(|(p, b)| (if b { &m1 } else { &m2 }).render(p)).collect::<Vec<_>>()
}
