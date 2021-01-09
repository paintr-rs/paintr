use druid::{Data, Event, EventCtx, MouseButton, Point};
use paintr::{
    actions::{MoveCanvas, MoveSelection},
    EditKind,
};

use super::Tool;
use crate::EditorState;

#[derive(Debug)]
pub(crate) struct MoveTool;

#[derive(Debug, Clone, Data, Eq, PartialEq)]
enum MoveKind {
    Selection,
    WholeCanvas,
}

#[derive(Debug, Clone, Data)]
pub(crate) struct MoveToolCtx {
    kind: MoveKind,
    down: Point,
    origin: Point,
    curr: Point,
}

impl MoveToolCtx {
    fn from_point(editor: &mut EditorState, pt: Point) -> Option<Self> {
        let canvas = editor.canvas.as_mut()?;
        let mut origin = canvas.position();
        let mut kind = MoveKind::WholeCanvas;

        if let Some(sel) = canvas.selection() {
            if sel.contains(pt) {
                kind = MoveKind::Selection;
                origin = sel.position();
            }
        }

        Some(Self { kind, down: pt, origin, curr: origin })
    }

    fn moved(&mut self, editor: &mut EditorState, pt: Point, kind: EditKind) -> Option<()> {
        let target = (pt.to_vec2() - self.down.to_vec2()) + self.origin.to_vec2();

        match self.kind {
            MoveKind::Selection => {
                if editor.canvas.as_ref().map(|it| it.selection()).is_none() {
                    return None;
                }
                editor.do_edit(MoveSelection::new(target - self.curr.to_vec2()), kind);
                self.curr = editor.canvas.as_ref()?.selection()?.position();
                assert_eq!(self.curr, target.to_point());
            }
            MoveKind::WholeCanvas => {
                if editor.canvas.is_none() {
                    return None;
                }
                editor.do_edit(MoveCanvas::new(target - self.curr.to_vec2()), kind);
                self.curr = editor.canvas.as_ref()?.position();
                assert_eq!(self.curr, target.to_point());
            }
        }

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
            Event::MouseMove(me) => {
                if let Some(tool_ctx) = tool_ctx.as_mut() {
                    if tool_ctx.moved(data, me.pos, EditKind::Mergeable).is_some() {
                        ctx.request_paint();
                    }
                }
            }
            Event::MouseUp(me) => {
                if me.button == MouseButton::Left {
                    if let Some(mut tool_ctx) = tool_ctx.take() {
                        if tool_ctx.moved(data, me.pos, EditKind::NonMergeable).is_some() {
                            ctx.request_paint();
                        }
                    }
                    ctx.set_active(false);
                }
            }
            _ => (),
        };
    }
}
