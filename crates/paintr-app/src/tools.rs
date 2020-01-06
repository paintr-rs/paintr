use druid::{Data, Event, EventCtx, MouseButton, Point, Rect};
use paintr::{CanvasData, Selection};

#[derive(Debug, Clone, Data, PartialEq, Eq, Copy)]
pub(crate) enum Tool {
    Move,
    Select,
}

#[derive(Debug, Clone, Data)]
pub(crate) enum ToolCtx {
    #[allow(dead_code)]
    Move(()),
    Select(SelectToolCtx),
}

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

impl SelectTool {
    pub(crate) fn event(
        &self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut Option<CanvasData>,
        mut tool_ctx: Option<SelectToolCtx>,
    ) -> Option<SelectToolCtx> {
        match event {
            Event::MouseDown(me) => {
                if me.button == MouseButton::Left {
                    ctx.set_active(true);
                    tool_ctx = SelectToolCtx::from_point(data, me.pos);
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

        tool_ctx
    }
}

macro_rules! downcast {
    ($ctx:expr, $id:ident) => {
        $ctx.and_then(|ctx| match ctx {
            ToolCtx::$id(it) => Some(it),
            _ => None,
        })
    };
}

impl Tool {
    pub(crate) fn event(
        &self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut Option<CanvasData>,
        tool_ctx: Option<ToolCtx>,
    ) -> Option<ToolCtx> {
        match self {
            Tool::Move => None,
            Tool::Select => {
                SelectTool.event(ctx, event, data, downcast!(tool_ctx, Select)).map(ToolCtx::Select)
            }
        }
    }
}
