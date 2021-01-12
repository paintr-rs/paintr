use crate::image_utils;
use crate::{CopyMode, Paintable, Selection};
use druid::{kurbo::Affine, PaintCtx};
use druid::{Data, RenderContext, Size, Vec2};
use image::{DynamicImage, GenericImageView};
use imageproc::drawing;

use std::{cell::RefCell, sync::Arc};

pub struct DrawPlane {
    img: DynamicImage,
    brush: Vec<Vec2>,
}

impl DrawPlane {
    pub fn new(w: u32, h: u32) -> Self {
        let img = image_utils::transparent_image(w, h);

        Self { img, brush: vec![] }
    }
}

pub enum Plane {
    Image(Arc<DynamicImage>),
    Draw(Arc<RefCell<DrawPlane>>),
}

impl std::fmt::Debug for Plane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (kind, s) = match self {
            Plane::Image(img) => {
                ("Image", format!("DynamicImage[{}, {}]", img.width(), img.height()))
            }
            Plane::Draw(img) => (
                "Draw",
                format!(
                    "DynamicImage[{}, {}]",
                    img.borrow().img.width(),
                    img.borrow().img.height()
                ),
            ),
        };

        write!(f, "Plane {{ {} : {} }}", kind, s)
    }
}

impl Paintable for Plane {
    fn paint(&self, paint_ctx: &mut PaintCtx) {
        match self {
            Plane::Image(it) => it.paint(paint_ctx),
            Plane::Draw(it) => it.borrow().img.paint(paint_ctx),
        };
    }

    fn paint_size(&self) -> Option<Size> {
        match self {
            Plane::Image(it) => it.paint_size(),
            Plane::Draw(it) => it.borrow().img.paint_size(),
        }
    }
}

impl Plane {
    fn image(&self) -> Arc<DynamicImage> {
        match self {
            Plane::Image(it) => it.clone(),
            Plane::Draw(it) => Arc::new(it.borrow().img.clone()),
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

    /// Merge all plane, ignore negative transform
    pub(crate) fn merged(&self) -> Option<Arc<DynamicImage>> {
        let size = self.max_size()?;
        let mut img = image_utils::transparent_image(size.width as u32, size.height as u32);

        for plane in &self.planes {
            image_utils::merge_image(&mut img, &plane.inner.image(), plane.transform);
        }

        Some(Arc::new(img))
    }

    pub(crate) fn merged_to(&self, mut img: DynamicImage, transform: Vec2) -> Arc<DynamicImage> {
        for plane in &self.planes {
            image_utils::merge_image(&mut img, &plane.inner.image(), plane.transform + transform);
        }
        Arc::new(img)
    }

    pub(crate) fn move_with_index(&mut self, idx: PlaneIndex, offset: Vec2) {
        self.planes[idx.0].transform += offset;
    }

    pub(crate) fn bind_selection(&mut self, sel: &Selection) -> PlaneIndex {
        let merged = self.merged().expect("Expect at least one plane exists");
        let cutout = sel.copy(merged, CopyMode::Expand).expect("Fail to copy image from selection");

        // TODO: Cut out all other planes
        for plane in &mut self.planes {
            let target = sel.transform(-plane.transform);
            let img = plane.inner.image();
            if let Some(it) = target.cutout(img) {
                plane.inner = Arc::new(Plane::Image(it));
            }
        }

        self.planes.push(PlaneData {
            inner: Arc::new(Plane::Image(cutout)),
            transform: sel.position().to_vec2(),
        });
        PlaneIndex(self.planes.len() - 1)
    }

    pub(crate) fn draw_with_brush(&mut self, pos: &Vec<Vec2>) {
        let (size, last) = match (self.max_size(), self.planes.last()) {
            (Some(size), Some(last)) => (size, last),
            _ => return,
        };

        if let Plane::Image(_) = last.inner.as_ref() {
            let empty =
                Arc::new(RefCell::new(DrawPlane::new(size.width as u32, size.height as u32)));
            self.push(Plane::Draw(empty.clone()));
        }

        // reuse last plane if it is draw plane
        let mut empty = match self.planes.last().map(|it| it.inner.as_ref()) {
            Some(Plane::Draw(img)) => img.borrow_mut(),
            _ => unreachable!(),
        };

        for p in pos {
            if !empty.brush.iter().all(|it| it != p) {
                continue;
            }

            drawing::draw_filled_circle_mut(
                &mut empty.img,
                (p.x as i32, p.y as i32),
                5,
                image_utils::colors::YELLOW,
            );

            empty.brush.push(*p);
        }
    }
}

impl Paintable for Planes {
    fn paint(&self, paint_ctx: &mut PaintCtx) {
        for plane in &self.planes {
            paint_ctx.with_save(|ctx| {
                ctx.transform(Affine::translate(plane.transform));
                plane.inner.paint(ctx);
            });
        }
    }
    fn paint_size(&self) -> Option<Size> {
        self.max_size()
    }
}
