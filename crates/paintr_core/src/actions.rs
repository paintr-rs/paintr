use super::canvas::CanvasData;
use super::edit::{Edit, EditDesc};
use druid::Vec2;
use std::any::Any;
use std::sync::Arc;

/// Paste image to canvas
pub struct Paste {
    img: Arc<image::DynamicImage>,
}

impl std::fmt::Debug for Paste {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Paste").field("x", &Arc::as_ptr(&self.img)).finish()
    }
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

/// Move the whole canvas
#[derive(Debug)]
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

/// Move selection
#[derive(Debug)]
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

/// Draw brush on canvas
#[derive(Debug)]
pub struct DrawBrush {
    pos: Vec<Vec2>,
}

impl DrawBrush {
    pub fn new(pos: Vec<Vec2>) -> Self {
        DrawBrush { pos }
    }
}

#[must_use]
impl Edit<CanvasData> for DrawBrush {
    fn apply(&self, data: &mut CanvasData) {
        data.draw_with_brush(&self.pos);
    }

    fn description(&self) -> EditDesc {
        EditDesc::new("Draw Brush")
    }

    fn merge(&self, other: &mut dyn Any) -> bool {
        if let Some(other) = other.downcast_mut::<Self>() {
            other.pos.append(&mut self.pos.clone());
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
    use image::{DynamicImage, GenericImageView};

    #[allow(unused)]
    macro_rules! dbg_img {
        ($e:expr) => {
            print_debug_img(&format!("{}:{}:{}", file!(), line!(), column!()), $e)
        };
    }

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
        canvas.select(Rect::from_origin_size(Point::ZERO, (4.0, 4.0)));

        let action = MoveSelection::new(Vec2::new(4.0, 4.0));
        let old = action.execute(&mut canvas);
        let img = old.merged();
        assert_eq!(img.get_pixel(2, 2), BLACK);

        let img = canvas.merged();
        assert_eq!(img.get_pixel(2, 2), TRANSPARENT);
    }

    #[test]
    fn move_selection_should_works_in_multiple_planes() {
        let mut canvas = canvas_fixture(16, 16, BLACK);

        canvas.select(Rect::from_origin_size(Point::ZERO, (4.0, 4.0)));
        let action = MoveSelection::new(Vec2::new(4.0, 4.0));
        action.execute(&mut canvas);
        assert_eq!(canvas.merged().get_pixel(2, 2), TRANSPARENT);

        canvas.select(Rect::from_origin_size(Point::ZERO, (8.0, 8.0)));
        let action = MoveSelection::new(Vec2::new(8.0, 8.0));
        action.execute(&mut canvas);
        assert_eq!(canvas.merged().get_pixel(6, 6), TRANSPARENT);
    }

    #[allow(unused)]
    fn print_debug_img(info: &str, img: &DynamicImage) {
        println!("{}", info);
        for y in 0..img.height() {
            for x in 0..img.width() {
                match img.get_pixel(x, y) {
                    TRANSPARENT => print!("T"),
                    BLACK => print!("B"),
                    WHITE => print!("W"),
                    _ => print!("?"),
                }
            }
            println!("");
        }
    }
}
