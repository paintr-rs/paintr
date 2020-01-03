use druid::kurbo::Shape;
use druid::piet::StrokeStyle;
use druid::{Color, Data, PaintCtx, Rect, RenderContext, Size};

use crate::Paintable;
use image::{DynamicImage, GenericImageView};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Selection {
    rect: Rect,
}

impl Selection {
    pub fn description(&self) -> String {
        let size = self.rect.size();
        format!(
            "X: {}, Y: {}, W: {}, H: {}",
            self.rect.x0 as usize, self.rect.y0 as usize, size.width as usize, size.height as usize,
        )
    }

    pub fn size(&self) -> Size {
        self.rect.size()
    }

    pub fn image(&self, img: &Arc<DynamicImage>) -> Arc<DynamicImage> {
        let (x, y) = self.rect.origin().into();
        let (w, h) = self.rect.size().into();
        let new_img = img.view(x as u32, y as u32, w as u32, h as u32).to_image();

        Arc::new(DynamicImage::ImageRgba8(new_img))
    }

    pub fn path(&self) -> impl Shape {
        self.rect
    }

    pub fn rect(rect: Rect) -> Selection {
        Selection { rect }
    }
}

impl Data for Selection {
    fn same(&self, other: &Self) -> bool {
        self.rect.size() == other.rect.size() && self.rect.origin() == self.rect.origin()
    }
}

impl Paintable for Selection {
    fn paint(&self, paint_ctx: &mut PaintCtx) {
        // Create a color
        let stroke_color = Color::rgb8(0xff, 0xff, 0xff);

        let mut style = StrokeStyle::new();
        let dashes = vec![2.0, 2.0];
        style.set_dash(dashes, 0.0);

        paint_ctx.stroke_styled(self.path(), &stroke_color, 1.0, &style);
    }

    fn paint_size(&self) -> Option<Size> {
        Some(self.size())
    }
}
