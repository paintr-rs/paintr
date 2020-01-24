use druid::kurbo::Affine;
use druid::{Data, Point, RenderContext, Size, Vec2};

use crate::image_utils;
use crate::plane::{PlaneIndex, Planes};
use crate::{Paintable, Selection};
use std::sync::Arc;

#[derive(Data, Clone)]
enum SelectionBinder {
    Unbind(Selection),
    Bind(Selection, PlaneIndex),
}

// FIXME: Change name to Layer
#[derive(Data, Clone)]
pub struct CanvasData {
    path: Arc<std::path::PathBuf>,
    selection: Option<SelectionBinder>,
    planes: Planes,
    transform: Vec2,
    size: Size,
}

impl CanvasData {
    pub fn new(path: impl Into<std::path::PathBuf>, img: image::DynamicImage) -> CanvasData {
        let mut planes = Planes::new();
        let size = img.paint_size().unwrap();
        planes.push(Arc::new(img));

        CanvasData {
            selection: None,
            planes,
            path: Arc::new(path.into()),
            transform: Vec2::default(),
            size,
        }
    }

    pub fn path(&self) -> &std::path::Path {
        self.path.as_ref()
    }

    pub fn save(&mut self, path: &std::path::Path) -> Result<(), std::io::Error> {
        let img = self.merged();
        img.save(path)?;
        self.path = Arc::new(path.into());
        Ok(())
    }

    pub fn selection(&self) -> Option<&Selection> {
        match self.selection.as_ref()? {
            SelectionBinder::Unbind(it) => Some(it),
            SelectionBinder::Bind(it, _) => Some(it),
        }
    }

    pub fn merged(&self) -> Arc<image::DynamicImage> {
        let img = self.planes.merged().expect("There is at least plane in Canvas");
        if self.transform == Vec2::ZERO {
            return img;
        }
        // Create partial image based on offset and size
        let mut output =
            image::DynamicImage::new_rgba8(self.size.width as u32, self.size.height as u32);
        image_utils::merge_image(&mut output, &img, self.transform);

        Arc::new(output)
    }

    pub fn select(&mut self, sel: impl Into<Selection>) {
        let sel = sel.into();
        if sel.area() == 0.0 {
            self.selection = None;
        } else {
            self.selection = Some(SelectionBinder::Unbind(sel));
        }
    }

    pub(crate) fn paste(&mut self, img: Arc<image::DynamicImage>) {
        self.planes.push(img);

        // FIXME: we don't need to mov the pasted image if we are using layer.
        self.planes.mov(-self.transform);
    }

    //FIXME: should be move layer, when we implemented layer
    pub(crate) fn move_canvas(&mut self, offset: Vec2) {
        self.transform += offset;

        if let Some(selection) = &self.selection {
            match selection {
                SelectionBinder::Unbind(it) => {
                    let sel = it.transform(offset);
                    self.selection = Some(SelectionBinder::Unbind(sel));
                }
                SelectionBinder::Bind(it, idx) => {
                    let sel = it.transform(offset);
                    self.selection = Some(SelectionBinder::Bind(sel, *idx));
                }
            }
        }
    }

    pub(crate) fn move_selection(&mut self, offset: Vec2) {
        if let Some(selection) = &self.selection {
            let (selection, index) = match selection {
                SelectionBinder::Unbind(it) => {
                    // Bind the selection
                    let sel = it.transform(-self.transform);
                    (it, self.planes.bind_selection(&sel))
                }
                SelectionBinder::Bind(it, index) => (it, *index),
            };

            let sel = selection.transform(offset);
            self.planes.move_with_index(index, offset);
            self.selection = Some(SelectionBinder::Bind(sel, index));
        }
    }

    pub fn position(&self) -> Point {
        self.transform.to_point()
    }
}

impl Paintable for CanvasData {
    fn paint(&self, render_ctx: &mut impl RenderContext) {
        if let Err(err) = render_ctx.with_save(|ctx| {
            ctx.transform(Affine::translate(self.transform));
            self.planes.paint(ctx);
            Ok(())
        }) {
            log::error!("Render context error {}", err);
        }
    }

    fn paint_size(&self) -> Option<Size> {
        Some(self.size)
    }
}
