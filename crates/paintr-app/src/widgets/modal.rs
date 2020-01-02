//! Modal and ModalContainer
//!
//! A widget represent modal widget

use druid::{
    lens::{self, LensExt},
    BoxConstraints, Command, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, Point, Rect, Size,
    UpdateCtx, Widget, WidgetPod,
};

pub trait Modal {
    fn is_closed(&self) -> Option<Command>;
}

pub struct ModalContainer<T: Data, M: Data + Modal, L: lens::Lens<T, Option<M>>> {
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
    closure: Box<dyn Fn(&M, &Env) -> Option<Box<dyn Widget<M>>>>,
    lens: L,
    modal: Option<WidgetPod<M, Box<dyn Widget<M>>>>,
}

impl<T: Data, M: Data + Modal, L: lens::Lens<T, Option<M>>> ModalContainer<T, M, L> {
    pub fn new<F>(inner: impl Widget<T> + 'static, closure: F, lens: L) -> Self
    where
        F: Fn(&M, &Env) -> Option<Box<dyn Widget<M>>> + 'static,
    {
        ModalContainer {
            inner: WidgetPod::new(inner).boxed(),
            closure: Box::new(closure),
            lens,
            modal: None,
        }
    }
}

impl<T: Data, M: Data + Modal + 'static, L: lens::Lens<T, Option<M>>> Widget<T>
    for ModalContainer<T, M, L>
{
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        // if we have modal, block all other event
        if let Some(modal) = &mut self.modal {
            self.lens.with_mut(data, |data| {
                if let Some(modal_data) = data {
                    modal.event(ctx, event, modal_data, env);

                    if let Some(cmd) = modal_data.is_closed() {
                        ctx.submit_command(cmd, ctx.window_id());
                        *data = None;
                    }
                }
            });

            return;
        }

        self.inner.event(ctx, event, data, env);
    }
    fn update(&mut self, ctx: &mut UpdateCtx, old_data: Option<&T>, data: &T, env: &Env) {
        self.inner.update(ctx, data, env);

        let changed = match old_data {
            Some(old_data) => self
                .lens
                .with(old_data, |old_data| self.lens.with(data, |data| !old_data.same(data))),
            _ => true,
        };

        if changed {
            self.modal = self
                .lens
                .get(data)
                .as_ref()
                .and_then(|data| (*self.closure)(data, env))
                .map(|w| WidgetPod::new(w).boxed());

            ctx.invalidate();
        }

        if let Some(modal) = &mut self.modal {
            self.lens.with(data, |data| {
                if let Some(data) = data {
                    modal.update(ctx, data, env);
                }
            });
        }
    }
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let size = self.inner.layout(ctx, bc, data, env);
        self.inner.set_layout_rect(Rect::from_origin_size(Point::ORIGIN, size));

        if let Some(modal) = &mut self.modal {
            self.lens.with(data, |data| {
                if let Some(data) = data {
                    let size = modal.layout(ctx, bc, data, env);
                    modal.set_layout_rect(Rect::from_origin_size(Point::ORIGIN, size));
                }
            });
        }

        size
    }
    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.inner.paint_with_offset(paint_ctx, data, env);

        if let Some(modal) = &mut self.modal {
            self.lens.with(data, |data| {
                if let Some(data) = data {
                    modal.paint_with_offset(paint_ctx, data, env);
                }
            });
        }
    }
}
