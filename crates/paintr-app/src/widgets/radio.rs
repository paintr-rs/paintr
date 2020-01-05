#![allow(dead_code)]

//! A radio button widget with any widget as children

use std::marker::PhantomData;

use druid::kurbo::{Point, Rect, Size};
use druid::theme;
use druid::widget::{Align, Flex, WidgetExt};
use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, RenderContext, UnitPoint,
    UpdateCtx, Widget, WidgetPod,
};

use crate::theme_ext::{PAINTR_TOGGLE_FOREGROND, PAINTR_TOGGLE_OFF, PAINTR_TOGGLE_ON};

/// A group of radio buttons
#[derive(Debug, Clone)]
pub struct RadioGroup<T: Data + PartialEq + 'static> {
    phantom: PhantomData<T>,
}

impl<T: Data + PartialEq + Copy + 'static> RadioGroup<T> {
    /// Given a vector of `(widget, enum_variant)` tuples, create a group of Radio buttons
    pub fn new(variants: impl IntoIterator<Item = (Box<dyn Widget<T>>, T)>) -> impl Widget<T> {
        let mut col = Flex::column();
        for (w, variant) in variants.into_iter() {
            let radio = Radio::new(w, variant);
            col.add_child(
                radio
                    .env_scope(move |env: &mut Env, data: &T| {
                        let color = if *data == variant {
                            env.get(PAINTR_TOGGLE_ON)
                        } else {
                            env.get(PAINTR_TOGGLE_OFF)
                        };

                        env.set(PAINTR_TOGGLE_FOREGROND, color);
                    })
                    .padding(5.0),
                0.0,
            );
        }
        col
    }
}

/// A single radio button
pub struct Radio<T: Data + PartialEq> {
    variant: T,
    child: WidgetPod<T, Box<dyn Widget<T>>>,
}

impl<T: Data + PartialEq + 'static> Radio<T> {
    /// Create a lone Radio button from label text and an enum variant
    pub fn new(w: impl Widget<T> + 'static, variant: T) -> impl Widget<T> {
        let radio = Self { variant, child: WidgetPod::new(w).boxed() };
        Align::vertical(UnitPoint::LEFT, radio)
    }
}

impl<T: Data + PartialEq> Widget<T> for Radio<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, _env: &Env) {
        match event {
            Event::MouseDown(_) => {
                ctx.set_active(true);
                ctx.invalidate();
            }
            Event::MouseUp(_) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    if ctx.is_hot() {
                        *data = self.variant.clone();
                    }
                    ctx.invalidate();
                }
            }
            Event::HotChanged(_) => {
                ctx.invalidate();
            }
            _ => (),
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: Option<&T>, _data: &T, _env: &Env) {
        ctx.invalidate();
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        env: &Env,
    ) -> Size {
        let size = self.child.layout(layout_ctx, &bc, data, env);
        self.child.set_layout_rect(Rect::from_origin_size(Point::ORIGIN, size));
        bc.constrain(size)
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &T, env: &Env) {
        let size = paint_ctx.size();
        let border_color =
            if paint_ctx.is_hot() { env.get(theme::BORDER_LIGHT) } else { env.get(theme::BORDER) };

        let rt = Rect::from_origin_size(Point::ORIGIN, size);

        // Check if data enum matches our variant
        if *data == self.variant {
            paint_ctx.fill(rt, &env.get(theme::LABEL_COLOR));
        }
        paint_ctx.stroke(rt, &border_color, 1.0);

        // Paint the text label
        self.child.paint_with_offset(paint_ctx, data, env);
    }
}
