use druid::{BoxConstraints, Env, Event, EventCtx, LayoutCtx, PaintCtx, Size, UpdateCtx, Widget};

use crate::tools::{Tool, ToolCtx};
use paintr::{CanvasData, Paintable};

#[derive(Debug)]
pub struct Canvas {
    tool_ctx: Option<ToolCtx>,
}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas { tool_ctx: None }
    }
}

type DataType = (Option<CanvasData>, Tool);

impl Widget<DataType> for Canvas {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut DataType, _env: &Env) {
        self.tool_ctx = data.1.event(ctx, event, &mut data.0, self.tool_ctx.take())
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
        data.0.as_ref().and_then(|data| data.paint_size()).unwrap_or_else(|| bc.max())
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &DataType, _env: &Env) {
        if let Some(data) = &data.0 {
            data.paint(paint_ctx);
        }
    }
}
