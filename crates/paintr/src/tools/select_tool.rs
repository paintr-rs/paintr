use std::any::Any;

use crate::tools::ToolKind;
use crate::widgets::{Tool, ToolCtx};
use crate::EditorState;

use druid::{Cursor, Data, Event, EventCtx, MouseButton, Point, Rect};
use paintr_core::{CanvasData, Selection};

#[derive(Debug)]
pub(crate) struct SelectTool;

#[derive(Debug, Clone, Data)]
pub(crate) enum SelectToolCtx {
    New { down: Point },
    Move { down: Point, old: Selection },
}

impl SelectToolCtx {
    fn from_point(canvas: &Option<CanvasData>, pt: Point) -> Option<SelectToolCtx> {
        if let Some(sel) = canvas.as_ref()?.selection() {
            if sel.contains(pt) {
                return Some(SelectToolCtx::Move { down: pt, old: sel.clone() });
            }
        }
        Some(SelectToolCtx::New { down: pt })
    }

    fn moved(&mut self, canvas: &mut Option<CanvasData>, pt: Point) -> Option<()> {
        let canvas = canvas.as_mut()?;

        match self {
            SelectToolCtx::New { down } => {
                canvas.select(Rect::from_points(*down, pt));
            }
            SelectToolCtx::Move { down, old } => {
                let offset = pt.to_vec2() - down.to_vec2();
                canvas.select(old.transform(offset));
            }
        }

        Some(())
    }
}

impl Tool for SelectTool {
    type Context = SelectToolCtx;
    type Kind = ToolKind;

    fn event(
        &self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut EditorState<ToolKind>,
        tool_ctx: &mut Option<SelectToolCtx>,
    ) {
        if data.cursor != Some(Cursor::Crosshair) {
            data.cursor = Some(Cursor::Crosshair);
            ctx.set_cursor(&Cursor::Crosshair);
        }

        match event {
            Event::MouseDown(me) => {
                if me.button == MouseButton::Left {
                    ctx.set_active(true);
                    *tool_ctx = SelectToolCtx::from_point(&mut data.canvas, me.pos);
                }
            }
            Event::MouseMove(me) => {
                if let Some(tool_ctx) = tool_ctx.as_mut() {
                    if tool_ctx.moved(&mut data.canvas, me.pos).is_some() {
                        ctx.request_paint();
                    }
                }
            }
            Event::MouseUp(me) => {
                if me.button == MouseButton::Left {
                    if let Some(mut tool_ctx) = tool_ctx.take() {
                        if tool_ctx.moved(&mut data.canvas, me.pos).is_some() {
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

impl ToolCtx for SelectToolCtx {
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
