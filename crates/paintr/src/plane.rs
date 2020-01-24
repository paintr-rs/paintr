use crate::image_utils;
use crate::{CopyMode, Paintable, Selection};
use druid::kurbo::Affine;
use druid::{Data, Point, RenderContext, Size, Vec2};
use image::{DynamicImage, GenericImageView};

use std::sync::Arc;

pub enum Plane {
    Image(Arc<DynamicImage>),
}

impl std::fmt::Debug for Plane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (kind, s) = match self {
            Plane::Image(img) => {
                ("Image", format!("DynamicImage[{}, {}]", img.width(), img.height()))
            }
        };

        write!(f, "Plane {{ {} : {} }}", kind, s)
    }
}

impl_from! {
    Plane : [Arc<DynamicImage> => Image]
}

impl Paintable for Plane {
    fn paint(&self, render_ctx: &mut impl RenderContext) {
        match self {
            Plane::Image(it) => it.paint(render_ctx),
        };
    }

    fn paint_size(&self) -> Option<Size> {
        match self {
            Plane::Image(it) => it.paint_size(),
        }
    }
}

impl Plane {
    fn image(&self) -> Arc<DynamicImage> {
        match self {
            Plane::Image(it) => it.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, Data)]
pub(crate) struct PlaneIndex(usize);

#[derive(Debug, Clone)]
struct PlaneData {
    inner: Arc<Plane>,
    transform: Vec2,
}

// FIXME: Move it to Canvas
#[derive(Debug, Clone)]
pub(crate) struct Planes {
    planes: Vec<PlaneData>,
}

impl Eq for Planes {}
impl PartialEq for Planes {
    fn eq(&self, other: &Planes) -> bool {
        if self.planes.len() != other.planes.len() {
            return false;
        }
        self.planes.iter().zip(other.planes.iter()).all(|(a, b)| Arc::ptr_eq(&a.inner, &b.inner))
    }
}
impl Data for Planes {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

impl Planes {
    pub(crate) fn new() -> Planes {
        Planes { planes: Vec::new() }
    }

    pub(crate) fn max_size(&self) -> Option<Size> {
        return self.planes.iter().fold(None, |acc, plane_data| {
            let size = plane_data.inner.paint_size();
            match (acc, size) {
                (None, _) => size,
                (Some(_), None) => acc,
                (Some(acc), Some(size)) => {
                    Some((acc.width.max(size.width), acc.height.max(size.height)).into())
                }
            }
        });
    }

    pub(crate) fn push(&mut self, plane: impl Into<Plane>) -> PlaneIndex {
        self.planes.push(PlaneData { inner: Arc::new(plane.into()), transform: Vec2::ZERO });
        PlaneIndex(self.planes.len() - 1)
    }

    pub(crate) fn merged(&self) -> Option<Arc<DynamicImage>> {
        let size = self.max_size()?;
        let mut img = image_utils::transparent_image(size.width as u32, size.height as u32);

        for plane in &self.planes {
            image_utils::merge_image(&mut img, &plane.inner.image(), plane.transform);
        }

        Some(Arc::new(img))
    }

    pub(crate) fn mov(&mut self, offset: Vec2) -> Option<Point> {
        let plane = self.planes.last_mut()?;
        plane.transform += offset;
        Some(plane.transform.to_point())
    }

    pub(crate) fn move_with_index(&mut self, idx: PlaneIndex, offset: Vec2) {
        self.planes[idx.0].transform += offset;
    }

    pub(crate) fn bind_selection(&mut self, sel: &Selection) -> PlaneIndex {
        let merged = self.merged().expect("Expect at least one plane exists");
        let cutout = sel.copy(merged, CopyMode::Expand).expect("Fail to copy image from selection");

        // TODO: Cut out all other planes
        for plane in &mut self.planes {
            let target = sel.transform(plane.transform);
            let img = plane.inner.image();
            if let Some(it) = target.cutout(img) {
                plane.inner = Arc::new(it.into());
            }
        }

        self.planes.push(PlaneData {
            inner: Arc::new(cutout.into()),
            transform: sel.position().to_vec2(),
        });
        PlaneIndex(self.planes.len() - 1)
    }
}

impl Paintable for Planes {
    fn paint(&self, render_ctx: &mut impl RenderContext) {
        for plane in &self.planes {
            let _ = render_ctx.with_save(|ctx| {
                ctx.transform(Affine::translate(plane.transform));
                plane.inner.paint(ctx);
                Ok(())
            });
        }
    }
    fn paint_size(&self) -> Option<Size> {
        self.max_size()
    }
}
