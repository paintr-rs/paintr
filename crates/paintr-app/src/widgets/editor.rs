use druid::{BoxConstraints, Env, Event, EventCtx, LayoutCtx, PaintCtx, Size, UpdateCtx, Widget};

use super::canvas::Canvas;
use crate::tools::ToolCtx;
use crate::EditorState;
use paintr::Paintable;

#[derive(Debug)]
pub struct Editor {
    tool_ctx: Option<ToolCtx>,
    canvas: Canvas,
}

impl Editor {
    pub fn new() -> Editor {
        Editor { canvas: Canvas::new(), tool_ctx: None }
    }
}

impl Widget<EditorState> for Editor {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EditorState, env: &Env) {
        let tool = data.tool;
        tool.event(ctx, event, data, &mut self.tool_ctx);

        self.canvas.event(ctx, event, &mut data.canvas, env);
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: Option<&EditorState>,
        data: &EditorState,
        env: &Env,
    ) {
        self.canvas.update(ctx, old_data.as_ref().map(|it| &it.canvas), &data.canvas, env)
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &EditorState,
        env: &Env,
    ) -> Size {
        self.canvas.layout(layout_ctx, bc, &data.canvas, env)
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &EditorState, env: &Env) {
        self.canvas.paint(paint_ctx, &data.canvas, env);

        if let Some(canvas) = &data.canvas {
            if let Some(selection) = canvas.selection().as_ref() {
                selection.paint(paint_ctx.render_ctx);
            }
        }
    }
}
