use crate::Paintable;
use druid::{Point, Rect, Vec2};
use image::{GenericImage, GenericImageView};

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
