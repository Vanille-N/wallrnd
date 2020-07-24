use crate::prelude::*;
use crate::scene::*;
use std::fmt;
pub struct Logger<'a> {
    pub frame: Frame,
    pub bg: &'a ColorItem,
    pub objects: &'a [Box<dyn Contains>],
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}", self.x, self.y, self.w, self.h)
    }
}

impl fmt::Display for ColorItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} ", self.shade.0, self.shade.1, self.shade.2)?;
        write!(f, "{} {} {} ", self.theme.0, self.theme.1, self.theme.2)?;
        write!(f, "{} {}", self.deviation, self.weight)
    }
}
