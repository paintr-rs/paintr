use crate::{Paintable, Selection};
use druid::{Data, PaintCtx, Size};
use image::{DynamicImage, GenericImageView};

use std::sync::Arc;

pub enum Plane {
    Image(Arc<DynamicImage>),
    Selection(Selection),
}

impl std::fmt::Debug for Plane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (kind, s) = match self {
            Plane::Image(img) => {
                ("Image", format!("DynamicImage[{}, {}]", img.width(), img.height()))
            }
            Plane::Selection(sel) => ("Selection", format!("{:?}", sel)),
        };

        write!(f, "Plane {{ {} : {} }}", kind, s)
    }
}

macro_rules! impl_from {
    ($trait:ident : [$($from:ty => $to:ident ),*] ) => {
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
    Plane : [Arc<DynamicImage> => Image, Selection => Selection]
}

impl Paintable for Plane {
    fn paint(&self, paint_ctx: &mut PaintCtx) {
        match self {
            Plane::Image(it) => it.paint(paint_ctx),
            Plane::Selection(it) => it.paint(paint_ctx),
        };
    }

    fn paint_size(&self) -> Option<Size> {
        match self {
            Plane::Image(it) => it.paint_size(),
            Plane::Selection(it) => it.paint_size(),
        }
    }
}

#[derive(Debug, Clone, Copy, Data)]
pub(crate) struct PlaneIndex(usize);

// FIXME: Move it to Canvas
#[derive(Debug, Clone)]
pub(crate) struct Planes {
    planes: Vec<Arc<Plane>>,
}

impl Eq for Planes {}
impl PartialEq for Planes {
    fn eq(&self, other: &Planes) -> bool {
        if self.planes.len() != other.planes.len() {
            return false;
        }
        self.planes.iter().zip(other.planes.iter()).all(|(a, b)| Arc::ptr_eq(a, b))
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
        return self.planes.iter().fold(None, |acc, plane| {
            let size = plane.paint_size();
            match (acc, size) {
                (None, _) => size,
                (Some(_), None) => acc,
                (Some(acc), Some(size)) => {
                    Some((acc.width.max(size.width), acc.height.max(size.height)).into())
                }
            }
        });
    }

    pub fn push(&mut self, plane: impl Into<Plane>) -> PlaneIndex {
        self.planes.push(Arc::new(plane.into()));
        PlaneIndex(self.planes.len() - 1)
    }
}

impl Paintable for Planes {
    fn paint(&self, paint_ctx: &mut PaintCtx) {
        for plane in &self.planes {
            plane.paint(paint_ctx);
        }
    }
    fn paint_size(&self) -> Option<Size> {
        self.max_size()
    }
}
