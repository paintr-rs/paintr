use std::{any::Any, sync::Arc};

use druid::{Cursor, Data, Event, EventCtx, MouseButton, Point};
use paintr_core::{actions::DrawBrush, EditKind};

use crate::tools::ToolKind;
use crate::widgets::{Tool, ToolCtx};
use crate::EditorState;

#[derive(Debug)]
pub(crate) struct BrushTool;

#[derive(Debug, Clone, Data)]
pub(crate) struct BrushToolCtx {
    origin: Point,
    cursor: Option<Cursor>,
    drawn: Arc<Vec<Point>>,
}

impl BrushToolCtx {
    fn from_point<T>(editor: &mut EditorState<T>, pt: Point) -> Option<Self> {
        let canvas = editor.canvas.as_mut()?;
        let mut origin = canvas.position();

        if let Some(sel) = canvas.selection() {
            if sel.contains(pt) {
                origin = sel.position();
            }
        }

        Some(Self { origin, cursor: None, drawn: Arc::new(vec![]) })
    }

    fn draw<T>(&mut self, editor: &mut EditorState<T>, pt: Point, kind: EditKind) -> Option<()> {
        let target = pt.to_vec2();

        if editor.canvas.is_none() {
            return None;
        }
        editor.do_edit(DrawBrush::new(vec![target]), kind);

        Some(())
    }
}

impl Tool for BrushTool {
    type Context = BrushToolCtx;
    type Kind = ToolKind;

    fn event(
        &self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut EditorState<ToolKind>,
        tool_ctx: &mut Option<BrushToolCtx>,
    ) {
        if data.cursor != Some(Cursor::Arrow) {
            data.cursor = Some(Cursor::Arrow);
            ctx.set_cursor(&Cursor::Arrow);
        }

        match event {
            Event::MouseDown(me) => {
                if me.button == MouseButton::Left {
                    ctx.set_active(true);
                    *tool_ctx = BrushToolCtx::from_point(data, me.pos);
                }
            }
            Event::MouseMove(me) => {
                if let Some(tool_ctx) = tool_ctx.as_mut() {
                    if tool_ctx.draw(data, me.pos, EditKind::Mergeable).is_some() {
                        ctx.request_paint();
                    }
                }
            }
            Event::MouseUp(me) => {
                if me.button == MouseButton::Left {
                    if let Some(mut tool_ctx) = tool_ctx.take() {
                        if tool_ctx.draw(data, me.pos, EditKind::NonMergeable).is_some() {
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

impl ToolCtx for BrushToolCtx {
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
