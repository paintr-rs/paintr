mod move_tool;
mod select_tool;

use crate::EditorState;
use druid::{Data, Event, EventCtx};
use move_tool::MoveTool;
use paintr::impl_from;
use select_tool::SelectTool;

pub(crate) trait Tool {
    type Context: FromToolCtx + Into<ToolCtx>;

    fn event(
        &self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut EditorState,
        tool_ctx: &mut Option<Self::Context>,
    );

    fn do_event(
        &self,
        tctx: &mut Option<ToolCtx>,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut EditorState,
    ) {
        let mut tool_ctx = tctx.take().and_then(|it| Self::Context::from_ctx(it));
        self.event(ctx, event, data, &mut tool_ctx);
        *tctx = tool_ctx.map(|it| it.into());
    }
}

#[derive(Debug, Clone, Data, PartialEq, Eq, Copy)]
pub(crate) enum ToolKind {
    Move,
    Select,
}

macro_rules! impl_tool_ctx {
    ($($tool:ident => $kind:ident),*) => {
        #[derive(Debug, Clone, Data)]
        pub(crate) enum ToolCtx {
            $(
                $kind(<$tool as Tool>::Context)
            ),*
        }

        impl_from! {
            ToolCtx : [
                $(
                    <$tool as Tool>::Context => $kind
                ),*
            ]
        }

        $(
        impl FromToolCtx for <$tool as Tool>::Context {
            fn from_ctx(ctx: ToolCtx) -> Option<Self> {
                match ctx {
                    ToolCtx::$kind(it) => Some(it),
                    _ => None,
                }
            }
        })*
    }
}

pub(crate) trait FromToolCtx {
    fn from_ctx(ctx: ToolCtx) -> Option<Self>
    where
        Self: Sized;
}

impl_tool_ctx! {
    MoveTool => Move,
    SelectTool => Select
}

impl ToolKind {
    pub(crate) fn event(
        &self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut EditorState,
        tool_ctx: &mut Option<ToolCtx>,
    ) {
        match self {
            ToolKind::Move => MoveTool.do_event(tool_ctx, ctx, event, data),
            ToolKind::Select => SelectTool.do_event(tool_ctx, ctx, event, data),
        }
    }
}
