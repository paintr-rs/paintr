use super::canvas::CanvasData;
use super::edit::{Edit, EditDesc};
use druid::Vec2;
use std::any::Any;
use std::sync::Arc;

pub struct Paste {
    img: Arc<image::DynamicImage>,
}

impl Paste {
    pub fn new(img: image::RgbaImage) -> Paste {
        Paste { img: Arc::new(image::DynamicImage::ImageRgba8(img)) }
    }
}

#[must_use]
impl Edit<CanvasData> for Paste {
    fn apply(&self, data: &mut CanvasData) {
        data.paste(self.img.clone());
    }

    fn description(&self) -> EditDesc {
        EditDesc::new("Paste")
    }
}

pub struct MoveCanvas {
    offset: Vec2,
}

impl MoveCanvas {
    pub fn new(offset: Vec2) -> MoveCanvas {
        MoveCanvas { offset }
    }
}

#[must_use]
impl Edit<CanvasData> for MoveCanvas {
    fn apply(&self, data: &mut CanvasData) {
        data.move_canvas(self.offset);
    }

    fn description(&self) -> EditDesc {
        EditDesc::new("Move")
    }

    fn merge(&self, other: &mut dyn Any) -> bool {
        if let Some(other) = other.downcast_mut::<Self>() {
            other.offset += self.offset;
            true
        } else {
            false
        }
    }
}

pub struct MoveSelection {
    offset: Vec2,
}

impl MoveSelection {
    pub fn new(offset: Vec2) -> MoveSelection {
        MoveSelection { offset }
    }
}

#[must_use]
impl Edit<CanvasData> for MoveSelection {
    fn apply(&self, data: &mut CanvasData) {
        data.move_selection(self.offset);
    }

    fn description(&self) -> EditDesc {
        EditDesc::new("Move Selection")
    }

    fn merge(&self, other: &mut dyn Any) -> bool {
        if let Some(other) = other.downcast_mut::<Self>() {
            other.offset += self.offset;
            true
        } else {
            false
        }
    }
}
