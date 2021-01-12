use druid::piet::StrokeStyle;
use druid::{kurbo::Shape, PaintCtx};
use druid::{Color, Data, Point, Rect, RenderContext, Size, Vec2};

use crate::Paintable;
use image::DynamicImage;
use std::sync::Arc;
mod rect;

trait SelectionShape: Shape {
    fn size(&self) -> Size;
    fn description(&self) -> String;
    fn copy(&self, img: Arc<DynamicImage>, mode: CopyMode) -> Option<Arc<DynamicImage>>;
    fn cutout(&self, img: Arc<DynamicImage>) -> Option<Arc<DynamicImage>>;
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

pub enum CopyMode {
    Shrink,
    Expand,
}

impl Selection {
    pub fn description(&self) -> String {
        match self {
            Selection::Rect(rt) => rt.description(),
        }
    }

    fn size(&self) -> Size {
        match self {
            Selection::Rect(rt) => rt.size(),
        }
    }

    pub fn area(&self) -> f64 {
        match self {
            Selection::Rect(rt) => rt.area(),
        }
    }

    pub fn copy(&self, img: Arc<DynamicImage>, mode: CopyMode) -> Option<Arc<DynamicImage>> {
        match self {
            Selection::Rect(rect) => rect.copy(img, mode),
        }
    }

    pub fn cutout(&self, img: Arc<DynamicImage>) -> Option<Arc<DynamicImage>> {
        match self {
            Selection::Rect(rect) => rect.cutout(img),
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

    pub fn position(&self) -> Point {
        self.shape().bounding_box().origin()
    }
}

impl Data for Selection {
    fn same(&self, other: &Self) -> bool {
        match (self, other) {
            (Selection::Rect(a), Selection::Rect(b)) => a == b,
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

        paint_ctx.render_ctx.stroke_styled(self.shape(), &stroke_color, 1.0, &style);
    }

    fn paint_size(&self) -> Option<Size> {
        Some(self.size())
    }
}
