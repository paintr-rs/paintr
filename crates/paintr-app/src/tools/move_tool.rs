use druid::{Data, Event, EventCtx, MouseButton, Point};
use paintr::CanvasData;

use super::Tool;

#[derive(Debug)]
pub(crate) struct MoveTool;

#[derive(Debug, Clone, Data)]
pub(crate) struct MoveToolCtx {
    down: Point,
}

impl MoveToolCtx {
    fn from_point(pt: Point) -> Option<Self> {
        Some(Self { down: pt })
    }

    fn moved(&mut self, canvas: &mut Option<CanvasData>, _pt: Point) -> Option<()> {
        let _canvas = canvas.as_mut()?;
        Some(())
    }
}

impl Tool for MoveTool {
    type Context = MoveToolCtx;

    fn event(
        &self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut Option<CanvasData>,
        tool_ctx: &mut Option<MoveToolCtx>,
    ) {
        match event {
            Event::MouseDown(me) => {
                if me.button == MouseButton::Left {
                    ctx.set_active(true);
                    *tool_ctx = MoveToolCtx::from_point(me.pos);
                }
            }
            Event::MouseMoved(me) => {
                if let Some(tool_ctx) = tool_ctx.as_mut() {
                    if tool_ctx.moved(data, me.pos).is_some() {
                        ctx.invalidate();
                    }
                }
            }
            Event::MouseUp(me) => {
                if me.button == MouseButton::Left {
                    if let Some(mut tool_ctx) = tool_ctx.take() {
                        if tool_ctx.moved(data, me.pos).is_some() {
                            ctx.invalidate();
                        }
                    }
                    ctx.set_active(false);
                }
            }
            _ => (),
        };
    }
}
