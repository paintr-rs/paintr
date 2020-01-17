use druid::{Data, Point, RenderContext, Size, Vec2};

use crate::plane::Planes;
use crate::{Paintable, Selection};

use std::sync::Arc;

// FIXME: Change name to Layer
#[derive(Data, Clone)]
pub struct CanvasData {
    pub(crate) path: Arc<std::path::PathBuf>,
    pub(crate) selection: Option<Selection>,
    pub(crate) planes: Planes,
}

impl CanvasData {
    pub fn new(path: impl Into<std::path::PathBuf>, img: image::RgbaImage) -> CanvasData {
        let mut planes = Planes::new();
        planes.push(Arc::new(image::ImageRgba8(img)));
        CanvasData { selection: None, planes, path: Arc::new(path.into()) }
    }

    pub fn path(&self) -> &std::path::Path {
        self.path.as_ref()
    }

    pub fn save(&mut self, path: &std::path::Path) -> Result<(), std::io::Error> {
        let img = self.image();
        img.save(path)?;
        self.path = Arc::new(path.into());
        Ok(())
    }

    pub fn selection(&self) -> Option<&Selection> {
        self.selection.as_ref()
    }

    pub fn image(&self) -> Arc<image::DynamicImage> {
        self.planes.merged().expect("There is at least plane in Canvas")
    }

    pub fn select(&mut self, sel: impl Into<Selection>) {
        let sel = sel.into();
        if sel.size() == Size::ZERO {
            self.selection = None;
        } else {
            self.selection = Some(sel);
        }
    }

    //FIXME: should be move layer, when we implemented layer
    pub(crate) fn mov(&mut self, offset: Vec2) -> Option<Point> {
        self.planes.mov(offset)
    }

    pub fn position(&self) -> Option<Point> {
        self.planes.position()
    }
}

impl Paintable for CanvasData {
    fn paint(&self, render_ctx: &mut impl RenderContext) {
        self.planes.paint(render_ctx);
    }

    fn paint_size(&self) -> Option<Size> {
        self.planes.paint_size()
    }
}
