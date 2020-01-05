//! Paint helper trait
//!
//! A paint trait which let overriding paint behaivor easier
use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, Size, UpdateCtx, Widget,
};

use crate::theme_ext::PAINTR_TOGGLE_FOREGROND;
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
        self.svg_image.set_default_fill(env.get(PAINTR_TOGGLE_FOREGROND));
        self.svg_image.paint(paint_ctx);
    }
}

impl<T: Data> Svg<T> {
    pub fn new(s: &str) -> Svg<T> {
        Svg { svg_image: SvgImage::new(s).unwrap(), phantom: std::marker::PhantomData::default() }
    }
}
