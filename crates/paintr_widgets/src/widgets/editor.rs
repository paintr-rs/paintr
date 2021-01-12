use std::any::Any;

use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size,
    UpdateCtx, Widget,
};

use super::canvas::Canvas;
use crate::EditorState;
use paintr_core::Paintable;

pub trait ToolCtx {
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

pub trait Tool {
    type Context: ToolCtx + 'static;
    type Kind;

    fn event(
        &self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut EditorState<Self::Kind>,
        tool_ctx: &mut Option<Self::Context>,
    );

    fn do_event(
        &self,
        tctx: &mut Option<Box<dyn ToolCtx>>,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut EditorState<Self::Kind>,
    ) {
        let mut tool_ctx = tctx.take().and_then(|it| Some(*it.into_any().downcast().ok()?));
        self.event(ctx, event, data, &mut tool_ctx);
        *tctx = tool_ctx.map(|it| {
            let b: Box<dyn ToolCtx> = Box::new(it);
            b
        });
    }
}

pub trait ToolKind: Copy {
    fn event(
        &self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut EditorState<Self>,
        tool_ctx: &mut Option<Box<dyn ToolCtx>>,
    );
}

pub struct Editor {
    tool_ctx: Option<Box<dyn ToolCtx>>,
    canvas: Canvas,
}

impl Editor {
    pub fn new() -> Editor {
        Editor { canvas: Canvas::new(), tool_ctx: None }
    }
}

impl<T> Widget<EditorState<T>> for Editor
where
    T: ToolKind,
{
    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &EditorState<T>,
        _env: &Env,
    ) {
    }

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EditorState<T>, env: &Env) {
        let tool = data.tool;
        tool.event(ctx, event, data, &mut self.tool_ctx);

        self.canvas.event(ctx, event, &mut data.canvas, env);
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &EditorState<T>,
        data: &EditorState<T>,
        env: &Env,
    ) {
        self.canvas.update(ctx, &old_data.canvas, &data.canvas, env)
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &EditorState<T>,
        env: &Env,
    ) -> Size {
        self.canvas.layout(layout_ctx, bc, &data.canvas, env)
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &EditorState<T>, env: &Env) {
        self.canvas.paint(paint_ctx, &data.canvas, env);

        if let Some(canvas) = &data.canvas {
            if let Some(selection) = canvas.selection().as_ref() {
                selection.paint(paint_ctx);
            }
        }
    }
}
