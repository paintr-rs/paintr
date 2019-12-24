//! Toast Box
//!
//! A widget represent a message box

use druid::kurbo::Size;
use druid::piet::{Color, UnitPoint};
use druid::widget::{Align, Label, List, WidgetExt};
use druid::{
    lens::{self, LensExt},
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx, Widget, WidgetPod,
};

use std::sync::Arc;

pub struct SnackBarContainer<T, L>
where
    T: Data,
{
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
    bars: Align<T>,
    snackbar_lens: L,
    t: f64,
}

impl<T: Data, L: lens::Lens<T, Arc<Vec<String>>> + 'static + Clone> SnackBarContainer<T, L> {
    pub fn new(inner: impl Widget<T> + 'static, snackbar_lens: L) -> Self {
        let bars = List::new(|| {
            Align::right(
                Label::new(|item: &String, _env: &_| item.clone())
                    .padding(10.0)
                    .background(Color::grey(0.3)),
            )
            .padding((10.0, 5.0))
        })
        .lens(snackbar_lens.clone());

        Self {
            inner: WidgetPod::new(inner).boxed(),
            bars: Align::vertical(UnitPoint::BOTTOM_RIGHT, bars),
            snackbar_lens,
            t: 0.0,
        }
    }
}

impl<T: Data, L: lens::Lens<T, Arc<Vec<String>>>> SnackBarContainer<T, L> {
    fn remove_item(&self, data: &mut T) {
        self.snackbar_lens.with_mut(data, |d: &mut _| {
            if d.len() > 0 {
                Arc::make_mut(d).remove(0);
            }
        })
    }

    fn has_item(&self, data: &T) -> bool {
        self.snackbar_lens.get(data).len() > 0
    }
}

impl<T: Data, L: lens::Lens<T, Arc<Vec<String>>>> Widget<T> for SnackBarContainer<T, L> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.inner.event(ctx, event, data, env);
        self.bars.event(ctx, event, data, env);

        match event {
            Event::AnimFrame(interval) => {
                self.t += (*interval as f64) * 1e-9;
                if self.t >= 8.0 {
                    self.remove_item(data);
                    self.t = 0.0;
                }
            }
            _ => (),
        }

        if self.has_item(data) {
            ctx.request_anim_frame();
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: Option<&T>, data: &T, env: &Env) {
        self.inner.update(ctx, data, env);
        self.bars.update(ctx, old_data, data, env);

        if let Some(d) = old_data {
            self.snackbar_lens.with(d, |old| {
                self.snackbar_lens.with(data, |new| {
                    if !new.same(old) {
                        ctx.invalidate();
                    }
                })
            })
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.bars.layout(ctx, &bc, data, env);
        self.inner.layout(ctx, &bc, data, env)
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.inner.paint(paint_ctx, data, env);

        if self.snackbar_lens.get(data).len() > 0 {
            self.bars.paint(paint_ctx, data, env);
        }
    }
}
