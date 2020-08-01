use crate::prelude::*;
use crate::scene::*;
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use std::rc::Rc;

pub struct Logger {
    pub frame: Frame,
    pub bg: ColorItem,
    pub objects: Vec<Rc<dyn Contains>>,
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {} #", self.x, self.y, self.w, self.h)
    }
}

impl fmt::Display for ColorItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} ", self.shade.0, self.shade.1, self.shade.2)?;
        write!(f, "{} {} {} ", self.theme.0, self.theme.1, self.theme.2)?;
        write!(f, "{} ", self.salt)?;
        write!(f, "{} {} #", self.deviation, self.distance)
    }
}

impl fmt::Display for SaltItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} ", self.color.0, self.color.1, self.color.2)?;
        write!(f, "{} {} ", self.likeliness, self.variability)
    }
}

impl fmt::Display for Salt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for item in self.0.iter() {
            write!(f, "{} ", item)?;
        }
        write!(f, "#")
    }
}

impl fmt::Display for Disc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Disc {} {} {} ",
            self.center.0, self.center.1, self.radius
        )?;
        write!(f, "{} #", self.color)
    }
}

impl fmt::Display for HalfPlane {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "HalfPlane {} {} {} {} ",
            self.limit.0, self.limit.1, self.reference.0, self.reference.1
        )?;
        write!(f, "{} #", self.color)
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Triangle {} {} ", self.a.0, self.a.1)?;
        write!(f, "{} {} ", self.b.0, self.b.1)?;
        write!(f, "{} {} ", self.c.0, self.c.1)?;
        write!(f, "{} #", self.color)
    }
}

impl fmt::Display for Spiral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Spiral {} {} {} {} ",
            self.center.0, self.center.1, self.width, self.tightness
        )?;
        write!(f, "{} #", self.color)
    }
}

impl fmt::Display for Stripe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Stripe {} {} {} {} ",
            self.limit.0, self.limit.1, self.reference.0, self.reference.1
        )?;
        write!(f, "{} #", self.color)
    }
}

impl fmt::Display for Wave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Wave {} {} {} {} ",
            self.limit.0, self.limit.1, self.reference.0, self.reference.1
        )?;
        write!(f, "{} {} ", self.amplitude, self.frequency)?;
        write!(f, "{} #", self.color)
    }
}

impl fmt::Display for Sawtooth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Sawtooth {} {} {} {} ",
            self.limit.0, self.limit.1, self.reference.0, self.reference.1
        )?;
        write!(f, "{} {} ", self.amplitude, self.frequency)?;
        write!(f, "{} #", self.color)
    }
}

impl fmt::Display for Logger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", self.frame)?;
        write!(f, "{} ", self.bg)?;
        write!(f, "{} ", self.objects.len())?;
        for o in &self.objects {
            write!(f, "{} ", o)?;
        }
        Ok(())
    }
}

impl Logger {
    pub fn save(&self, dest: &str) -> std::io::Result<()> {
        let mut buffer = File::create(dest)?;
        buffer.write_all(&format!("{}", &self).into_bytes())
    }

    pub fn load(src: &str) -> Self {
        let mut s = String::new();
        let mut file = File::open(src).unwrap();
        file.read_to_string(&mut s).unwrap();
        let mut items = s.split(' ');
        Logger::restore(&mut items)
    }
}

trait Restore {
    fn restore<'a>(items: &mut impl Iterator<Item = &'a str>) -> Self;
}

impl Restore for usize {
    fn restore<'a>(items: &mut impl Iterator<Item = &'a str>) -> Self {
        items.next().unwrap().parse::<Self>().unwrap()
    }
}

impl Restore for f64 {
    fn restore<'a>(items: &mut impl Iterator<Item = &'a str>) -> Self {
        items.next().unwrap().parse::<Self>().unwrap()
    }
}

impl Restore for Pos {
    fn restore<'a>(items: &mut impl Iterator<Item = &'a str>) -> Self {
        Self(f64::restore(items), f64::restore(items))
    }
}

impl Restore for Color {
    fn restore<'a>(items: &mut impl Iterator<Item = &'a str>) -> Self {
        Self(
            usize::restore(items),
            usize::restore(items),
            usize::restore(items),
        )
    }
}

impl Restore for Logger {
    fn restore<'a>(items: &mut impl Iterator<Item = &'a str>) -> Self {
        let frame = Frame::restore(items);
        let bg = ColorItem::restore(items);
        let len = items.next().unwrap().parse::<usize>().unwrap();
        let mut objects = Vec::new();
        for _ in 0..len {
            objects.push(match items.next().unwrap() {
                "Disc" => Rc::new(Disc::restore(items)) as Rc<dyn Contains>,
                "HalfPlane" => Rc::new(HalfPlane::restore(items)) as Rc<dyn Contains>,
                "Stripe" => Rc::new(Stripe::restore(items)) as Rc<dyn Contains>,
                "Triangle" => Rc::new(Triangle::restore(items)) as Rc<dyn Contains>,
                "Spiral" => Rc::new(Spiral::restore(items)) as Rc<dyn Contains>,
                "Wave" => Rc::new(Wave::restore(items)) as Rc<dyn Contains>,
                "Sawtooth" => Rc::new(Sawtooth::restore(items)) as Rc<dyn Contains>,
                _ => panic!("Unknown item"),
            });
        }
        Self { frame, bg, objects }
    }
}

impl Restore for Frame {
    fn restore<'a>(items: &mut impl Iterator<Item = &'a str>) -> Self {
        let x = usize::restore(items);
        let y = usize::restore(items);
        let w = usize::restore(items);
        let h = usize::restore(items);
        assert_eq!(items.next().unwrap(), "#");
        Self { x, y, w, h }
    }
}

impl Restore for ColorItem {
    fn restore<'a>(items: &mut impl Iterator<Item = &'a str>) -> Self {
        let shade = Color::restore(items);
        let theme = Color::restore(items);
        let deviation = usize::restore(items);
        let distance = usize::restore(items);
        assert_eq!(items.next().unwrap(), "#");
        Self {
            shade,
            theme,
            deviation,
            distance,
        }
    }
}

impl Restore for Disc {
    fn restore<'a>(items: &mut impl Iterator<Item = &'a str>) -> Self {
        let center = Pos::restore(items);
        let radius = f64::restore(items);
        let color = ColorItem::restore(items);
        assert_eq!(items.next().unwrap(), "#");
        Self {
            center,
            radius,
            color,
        }
    }
}

impl Restore for HalfPlane {
    fn restore<'a>(items: &mut impl Iterator<Item = &'a str>) -> Self {
        let limit = Pos::restore(items);
        let reference = Pos::restore(items);
        let color = ColorItem::restore(items);
        assert_eq!(items.next().unwrap(), "#");
        Self {
            limit,
            reference,
            color,
        }
    }
}

impl Restore for Stripe {
    fn restore<'a>(items: &mut impl Iterator<Item = &'a str>) -> Self {
        let limit = Pos::restore(items);
        let reference = Pos::restore(items);
        let color = ColorItem::restore(items);
        assert_eq!(items.next().unwrap(), "#");
        Self {
            limit,
            reference,
            color,
        }
    }
}

impl Restore for Triangle {
    fn restore<'a>(items: &mut impl Iterator<Item = &'a str>) -> Self {
        let a = Pos::restore(items);
        let b = Pos::restore(items);
        let c = Pos::restore(items);
        let color = ColorItem::restore(items);
        assert_eq!(items.next().unwrap(), "#");
        Self { a, b, c, color }
    }
}

impl Restore for Spiral {
    fn restore<'a>(items: &mut impl Iterator<Item = &'a str>) -> Self {
        let center = Pos::restore(items);
        let width = f64::restore(items);
        let tightness = f64::restore(items);
        let color = ColorItem::restore(items);
        assert_eq!(items.next().unwrap(), "#");
        Self {
            center,
            width,
            color,
            tightness,
        }
    }
}

impl Restore for Wave {
    fn restore<'a>(items: &mut impl Iterator<Item = &'a str>) -> Self {
        let limit = Pos::restore(items);
        let reference = Pos::restore(items);
        let amplitude = f64::restore(items);
        let frequency = f64::restore(items);
        let color = ColorItem::restore(items);
        assert_eq!(items.next().unwrap(), "#");
        Self {
            limit,
            reference,
            amplitude,
            frequency,
            color,
        }
    }
}

impl Restore for Sawtooth {
    fn restore<'a>(items: &mut impl Iterator<Item = &'a str>) -> Self {
        let limit = Pos::restore(items);
        let reference = Pos::restore(items);
        let amplitude = f64::restore(items);
        let frequency = f64::restore(items);
        let color = ColorItem::restore(items);
        assert_eq!(items.next().unwrap(), "#");
        Self {
            limit,
            reference,
            amplitude,
            frequency,
            color,
        }
    }
}
