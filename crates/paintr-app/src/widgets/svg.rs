//! Paint helper trait
//!
//! A paint trait which let overriding paint behaivor easier
use druid::{
    theme, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, Point, Rect,
    RenderContext, Size, UpdateCtx, Widget,
};

use paintr::{Paintable, SvgImage};

pub struct Svg<T: Data> {
    svg_image: SvgImage,
    phantom: std::marker::PhantomData<T>,
}

impl<T: Data> Widget<T> for Svg<T> {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {}
    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: Option<&T>, _data: &T, _env: &Env) {}
    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, _env: &Env) -> Size {
        bc.constrain(self.svg_image.paint_size().unwrap_or_else(|| (100.0, 100.0).into()))
    }
    fn paint(&mut self, paint_ctx: &mut PaintCtx, _data: &T, env: &Env) {
        let rt = Rect::from_origin_size(Point::ORIGIN, paint_ctx.size());
        let color = env.get(theme::BACKGROUND_LIGHT);
        self.svg_image.set_default_fill(color.clone());

        paint_ctx.stroke(rt, &color, 1.0);
        self.svg_image.paint(paint_ctx);
    }
}

impl<T: Data> Svg<T> {
    pub fn new(s: &str) -> Svg<T> {
        Svg { svg_image: SvgImage::new(s).unwrap(), phantom: std::marker::PhantomData::default() }
    }
}
