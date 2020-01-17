use druid::{BoxConstraints, Env, Event, EventCtx, LayoutCtx, PaintCtx, Size, UpdateCtx, Widget};
use paintr::{CanvasData, Paintable};

#[derive(Debug)]
pub struct Canvas {}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas {}
    }
}

type DataType = Option<CanvasData>;

impl Widget<DataType> for Canvas {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut DataType, _env: &Env) {}

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
        if let Some(canvas) = &data {
            canvas.paint(paint_ctx.render_ctx);
        }
    }
}
