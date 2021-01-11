//! Conditional widget
//!
//! A widget represent a conditional widget
use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    Point, Size, UpdateCtx, Widget, WidgetPod,
};

pub struct Conditional<T: Data> {
    closure: Box<dyn Fn(&T, &Env) -> bool>,
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
    current: bool,
}

impl<T: Data> Widget<T> for Conditional<T> {
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.current = (self.closure)(data, env);
        }

        if event.should_propagate_to_hidden() || self.current {
            self.inner.lifecycle(ctx, event, data, env);
        }
    }
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if event.should_propagate_to_hidden() || self.current {
            self.inner.event(ctx, event, data, env);
        }
    }
    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        let current = (self.closure)(data, env);
        if current != self.current {
            self.current = current;
            ctx.request_layout();
        }

        if current {
            self.inner.update(ctx, data, env);
        }
    }
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        if self.current {
            let size = self.inner.layout(ctx, bc, data, env);
            self.inner.set_origin(ctx, data, env, Point::ORIGIN);
            ctx.set_paint_insets(self.inner.paint_insets());
            size
        } else {
            // FIXME: We should be supported occupied mode
            self.inner.layout(ctx, bc, data, env)
        }
    }
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        if self.current {
            self.inner.paint(ctx, data, env)
        }
    }
}

impl<T: Data> Conditional<T> {
    pub fn new(
        closure: impl Fn(&T, &Env) -> bool + 'static,
        inner: impl Widget<T> + 'static,
    ) -> impl Widget<T>
    where
        T: Data + 'static,
    {
        Conditional {
            closure: Box::new(closure),
            inner: WidgetPod::new(inner).boxed(),
            current: false,
        }
    }
}
