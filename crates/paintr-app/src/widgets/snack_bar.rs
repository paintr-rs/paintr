//! Toast Box
//!
//! A widget represent a message box

use druid::kurbo::{Point, Rect, Size};
use druid::piet::{Color, UnitPoint};
use druid::widget::{Align, Label, List, WidgetExt};
use druid::{
    lens::{self, LensExt},
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx, Widget, WidgetPod,
};

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[derive(Hash, Clone)]
struct TimerKey(Arc<String>);
impl PartialEq for TimerKey {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for TimerKey {}

pub struct SnackBarContainer<T, L>
where
    T: Data,
{
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
    bars: Align<T>,
    snackbar_lens: L,

    lifes: HashMap<TimerKey, f64>,
}

type MessagesData = Arc<Vec<Arc<String>>>;

impl<T: Data, L: lens::Lens<T, MessagesData> + 'static + Clone> SnackBarContainer<T, L> {
    pub fn new(inner: impl Widget<T> + 'static, snackbar_lens: L) -> Self {
        let bars = List::new(|| {
            Align::right(
                Label::new(|item: &Arc<String>, _env: &_| item.as_ref().clone())
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
            lifes: HashMap::default(),
        }
    }
}

impl<T: Data, L: lens::Lens<T, MessagesData>> SnackBarContainer<T, L> {
    fn remove_item(&self, data: &mut T, item: &Arc<String>) {
        self.snackbar_lens.with_mut(data, |d: &mut _| {
            if d.len() > 0 {
                Arc::make_mut(d).retain(|it| !Arc::ptr_eq(it, item));
            }
        })
    }

    fn has_item(&self, data: &T) -> bool {
        self.snackbar_lens.get(data).len() > 0
    }

    fn sync_lifes(&mut self, data: &MessagesData) {
        let mut avails = HashSet::new();
        for item in data.iter() {
            let key = TimerKey(item.clone());
            self.lifes.entry(key.clone()).or_insert_with(|| 0.0);
            avails.insert(key);
        }
        self.lifes.retain(|key, _| avails.contains(key));
    }
}

impl<T: Data, L: lens::Lens<T, MessagesData> + Clone> Widget<T> for SnackBarContainer<T, L> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.inner.event(ctx, event, data, env);
        self.bars.event(ctx, event, data, env);

        match event {
            Event::AnimFrame(interval) => {
                let dt = (*interval as f64) * 1e-9;
                let mut remains = std::mem::take(&mut self.lifes);
                remains.retain(|item, t| {
                    *t += dt;
                    if *t >= 3.0 {
                        self.remove_item(data, &item.0);
                        false
                    } else {
                        true
                    }
                });
                self.lifes = rema_ins;
                self.snackbar_lens.clone().with(data, |it| {
                    self.sync_lifes(it);
                });
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

        let lens = self.snackbar_lens.clone();

        if let Some(d) = old_data {
            lens.with(d, |old| {
                lens.with(data, |new| {
                    if !new.same(old) {
                        ctx.invalidate();
                        self.sync_lifes(new);
                    }
                })
            })
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let size = self.bars.layout(ctx, &bc, data, env);
        self.inner
            .set_layout_rect(Rect::from_origin_size(Point::ORIGIN, size));

        let size = self.inner.layout(ctx, &bc, data, env);
        self.inner
            .set_layout_rect(Rect::from_origin_size(Point::ORIGIN, size));
        size
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.inner.paint_with_offset(paint_ctx, data, env);

        if self.snackbar_lens.get(data).len() > 0 {
            self.bars.paint(paint_ctx, data, env);
        }
    }
}
