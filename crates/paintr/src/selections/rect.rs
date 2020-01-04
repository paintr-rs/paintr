use druid::{Point, Rect, Size, Vec2};
use image::{DynamicImage, GenericImageView};
use std::sync::Arc;

use super::SelectionShape;

impl SelectionShape for Rect {
    fn description(&self) -> String {
        format!(
            "X: {}, Y: {}, W: {}, H: {}",
            self.x0 as i32,
            self.y0 as i32,
            self.width() as i32,
            self.height() as i32,
        )
    }

    fn size(&self) -> Size {
        self.size()
    }

    fn copy_image(&self, img: Arc<DynamicImage>) -> Option<Arc<DynamicImage>> {
        // FIXME: Is bounding a best solution here?
        let img_dims = img.dimensions();
        let bound = Rect::from_origin_size(Point::ORIGIN, (img_dims.0 as f64, img_dims.1 as f64));
        let rect = bound.intersect(*self);
        if rect.size() == Size::ZERO {
            return None;
        }

        let (x, y) = rect.origin().into();
        let (w, h) = rect.size().into();

        let new_img = img.view(x as u32, y as u32, w as u32, h as u32).to_image();

        Some(Arc::new(DynamicImage::ImageRgba8(new_img)))
    }

    fn contains(&self, pt: Point) -> bool {
        self.contains(pt)
    }

    fn transform(&self, offset: Vec2) -> Self {
        let new_origin = (self.origin().to_vec2() + offset).to_point();
        Rect::from_origin_size(new_origin, self.size())
    }

    fn same(&self, other: &Self) -> bool {
        self.origin() == other.origin() && self.size() == other.size()
    }
}
