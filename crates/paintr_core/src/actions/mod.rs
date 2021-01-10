use super::canvas::CanvasData;
use super::edit::{Edit, EditDesc};
use druid::Vec2;
use std::any::Any;
use std::sync::Arc;

pub struct Paste {
    img: Arc<image::DynamicImage>,
}

impl Paste {
    pub fn new(img: image::DynamicImage) -> Paste {
        Paste { img: Arc::new(img) }
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::image_utils::{
        colors::{BLACK, TRANSPARENT, WHITE},
        make_color_img,
    };
    use crate::test_utils::canvas_fixture;
    use druid::{Point, Rect};
    use image::GenericImageView;

    #[test]
    fn paste_should_works() {
        let mut canvas = canvas_fixture(16, 16, BLACK);

        let white = make_color_img(4, 4, WHITE);
        let action = Paste::new(white);
        let old = action.execute(&mut canvas);
        let img = old.merged();
        assert_eq!(img.get_pixel(2, 2), BLACK);

        let img = canvas.merged();
        assert_eq!(img.get_pixel(2, 2), WHITE);
    }

    #[test]
    fn move_canvas_should_works() {
        let mut canvas = canvas_fixture(16, 16, BLACK);

        let action = MoveCanvas::new(Vec2::new(4.0, 4.0));
        let old = action.execute(&mut canvas);
        let img = old.merged();
        assert_eq!(img.get_pixel(2, 2), BLACK);

        let img = canvas.merged();
        assert_eq!(img.get_pixel(2, 2), TRANSPARENT);
    }

    #[test]
    fn move_selection_should_works() {
        let mut canvas = canvas_fixture(16, 16, BLACK);
        canvas.select(Rect::from_origin_size(Point::ZERO, (2.0, 2.0)));

        let action = MoveSelection::new(Vec2::new(4.0, 4.0));
        let old = action.execute(&mut canvas);
        let img = old.merged();
        assert_eq!(img.get_pixel(2, 2), BLACK);

        let img = canvas.merged();
        assert_eq!(img.get_pixel(2, 2), TRANSPARENT);
    }
}
