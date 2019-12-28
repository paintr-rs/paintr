//! Paint helper trait
//!
//! A paint trait which let overriding paint behaivor easier

use druid::kurbo::{Point, Rect, Size};
use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx, Widget, WidgetPod,
};

pub struct Paint<T: Data> {
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
    f: Box<dyn Fn(&mut PaintCtx, &T, &Env)>,
}

impl<T: Data> Widget<T> for Paint<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.inner.event(ctx, event, data, env);
    }
    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: Option<&T>, data: &T, env: &Env) {
        self.inner.update(ctx, data, env);
    }
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let size = self.inner.layout(ctx, bc, data, env);
        self.inner.set_layout_rect(Rect::from_origin_size(Point::ORIGIN, size));
        size
    }
    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &T, env: &Env) {
        (*self.f)(paint_ctx, data, env);
        self.inner.paint_with_offset(paint_ctx, data, env);
    }
}

pub trait Painter<T: Data> {
    fn painter(self, f: impl Fn(&mut PaintCtx, &T, &Env) + 'static) -> Paint<T>;
}

impl<W, T: Data> Painter<T> for W
where
    W: Widget<T> + 'static,
{
    fn painter(self, f: impl Fn(&mut PaintCtx, &T, &Env) + 'static) -> Paint<T> {
        Paint { inner: WidgetPod::new(self).boxed(), f: Box::new(f) }
    }
}
