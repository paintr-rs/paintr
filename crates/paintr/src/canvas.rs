use druid::{Data, PaintCtx, Rect, Size};
use image::DynamicImage;

use crate::plane::Planes;
use crate::{Paintable, Selection};

use std::sync::Arc;

// FIXME: Change name to Layer
#[derive(Data, Clone)]
pub struct CanvasData {
    img: Arc<DynamicImage>,
    selection: Option<Selection>,
    planes: Planes,
}

impl CanvasData {
    pub fn new(img: Arc<DynamicImage>) -> CanvasData {
        let mut planes = Planes::new();
        planes.push(img.clone());
        CanvasData { img, selection: None, planes }
    }

    pub fn save(&self, path: &std::path::Path) -> Result<Arc<DynamicImage>, std::io::Error> {
        if let Some(sel) = self.selection() {
            let sel_img = sel.image(&self.img);
            sel_img.save(path)?;
            Ok(sel_img)
        } else {
            self.img.save(path)?;
            Ok(self.img.clone())
        }
    }

    pub fn selection(&self) -> Option<&Selection> {
        self.selection.as_ref()
    }

    pub fn image(&self) -> &Arc<DynamicImage> {
        &self.img
    }

    pub fn select_rect(&mut self, rect: Rect) {
        if rect.size() == Size::ZERO {
            self.selection = None;
        } else {
            self.selection = Some(Selection::rect(rect));
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
