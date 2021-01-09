use std::io::Cursor;

use super::ClipboardError;
use druid::{Application, ClipboardFormat};

pub fn get_image_from_clipboard() -> Result<Option<image::DynamicImage>, ClipboardError> {
    let clipboard = Application::global().clipboard();

    let format_id = match clipboard.preferred_format(&["image/png"]) {
        Some(id) => id,
        None => return Ok(None),
    };

    let data = match clipboard.get_format(format_id) {
        Some(data) => data,
        None => return Ok(None),
    };

    Ok(Some(image::load(Cursor::new(data), image::ImageFormat::Png)?))
}

pub fn put_image_to_clipboard(img: &image::DynamicImage) -> Result<(), ClipboardError> {
    let mut clipboard = Application::global().clipboard();
    let mut data = vec![];
    img.write_to(&mut data, image::ImageFormat::Png)?;

    let fmt = ClipboardFormat::new("image/png", data);
    clipboard.put_formats(&[fmt]);

    Ok(())
}
