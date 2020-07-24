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

impl fmt::Display for Disc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Disc {} {} {} ", self.center.0, self.center.1, self.radius)?;
        write!(f, "{}", self.color)
    }
}

impl fmt::Display for HalfPlane {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HalfPlane {} {} {} {} ", self.limit.0, self.limit.1, self.reference.0, self.reference.1)?;
        write!(f, "{}", self.color)
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Triangle {} {} ", self.a.0, self.a.1)?;
        write!(f, "{} {} ", self.b.0, self.b.1)?;
        write!(f, "{} {} ", self.c.0, self.c.1)?;
        write!(f, "{}", self.color)
    }
}

impl fmt::Display for Spiral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Spiral {} {} {} ", self.center.0, self.center.1, self.width)?;
        write!(f, "{}", self.color)
    }
}

impl fmt::Display for Stripe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stripe {} {} {} {} ", self.limit.0, self.limit.1, self.reference.0, self.reference.1)?;
        write!(f, "{}", self.color)
    }
}

impl fmt::Display for Wave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wave {} {} {} {} ", self.limit.0, self.limit.1, self.reference.0, self.reference.1)?;
        write!(f, "{}", self.color)
    }
}

impl<'a> fmt::Display for Logger<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", self.frame)?;
        write!(f, "{} ", self.bg)?;
        write!(f, "{} ", self.objects.len())?;
        for o in self.objects {
            write!(f, "{} ", o)?;
        }
        Ok(())
    }
}

