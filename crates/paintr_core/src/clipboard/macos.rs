use super::ClipboardError;

pub fn get_image_from_clipboard() -> Result<Option<image::DynamicImage>, ClipboardError> {
    unimplemented!();
}

pub fn put_image_to_clipboard(_img: &image::DynamicImage) -> Result<(), ClipboardError> {
    unimplemented!();
}
