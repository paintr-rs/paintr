use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, MouseButton, PaintCtx, Point, Rect,
    Size, UpdateCtx, Widget,
};

use image::RgbaImage;
use paintr::{CanvasData, Paintable, Selection};

#[derive(Debug)]
enum Plane {
    Image(RgbaImage),
    Selection(Selection),
}

macro_rules! impl_from {
    ($trait:ident : [$($from:ident => $to:ident ),*] ) => {
        $(
            impl From<$from> for $trait {
                fn from(f: $from) -> $trait {
                    $trait::$to(f)
                }
            }
        )*
    }
}

impl_from! {
    Plane : [RgbaImage => Image, Selection => Selection]
}

impl Plane {
    fn paint(&self, paint_ctx: &mut PaintCtx) {
        match self {
            Plane::Image(it) => it.paint(paint_ctx),
            Plane::Selection(it) => it.paint(paint_ctx),
        };
    }

    fn size(&self) -> Size {
        match self {
            Plane::Image(it) => it.paint_size(),
            Plane::Selection(it) => it.paint_size(),
        }
    }
}

#[derive(Debug)]
struct PlaneIndex(usize);

#[derive(Debug)]
pub struct Canvas {
    planes: Vec<Plane>,
    down: Option<Point>,
    has_selection: Option<PlaneIndex>,
}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas { planes: Vec::new(), down: None, has_selection: None }
    }

    fn max_size(&self) -> Option<Size> {
        return self.planes.iter().fold(None, |acc, plane| {
            let size = plane.size();
            Some(match acc {
                None => size,
                Some(acc) => (acc.width.max(size.width), acc.height.max(size.height)).into(),
            })
        });
    }

    fn update_selection(&mut self, selection: Option<Selection>) {
        let selection = match selection {
            None => {
                if let Some(idx) = self.has_selection.take() {
                    self.planes.remove(idx.0);
                }
                return;
            }
            Some(selection) => selection,
        };

        match &self.has_selection {
            None => {
                self.planes.push(selection.into());
                self.has_selection = Some(PlaneIndex(self.planes.len() - 1));
            }
            Some(sel) => {
                self.planes[sel.0] = selection.into();
            }
        };
    }
}

type DataType = Option<CanvasData>;

impl Widget<DataType> for Canvas {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut DataType, _env: &Env) {
        match event {
            Event::MouseDown(me) => {
                if me.button == MouseButton::Left {
                    self.down = Some(me.pos)
                }
            }
            Event::MouseMoved(me) => {
                if let Some(down) = self.down {
                    if let Some(data) = data {
                        data.select_rect(Rect::from_points(down, me.pos));
                        ctx.invalidate();
                    }
                }
            }
            Event::MouseUp(me) => {
                if me.button == MouseButton::Left {
                    if let Some(down) = self.down.take() {
                        if let Some(data) = data {
                            data.select_rect(Rect::from_points(down, me.pos));
                            ctx.invalidate();
                        }
                    }
                }
            }
            _ => (),
        }
    }

    fn update(
        &mut self,
        _ctx: &mut UpdateCtx,
        old_data: Option<&DataType>,
        data: &DataType,
        _env: &Env,
    ) {
        let changed = match (old_data, data) {
            (Some(Some(old)), Some(new)) => !old.same(&new),
            _ => true,
        };

        if changed {
            let old_img = old_data.map(|it| it.as_ref().map(|it| it.image())).flatten();
            let old_selection = old_data.map(|it| it.as_ref().map(|it| it.selection())).flatten();

            let img_changed = !old_img.same(&data.as_ref().map(|it| it.image()));
            let selection_changed = !old_selection.same(&data.as_ref().map(|it| it.selection()));

            if img_changed {
                self.planes.clear();
                self.has_selection = None;
                if let Some(canvas) = data.as_ref() {
                    self.planes.push(canvas.image().to_rgba().into());
                }
            }

            if selection_changed || img_changed {
                let selection = data.as_ref().and_then(|c| c.selection().cloned());
                self.update_selection(selection);
            }
        }
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &DataType,
        _env: &Env,
    ) -> Size {
        self.max_size().map(|it| it.into()).unwrap_or_else(|| bc.max())
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, _data: &DataType, _env: &Env) {
        for plane in &self.planes {
            plane.paint(paint_ctx);
        }
    }
}
