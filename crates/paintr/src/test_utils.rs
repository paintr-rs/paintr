use crate::image_utils::make_color_img;
use crate::CanvasData;
use image::Rgba;

pub(crate) fn canvas_fixture(w: u32, h: u32, color: Rgba<u8>) -> CanvasData {
    let img = make_color_img(w, h, color);
    CanvasData::new("test-img", img)
}
