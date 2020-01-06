mod move_tool;
mod select_tool;

use druid::{Data, Event, EventCtx};
use move_tool::MoveTool;
use paintr::impl_from;
use paintr::CanvasData;
use select_tool::SelectTool;

pub trait Tool {
    type Context;

    fn event(
        &self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut Option<CanvasData>,
        tool_ctx: &mut Option<Self::Context>,
    );
}

#[derive(Debug, Clone, Data, PartialEq, Eq, Copy)]
pub(crate) enum ToolKind {
    Move,
    Select,
}

#[derive(Debug, Clone, Data)]
pub(crate) enum ToolCtx {
    Move(<MoveTool as Tool>::Context),
    Select(<SelectTool as Tool>::Context),
}

impl_from! {
    ToolCtx : [
        <SelectTool as Tool>::Context => Select,
        <MoveTool as Tool>::Context => Move
    ]

}

macro_rules! downcast {
    ($ctx:expr, $id:ident) => {
        $ctx.take().and_then(|ctx| match ctx {
            ToolCtx::$id(it) => Some(it),
            _ => None,
        })
    };
}

impl ToolKind {
    pub(crate) fn event(
        &self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut Option<CanvasData>,
        tool_ctx: &mut Option<ToolCtx>,
    ) {
        match self {
            ToolKind::Move => {
                let mut new_tool_ctx = downcast!(tool_ctx, Move);
                MoveTool.event(ctx, event, data, &mut new_tool_ctx);
                *tool_ctx = new_tool_ctx.map(|it| it.into());
            }
            ToolKind::Select => {
                let mut new_tool_ctx = downcast!(tool_ctx, Select);
                SelectTool.event(ctx, event, data, &mut new_tool_ctx);
                *tool_ctx = new_tool_ctx.map(|it| it.into());
            }
        }
    }
}
