use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, MouseButton, PaintCtx, Point, Rect, Size,
    UpdateCtx, Widget,
};

use paintr::{CanvasData, Paintable};

#[derive(Debug)]
pub struct Canvas {
    down: Option<Point>,
}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas { down: None }
    }
}

type DataType = Option<CanvasData>;

impl Widget<DataType> for Canvas {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut DataType, _env: &Env) {
        match event {
            Event::MouseDown(me) => {
                if me.button == MouseButton::Left {
                    self.down = Some(me.pos)
                }
            }
            Event::MouseMoved(me) => {
                if let Some(down) = self.down {
                    if let Some(data) = data {
                        data.select_rect(Rect::from_points(down, me.pos));
                        ctx.invalidate();
                    }
                }
            }
            Event::MouseUp(me) => {
                if me.button == MouseButton::Left {
                    if let Some(down) = self.down.take() {
                        if let Some(data) = data {
                            data.select_rect(Rect::from_points(down, me.pos));
                            ctx.invalidate();
                        }
                    }
                }
            }
            _ => (),
        }
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: Option<&DataType>,
        _data: &DataType,
        _env: &Env,
    ) {
        ctx.invalidate()
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &DataType,
        _env: &Env,
    ) -> Size {
        data.as_ref().and_then(|data| data.paint_size()).unwrap_or_else(|| bc.max())
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &DataType, _env: &Env) {
        if let Some(data) = data {
            data.paint(paint_ctx);
        }
    }
}
