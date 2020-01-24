use crate::Paintable;
use druid::{Point, Rect, Vec2};
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba};

pub mod colors {
    use super::Rgba;

    pub const TRANSPARENT: Rgba<u8> = super::Rgba([0u8, 0u8, 0u8, 0xffu8]);
    pub const BLACK: Rgba<u8> = Rgba([0x0u8, 0x0u8, 0x0u8, 0xFFu8]);
    pub const WHITE: Rgba<u8> = Rgba([0xffu8, 0xf0u8, 0xffu8, 0xffu8]);
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

    dest.copy_from(&section, origin.x as u32, origin.y as u32);
}

pub(crate) fn make_color_img(w: u32, h: u32, color: Rgba<u8>) -> DynamicImage {
    DynamicImage::ImageRgba8(ImageBuffer::from_fn(w, h, |_, _| color))
}

pub(crate) fn transparent_image(w: u32, h: u32) -> DynamicImage {
    make_color_img(w, h, colors::TRANSPARENT)
}
