use druid::piet::{ImageFormat, InterpolationMode};
use druid::{PaintCtx, Point, Rect, RenderContext, Size};
use image::{DynamicImage, GenericImageView};

pub trait Paintable {
    fn paint(&self, paint_ctx: &mut PaintCtx);
    fn paint_size(&self) -> Option<Size>;
}

impl Paintable for DynamicImage {
    fn paint(&self, paint_ctx: &mut PaintCtx) {
        let size = (self.width() as usize, self.height() as usize);

        // FIXME: Draw image only in paint_ctx.region
        let image = paint_ctx
            .make_image(size.0, size.1, &self.as_rgba8().unwrap(), ImageFormat::RgbaSeparate)
            .unwrap();
        // The image is automatically scaled to fit the rect you pass to draw_image
        paint_ctx.draw_image(
            &image,
            Rect::from_origin_size(Point::ORIGIN, self.paint_size().unwrap()),
            InterpolationMode::NearestNeighbor,
        );
    }

    fn paint_size(&self) -> Option<Size> {
        Some((self.width() as f64, self.height() as f64).into())
    }
}
