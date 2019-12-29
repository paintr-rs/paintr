use druid::kurbo::{Point, Rect, Size};
use druid::piet::{Color, ImageFormat, InterpolationMode, RenderContext, StrokeStyle};
use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, MouseButton, PaintCtx, UpdateCtx, Widget,
};

use image::{DynamicImage, GenericImageView, RgbaImage};
use std::sync::Arc;

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

pub trait Paintable: std::fmt::Debug {
    fn paint(&self, paint_ctx: &mut PaintCtx);
    fn paint_size(&self) -> Size;
}

impl Paintable for RgbaImage {
    fn paint(&self, paint_ctx: &mut PaintCtx) {
        let size = (self.width() as usize, self.height() as usize);

        // FIXME: Draw image only in paint_ctx.region
        let image = paint_ctx.make_image(size.0, size.1, self, ImageFormat::RgbaSeparate).unwrap();
        // The image is automatically scaled to fit the rect you pass to draw_image
        paint_ctx.draw_image(
            &image,
            Rect::from_origin_size(Point::ORIGIN, self.paint_size()),
            InterpolationMode::NearestNeighbor,
        );
    }

    fn paint_size(&self) -> Size {
        (self.width() as f64, self.height() as f64).into()
    }
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

#[derive(Debug, Clone)]
struct Selection {
    rect: Rect,
}

impl Data for Selection {
    fn same(&self, other: &Self) -> bool {
        self.rect.size() == other.rect.size() && self.rect.origin() == self.rect.origin()
    }
}

impl Paintable for Selection {
    fn paint(&self, paint_ctx: &mut PaintCtx) {
        let path = self.rect;
        // Create a color
        let stroke_color = Color::rgb8(0xff, 0xff, 0xff);

        let mut style = StrokeStyle::new();
        let dashes = vec![0.0, 0.0, 2.0, 4.0];
        style.set_dash(dashes, 0.0);

        paint_ctx.stroke_styled(path, &stroke_color, 2.0, &style);
    }

    fn paint_size(&self) -> Size {
        self.rect.size()
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

    fn update_selection(&mut self, data: &mut CanvasData, pt0: Point, pt1: Point) {
        let rect = Rect::from_points(pt0, pt1);
        if rect.size() == Size::ZERO {
            data.selection = None;
        } else {
            data.selection = Some(Selection { rect });
        }
    }

    fn set_selection(&mut self, selection: Option<Selection>) {
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

#[derive(Data, Clone)]
pub struct CanvasData {
    img: Arc<DynamicImage>,
    selection: Option<Selection>,
}

impl CanvasData {
    pub fn new(img: Arc<DynamicImage>) -> CanvasData {
        CanvasData { img, selection: None }
    }

    pub fn save(&self, path: &std::path::Path) -> Result<Arc<DynamicImage>, std::io::Error> {
        if let Some(sel_img) = self.selection() {
            sel_img.save(path)?;
            Ok(sel_img)
        } else {
            self.img.save(path)?;
            Ok(self.img.clone())
        }
    }

    pub fn selection(&self) -> Option<Arc<DynamicImage>> {
        let selection = self.selection.as_ref()?;
        let (x, y) = selection.rect.origin().into();
        let (w, h) = selection.rect.size().into();
        let new_img = self.img.view(x as u32, y as u32, w as u32, h as u32).to_image();
        Some(Arc::new(DynamicImage::ImageRgba8(new_img)))
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
                        self.update_selection(data, down, me.pos);
                        ctx.invalidate();
                    }
                }
            }

            Event::MouseUp(me) => {
                if me.button == MouseButton::Left {
                    if let Some(down) = self.down.take() {
                        if let Some(data) = data {
                            self.update_selection(data, down, me.pos);
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
            let old_img = old_data.map(|it| it.as_ref().map(|it| &it.img)).flatten();
            let old_selection = old_data.map(|it| it.as_ref().map(|it| &it.selection)).flatten();

            let img_changed = !old_img.same(&data.as_ref().map(|it| &it.img));
            let selection_changed = !old_selection.same(&data.as_ref().map(|it| &it.selection));

            if img_changed {
                self.planes.clear();
                self.has_selection = None;
                if let Some(canvas) = data.as_ref() {
                    self.planes.push(canvas.img.to_rgba().into());
                }
            }

            if selection_changed || img_changed {
                if let Some(canvas) = data.as_ref() {
                    self.set_selection(canvas.selection.clone());
                } else {
                    self.set_selection(None);
                }
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
