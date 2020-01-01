use crate::Selection;
use druid::{Data, Rect, Size};
use image::DynamicImage;

use std::sync::Arc;

// FIXME: Change name to Layer
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

    pub fn set_selection(&mut self, selection: Selection) {
        self.selection = Some(selection);
    }

    fn clear_selection(&mut self) {
        self.selection = None;
    }

    pub fn image(&self) -> &Arc<DynamicImage> {
        &self.img
    }

    pub fn select_rect(&mut self, rect: Rect) {
        if rect.size() == Size::ZERO {
            self.clear_selection();
        } else {
            self.set_selection(Selection::rect(rect));
        }
    }
}
