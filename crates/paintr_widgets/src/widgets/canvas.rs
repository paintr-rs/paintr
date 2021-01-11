use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size,
    UpdateCtx, Widget,
};
use paintr_core::{CanvasData, Paintable};

#[derive(Debug)]
pub struct Canvas {}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas {}
    }
}

type DataType = Option<CanvasData>;

impl Widget<DataType> for Canvas {
    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &DataType,
        _env: &Env,
    ) {
    }

    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut DataType, _env: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &DataType, data: &DataType, _env: &Env) {
        if !is_same(old_data, data) {
            ctx.request_paint();
        }
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
        if let Some(canvas) = &data {
            canvas.paint(paint_ctx.render_ctx);
        }
    }
}

fn is_same(a: &DataType, b: &DataType) -> bool {
    match (a, b) {
        (None, None) => true,
        (None, Some(_)) => false,
        (Some(_), None) => false,
        (Some(a), Some(b)) => a.same(b),
    }
}
