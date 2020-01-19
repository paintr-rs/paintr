use druid::{Data, Event, EventCtx, MouseButton, Point};
use paintr::{actions::Move, EditKind};

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
    fn from_point(editor: &mut EditorState, pt: Point) -> Option<Self> {
        let canvas = editor.canvas.as_mut()?;
        let origin = canvas.position();
        Some(Self { down: pt, origin, curr: origin })
    }

    fn moved(&mut self, editor: &mut EditorState, pt: Point, kind: EditKind) -> Option<()> {
        if editor.canvas.is_none() {
            return None;
        }

        let target = (pt.to_vec2() - self.down.to_vec2()) + self.origin.to_vec2();
        editor.do_edit(Move::new(target - self.curr.to_vec2()), kind);
        self.curr = editor.canvas.as_ref()?.position();
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
                    *tool_ctx = MoveToolCtx::from_point(data, me.pos);
                }
            }
            Event::MouseMoved(me) => {
                if let Some(tool_ctx) = tool_ctx.as_mut() {
                    if tool_ctx.moved(data, me.pos, EditKind::Mergeable).is_some() {
                        ctx.invalidate();
                    }
                }
            }
            Event::MouseUp(me) => {
                if me.button == MouseButton::Left {
                    if let Some(mut tool_ctx) = tool_ctx.take() {
                        if tool_ctx.moved(data, me.pos, EditKind::NonMergeable).is_some() {
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
