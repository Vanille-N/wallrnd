use crate::prelude::*;
use crate::scene::*;
pub struct Logger<'a> {
    pub frame: Frame,
    pub bg: &'a ColorItem,
    pub objects: &'a [Box<dyn Contains>],
}
