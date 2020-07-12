use crate::pos::Pos;

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
