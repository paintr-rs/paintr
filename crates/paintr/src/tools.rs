mod move_tool;
mod select_tool;

use crate::widgets::Tool;
use crate::widgets::ToolCtx;
use crate::EditorState;
use druid::{Data, Event, EventCtx};
use move_tool::MoveTool;
use select_tool::SelectTool;

macro_rules! register_tool {
    ($($e:ident => $tool:path),* $(,)?) => {
        #[derive(Debug, Clone, Data, PartialEq, Eq, Copy)]
        pub(crate) enum ToolKind {
            $($e),*
        }

        impl crate::widgets::ToolKind for ToolKind {
            fn event(
                &self,
                ctx: &mut EventCtx,
                event: &Event,
                data: &mut EditorState<ToolKind>,
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
