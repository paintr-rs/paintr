use crate::Paintable;
use druid::{Point, Rect, Vec2};
use image::{
    error::ParameterError, error::ParameterErrorKind, DynamicImage, GenericImage, GenericImageView,
    ImageBuffer, Pixel, Rgba,
};

pub mod colors {
    use super::Rgba;

    pub const TRANSPARENT: Rgba<u8> = super::Rgba([0u8, 0u8, 0u8, 0x0u8]);
    pub const BLACK: Rgba<u8> = Rgba([0x0u8, 0x0u8, 0x0u8, 0xFFu8]);
    pub const WHITE: Rgba<u8> = Rgba([0xffu8, 0xf0u8, 0xffu8, 0xffu8]);
    pub const YELLOW: Rgba<u8> = Rgba([0xffu8, 0xc9u8, 0x22u8, 0xffu8]);
}

fn blend_from<O>(dest: &mut DynamicImage, other: &O, x: u32, y: u32) -> image::ImageResult<()>
where
    O: GenericImageView<Pixel = Rgba<u8>>,
{
    // Do bounds checking here so we can use the non-bounds-checking
    // functions to copy pixels.
    if dest.width() < other.width() + x || dest.height() < other.height() + y {
        return Err(image::ImageError::Parameter(ParameterError::from_kind(
            ParameterErrorKind::DimensionMismatch,
        )));
    }

    for i in 0..other.width() {
        for k in 0..other.height() {
            let p = other.get_pixel(i, k);
            let mut to = dest.get_pixel(i + x, k + y);
            to.blend(&p);
            dest.put_pixel(i + x, k + y, to);
        }
    }

    Ok(())
}

pub(crate) fn merge_image(
    dest: &mut image::DynamicImage,
    src: &image::DynamicImage,
    transform: Vec2,
) {
    let src_size = src.paint_size().unwrap();
    let dest_size = dest.paint_size().unwrap();

    let full = Rect::from_origin_size(Point::ZERO, dest_size);
    let rt = Rect::from_origin_size(transform.to_point(), src_size);
    let rt = rt.intersect(full);

    let origin = rt.origin().to_vec2();
    let offset = origin - transform;

    let section =
        src.view(offset.x as u32, offset.y as u32, rt.size().width as u32, rt.size().height as u32);

    blend_from(dest, &section, origin.x as u32, origin.y as u32).expect("The size is invalid");
}

pub(crate) fn make_color_img(w: u32, h: u32, color: Rgba<u8>) -> DynamicImage {
    DynamicImage::ImageRgba8(ImageBuffer::from_fn(w, h, |_, _| color))
}

pub(crate) fn transparent_image(w: u32, h: u32) -> DynamicImage {
    make_color_img(w, h, colors::TRANSPARENT)
}
