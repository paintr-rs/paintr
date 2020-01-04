use druid::{Data, PaintCtx, Size};
use image::DynamicImage;

use crate::edit::{Edit, EditDesc};
use crate::plane::Planes;
use crate::{Paintable, Selection};

use std::sync::Arc;

// FIXME: Change name to Layer
#[derive(Data, Clone)]
pub struct CanvasData {
    path: Arc<std::path::PathBuf>,
    selection: Option<Selection>,
    planes: Planes,
}

impl CanvasData {
    pub fn new(path: impl Into<std::path::PathBuf>, img: Arc<DynamicImage>) -> CanvasData {
        let mut planes = Planes::new();
        planes.push(img.clone());
        CanvasData { selection: None, planes, path: Arc::new(path.into()) }
    }

    pub fn path(&self) -> &std::path::Path {
        self.path.as_ref()
    }

    pub fn save(&self, path: &std::path::Path) -> Result<Arc<DynamicImage>, std::io::Error> {
        let img = self.image();
        img.save(path)?;
        Ok(img)
    }

    pub fn selection(&self) -> Option<&Selection> {
        self.selection.as_ref()
    }

    pub fn image(&self) -> Arc<DynamicImage> {
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
}

impl Paintable for CanvasData {
    fn paint(&self, paint_ctx: &mut PaintCtx) {
        self.planes.paint(paint_ctx);

        if let Some(selection) = self.selection.as_ref() {
            selection.paint(paint_ctx);
        }
    }

    fn paint_size(&self) -> Option<Size> {
        self.planes.paint_size()
    }
}

pub struct Paste {
    img: DynamicImage,
}

impl Paste {
    pub fn new(img: DynamicImage) -> Paste {
        Paste { img }
    }
}

#[must_use]
impl Edit<CanvasData> for Paste {
    fn apply(&self, data: &mut CanvasData) {
        data.planes.push(Arc::new(self.img.clone()));
    }

    fn description(&self) -> EditDesc {
        EditDesc::new("Paste")
    }
}
