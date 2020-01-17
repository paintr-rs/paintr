use super::canvas::CanvasData;
use super::edit::{Edit, EditDesc};
use druid::Vec2;
use std::sync::Arc;

pub struct Paste {
    img: image::DynamicImage,
}

impl Paste {
    pub fn new(img: image::RgbaImage) -> Paste {
        Paste { img: image::DynamicImage::ImageRgba8(img) }
    }
}

#[must_use]
impl Edit<CanvasData> for Paste {
    fn apply(&self, data: &mut CanvasData) {
        data.planes.push(Arc::new(self.img.clone()));
    }

    fn description(&self) -> EditDesc {
        EditDesc::new("Paste")
    }
}

pub struct Move {
    offset: Vec2,
}

impl Move {
    pub fn new(offset: Vec2) -> Move {
        Move { offset }
    }
}

#[must_use]
impl Edit<CanvasData> for Move {
    fn apply(&self, data: &mut CanvasData) {
        data.mov(self.offset);
    }
    fn description(&self) -> EditDesc {
        EditDesc::new("Move")
    }
}
