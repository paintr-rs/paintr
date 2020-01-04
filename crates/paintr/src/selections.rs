use druid::kurbo::Shape;
use druid::piet::StrokeStyle;
use druid::{Color, Data, PaintCtx, Point, Rect, RenderContext, Size, Vec2};

use crate::Paintable;
use image::DynamicImage;
use std::sync::Arc;
mod rect;

trait SelectionShape: Shape {
    fn size(&self) -> Size;
    fn description(&self) -> String;
    fn copy_image(&self, img: Arc<DynamicImage>) -> Option<Arc<DynamicImage>>;
    fn contains(&self, pt: Point) -> bool;
    fn transform(&self, offset: Vec2) -> Self;
    fn same(&self, other: &Self) -> bool;
}

#[derive(Debug, Clone)]
pub enum Selection {
    Rect(Rect),
}

impl_from! {
    Selection : [
        Rect => Rect
    ]
}

impl Selection {
    pub fn description(&self) -> String {
        match self {
            Selection::Rect(rt) => rt.description(),
        }
    }

    pub fn size(&self) -> Size {
        match self {
            Selection::Rect(rt) => rt.size(),
        }
    }

    pub fn copy_image(&self, img: Arc<DynamicImage>) -> Option<Arc<DynamicImage>> {
        match self {
            Selection::Rect(rect) => rect.copy_image(img),
        }
    }

    pub fn shape(&self) -> impl Shape {
        match self {
            Selection::Rect(rt) => *rt,
        }
    }

    pub fn contains(&self, pt: Point) -> bool {
        match self {
            Selection::Rect(rt) => rt.contains(pt),
        }
    }

    pub fn transform(&self, offset: Vec2) -> Selection {
        match self {
            Selection::Rect(rt) => rt.transform(offset).into(),
        }
    }
}

impl Data for Selection {
    fn same(&self, other: &Self) -> bool {
        match (self, other) {
            (Selection::Rect(a), Selection::Rect(b)) => a.same(b),
        }
    }
}

impl Paintable for Selection {
    fn paint(&self, paint_ctx: &mut PaintCtx) {
        // Create a color
        let stroke_color = Color::rgb8(0xff, 0xff, 0xff);

        let mut style = StrokeStyle::new();
        let dashes = vec![2.0, 2.0];
        style.set_dash(dashes, 0.0);

        paint_ctx.stroke_styled(self.shape(), &stroke_color, 1.0, &style);
    }

    fn paint_size(&self) -> Option<Size> {
        Some(self.size())
    }
}
