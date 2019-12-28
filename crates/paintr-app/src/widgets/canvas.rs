use druid::kurbo::{Point, Rect, Size};
use druid::piet::{ImageFormat, InterpolationMode, RenderContext};
use druid::{BoxConstraints, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx, Widget};

use image::{DynamicImage, RgbaImage};
use std::sync::Arc;
pub struct Canvas {
    image: Option<RgbaImage>,
}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas { image: None }
    }
}

type DataType = Option<Arc<DynamicImage>>;

impl Widget<DataType> for Canvas {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut DataType, _env: &Env) {}

    fn update(
        &mut self,
        _ctx: &mut UpdateCtx,
        old_data: Option<&DataType>,
        data: &DataType,
        _env: &Env,
    ) {
        let changed = match (old_data, data) {
            (None, _) => true,
            (Some(_), None) => true,
            (Some(&None), Some(_)) => true,
            (Some(Some(old)), Some(new)) => !Arc::ptr_eq(old, new),
        };

        if changed {
            self.image = data.as_ref().map(|img| img.to_rgba());
        }
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &DataType,
        _env: &Env,
    ) -> Size {
        match &self.image {
            Some(img) => (img.width() as f64, img.height() as f64).into(),
            None => bc.max(),
        }
    }

    // The paint method gets called last, after an event flow.
    // It goes event -> update -> layout -> paint, and each method can influence the next.
    // Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, paint_ctx: &mut PaintCtx, _data: &DataType, _env: &Env) {
        let img = match &self.image {
            Some(img) => img,
            _ => return,
        };
        let size = (img.width() as usize, img.height() as usize);

        // FIXME: Draw image only in paint_ctx.region
        let image = paint_ctx.make_image(size.0, size.1, &img, ImageFormat::RgbaSeparate).unwrap();
        // The image is automatically scaled to fit the rect you pass to draw_image
        paint_ctx.draw_image(
            &image,
            Rect::from_origin_size(Point::ORIGIN, (size.0 as f64, size.1 as f64)),
            InterpolationMode::NearestNeighbor,
        );
    }
}
