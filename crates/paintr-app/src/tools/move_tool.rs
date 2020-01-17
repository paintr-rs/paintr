use druid::{Data, Event, EventCtx, MouseButton, Point, Vec2};
use paintr::CanvasData;

use super::Tool;
use crate::EditorState;

#[derive(Debug)]
pub(crate) struct MoveTool;

#[derive(Debug, Clone, Data)]
pub(crate) struct MoveToolCtx {
    down: Point,
    origin: Point,
    curr: Point,
}

impl MoveToolCtx {
    fn from_point(canvas: &mut Option<CanvasData>, pt: Point) -> Option<Self> {
        let canvas = canvas.as_mut()?;
        let origin = canvas.mov(Vec2::ZERO)?;
        Some(Self { down: pt, origin, curr: origin })
    }

    fn moved(&mut self, canvas: &mut Option<CanvasData>, pt: Point) -> Option<()> {
        let canvas = canvas.as_mut()?;
        let target = (pt.to_vec2() - self.down.to_vec2()) + self.origin.to_vec2();
        self.curr = canvas.mov(target - self.curr.to_vec2())?;
        assert_eq!(self.curr, target.to_point());

        Some(())
    }
}

impl Tool for MoveTool {
    type Context = MoveToolCtx;

    fn event(
        &self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut EditorState,
        tool_ctx: &mut Option<MoveToolCtx>,
    ) {
        match event {
            Event::MouseDown(me) => {
                if me.button == MouseButton::Left {
                    ctx.set_active(true);
                    *tool_ctx = MoveToolCtx::from_point(&mut data.canvas, me.pos);
                }
            }
            Event::MouseMoved(me) => {
                if let Some(tool_ctx) = tool_ctx.as_mut() {
                    if tool_ctx.moved(&mut data.canvas, me.pos).is_some() {
                        ctx.invalidate();
                    }
                }
            }
            Event::MouseUp(me) => {
                if me.button == MouseButton::Left {
                    if let Some(mut tool_ctx) = tool_ctx.take() {
                        if tool_ctx.moved(&mut data.canvas, me.pos).is_some() {
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
