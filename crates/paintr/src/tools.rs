mod move_tool;
mod select_tool;

use std::any::Any;

use crate::EditorState;
use druid::{Data, Event, EventCtx};
use move_tool::MoveTool;
use select_tool::SelectTool;

pub trait ToolCtx {
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

pub(crate) trait Tool {
    type Context: ToolCtx + 'static;

    fn event(
        &self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut EditorState,
        tool_ctx: &mut Option<Self::Context>,
    );

    fn do_event(
        &self,
        tctx: &mut Option<Box<dyn ToolCtx>>,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut EditorState,
    ) {
        let mut tool_ctx = tctx.take().and_then(|it| Some(*it.into_any().downcast().ok()?));
        self.event(ctx, event, data, &mut tool_ctx);
        *tctx = tool_ctx.map(|it| {
            let b: Box<dyn ToolCtx> = Box::new(it);
            b
        });
    }
}

macro_rules! register_tool {
    ($($e:ident => $tool:path),* $(,)?) => {
        #[derive(Debug, Clone, Data, PartialEq, Eq, Copy)]
        pub(crate) enum ToolKind {
            $($e),*
        }

        impl ToolKind {
            pub(crate) fn event(
                &self,
                ctx: &mut EventCtx,
                event: &Event,
                data: &mut EditorState,
                tool_ctx: &mut Option<Box<dyn ToolCtx>>,
            ) {
                match self {
                    $(ToolKind::$e => $tool.do_event(tool_ctx, ctx, event, data),)*
                }
            }
        }
    };
}

register_tool! {
    Move => MoveTool,
    Select => SelectTool
}
