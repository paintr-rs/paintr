use druid::kurbo::Affine;
use druid::{Data, Point, RenderContext, Size, Vec2};

use crate::image_utils;
use crate::plane::Planes;
use crate::{Paintable, Selection};
use std::sync::Arc;

// FIXME: Change name to Layer
#[derive(Data, Clone)]
pub struct CanvasData {
    pub(crate) path: Arc<std::path::PathBuf>,
    pub(crate) selection: Option<Selection>,
    pub(crate) planes: Planes,
    pub(crate) transform: Vec2,
    pub(crate) size: Size,
}

impl CanvasData {
    pub fn new(path: impl Into<std::path::PathBuf>, img: image::RgbaImage) -> CanvasData {
        let mut planes = Planes::new();
        let img = image::ImageRgba8(img);
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
        self.selection.as_ref()
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
        if sel.size() == Size::ZERO {
            self.selection = None;
        } else {
            self.selection = Some(sel);
        }
    }

    pub(crate) fn paste(&mut self, img: Arc<image::DynamicImage>) {
        self.planes.push(img);

        // FIXME: we don't need to mov the pasted image if we are using layer.
        self.planes.mov(-self.transform);
    }

    //FIXME: should be move layer, when we implemented layer
    pub(crate) fn mov(&mut self, offset: Vec2) {
        self.transform += offset;

        if let Some(selection) = self.selection() {
            self.selection = Some(selection.transform(offset));
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
