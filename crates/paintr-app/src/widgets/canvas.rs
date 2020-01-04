use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, MouseButton, PaintCtx, Point, Rect, Size,
    UpdateCtx, Widget,
};

use paintr::{CanvasData, Paintable, Selection};

#[derive(Debug)]
enum EditMode {
    New { down: Point },
    Move { down: Point, old: Selection },
}

impl EditMode {
    fn from_point(canvas: &Option<CanvasData>, pt: Point) -> Option<EditMode> {
        if let Some(sel) = canvas.as_ref()?.selection() {
            if sel.contains(pt) {
                return Some(EditMode::Move { down: pt, old: sel.clone() });
            }
        }

        Some(EditMode::New { down: pt })
    }

    fn moved(&mut self, canvas: &mut Option<CanvasData>, pt: Point) -> Option<()> {
        let canvas = canvas.as_mut()?;

        match self {
            EditMode::New { down } => {
                canvas.select(Rect::from_points(*down, pt));
            }
            EditMode::Move { down, old } => {
                let offset = pt.to_vec2() - down.to_vec2();
                canvas.select(old.transform(offset));
            }
        }

        Some(())
    }
}

#[derive(Debug)]
pub struct Canvas {
    mode: Option<EditMode>,
}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas { mode: None }
    }
}

type DataType = Option<CanvasData>;

impl Widget<DataType> for Canvas {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut DataType, _env: &Env) {
        match event {
            Event::MouseDown(me) => {
                if me.button == MouseButton::Left {
                    self.mode = EditMode::from_point(data, me.pos);
                    ctx.set_active(true);
                }
            }
            Event::MouseMoved(me) => {
                if let Some(mode) = self.mode.as_mut() {
                    if mode.moved(data, me.pos).is_some() {
                        ctx.invalidate();
                    }
                }
            }
            Event::MouseUp(me) => {
                if me.button == MouseButton::Left {
                    if let Some(mut mode) = self.mode.take() {
                        if mode.moved(data, me.pos).is_some() {
                            ctx.invalidate();
                        }
                    }
                    ctx.set_active(false);
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
