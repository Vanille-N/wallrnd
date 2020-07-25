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
        write!(f, "{} {} #", self.deviation, self.weight)
    }
}

impl fmt::Display for Disc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Disc {} {} {} ", self.center.0, self.center.1, self.radius)?;
        write!(f, "{} #", self.color)
    }
}

impl fmt::Display for HalfPlane {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HalfPlane {} {} {} {} ", self.limit.0, self.limit.1, self.reference.0, self.reference.1)?;
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
        write!(f, "Spiral {} {} {} ", self.center.0, self.center.1, self.width)?;
        write!(f, "{} #", self.color)
    }
}

impl fmt::Display for Stripe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stripe {} {} {} {} ", self.limit.0, self.limit.1, self.reference.0, self.reference.1)?;
        write!(f, "{} #", self.color)
    }
}

impl fmt::Display for Wave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wave {} {} {} {} ", self.limit.0, self.limit.1, self.reference.0, self.reference.1)?;
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

impl Restore for Logger {
    fn restore<'a>(items: &mut impl Iterator<Item = &'a str>) -> Self {
        let frame = Frame::restore(items);
        let bg = ColorItem::restore(items);
        let len = items.next().unwrap().parse::<usize>().unwrap();
        let mut objects = Vec::new();
        for _ in 0..len {
            objects.push(Rc::new(match items.next().unwrap() {
                "Disc" => Disc::restore(items),
                _ => panic!("Unknown item"),
            }) as Rc<dyn Contains>);
        }
        Self { frame, bg, objects }
    }
}

impl Restore for Frame {
    fn restore<'a>(items: &mut impl Iterator<Item = &'a str>) -> Self {
        let x = items.next().unwrap().parse::<usize>().unwrap();
        let y = items.next().unwrap().parse::<usize>().unwrap();
        let w = items.next().unwrap().parse::<usize>().unwrap();
        let h = items.next().unwrap().parse::<usize>().unwrap();
        assert_eq!(items.next().unwrap(), "#");
        Self { x, y, w, h }
    }
}

impl Restore for ColorItem {
    fn restore<'a>(items: &mut impl Iterator<Item = &'a str>) -> Self {
        let mut v = Vec::new();
        for _ in 0..8 {
            v.push(items.next().unwrap().parse::<usize>().unwrap() as i32);
        }
        assert_eq!(items.next().unwrap(), "#");
        Self {
            shade: Color(v[0], v[1], v[2]),
            theme: Color(v[3], v[4], v[5]),
            deviation: v[6],
            weight: v[7],
        }
    }
}
}
