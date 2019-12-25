//! Named Window
//!
//! A widget represent a named window

use druid::kurbo::{Point, Rect, Size};
use druid::piet::{Color, RenderContext};
use druid::widget::{Label, LabelText, WidgetExt};
use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx, Widget, WidgetPod,
};

pub struct Named<T: Data> {
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
    label: Box<dyn Widget<T>>,
}

impl<T: Data> Widget<T> for Named<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.inner.event(ctx, event, data, env);
    }
    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: Option<&T>, data: &T, env: &Env) {
        self.inner.update(ctx, data, env);
    }
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let size = self.label.layout(ctx, &bc, data, env);
        self.inner
            .set_layout_rect(Rect::from_origin_size(Point::ORIGIN, size));

        let header_offset = size.height;
        let child_bc = bc.shrink((0.0, header_offset));
        let size = self.inner.layout(ctx, &child_bc, data, env);
        let origin = Point::new(0.0, header_offset);
        self.inner
            .set_layout_rect(Rect::from_origin_size(origin, size));

        Size::new(size.width, size.height + header_offset)
    }
    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &T, env: &Env) {
        let size = paint_ctx.size();
        paint_ctx
            .with_save(|paint_ctx| {
                let rect = Rect::from_origin_size(Point::ORIGIN, size);
                paint_ctx.fill(rect, &Color::rgb8(0x11, 0x93, 0x92));
                Ok(())
            })
            .unwrap();

        self.inner.paint_with_offset(paint_ctx, data, env);
        self.label.paint(paint_ctx, data, env);
    }
}

impl<T: Data> Named<T> {
    pub fn new(inner: impl Widget<T> + 'static, label: impl Into<LabelText<T>>) -> impl Widget<T>
    where
        T: Data + 'static,
    {
        let label = Label::new(label.into()).padding(10.0);

        Named {
            inner: WidgetPod::new(inner).boxed(),
            label: Box::new(label),
        }
    }
}
