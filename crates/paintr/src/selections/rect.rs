use druid::{Point, Rect, Size, Vec2};
use image::{DynamicImage, GenericImage, GenericImageView};
use imageproc::rect::Rect as ImRect;
use std::sync::Arc;

use super::{CopyMode, SelectionShape};

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

    fn copy_image(&self, img: Arc<DynamicImage>, mode: CopyMode) -> Option<Arc<DynamicImage>> {
        // FIXME: Is bounding a best solution here?
        let rect = intersect(*self, &img)?;

        let (x, y) = rect.origin().into();
        let (w, h) = rect.size().into();

        let new_img = img.view(x as u32, y as u32, w as u32, h as u32).to_image();

        match mode {
            CopyMode::Shrink => Some(Arc::new(DynamicImage::ImageRgba8(new_img))),
            CopyMode::Expand => {
                if rect.size() == self.size() {
                    return Some(Arc::new(DynamicImage::ImageRgba8(new_img)));
                }
                let mut output =
                    image::DynamicImage::new_rgba8(self.width() as u32, self.height() as u32);
                let pos = rect.origin() - self.origin();
                output.copy_from(&new_img, pos.x as u32, pos.y as u32);

                Some(Arc::new(output))
            }
        }
    }

    fn cut_image(&self, img: Arc<DynamicImage>) -> Option<Arc<DynamicImage>> {
        let rect = intersect(*self, &img)?;

        let rect = ImRect::at(rect.origin().x as i32, rect.origin().y as i32)
            .of_size(rect.width() as u32, rect.height() as u32);

        // Deep clone an image
        let img = imageproc::drawing::draw_filled_rect(
            img.as_ref(),
            rect,
            image::Rgba([0u8, 0u8, 0u8, 0u8]),
        );

        Some(Arc::new(DynamicImage::ImageRgba8(img)))
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

fn intersect(rt: Rect, img: &DynamicImage) -> Option<Rect> {
    let img_dims = img.dimensions();
    let bound = Rect::from_origin_size(Point::ORIGIN, (img_dims.0 as f64, img_dims.1 as f64));
    let rect = bound.intersect(rt);
    if rect.area() == 0.0 {
        return None;
    }

    Some(rect)
}
