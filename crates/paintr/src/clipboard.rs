#[derive(Debug)]
pub enum ClipboardError {
    IOError(std::io::Error),
    ImageError(image::ImageError),
}

impl std::error::Error for ClipboardError {}
impl std::fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClipboardError::IOError(err) => write!(f, "ClipboardError: {}", err),
            ClipboardError::ImageError(err) => write!(f, "ClipboardError: {}", err),
        }
    }
}

impl From<std::io::Error> for ClipboardError {
    fn from(s: std::io::Error) -> Self {
        ClipboardError::IOError(s)
    }
}

impl From<image::ImageError> for ClipboardError {
    fn from(s: image::ImageError) -> Self {
        ClipboardError::ImageError(s)
    }
}

#[cfg(target_os = "windows")]
pub use windows::get_image_from_clipboard;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn get_image_from_clipboard() -> Result<Option<image::DynamicImage>, ClipboardError> {
    unimplemented!();
}

#[cfg(target_os = "windows")]
mod windows {
    use super::ClipboardError;
    use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
    use druid::Application;
    use std::io::Cursor;
    use std::io::Write;

    pub fn get_image_from_clipboard() -> Result<Option<image::DynamicImage>, ClipboardError> {
        let clipboard = Application::clipboard();

        let format_id = match clipboard.preferred_format(&["CF_DIBV5"]) {
            Some(id) => id,
            None => return Ok(None),
        };

        let mut data = match clipboard.get_format(format_id) {
            Some(data) => data,
            None => return Ok(None),
        };

        let mut bmp_buf = compute_bmp_header(&data)?;
        bmp_buf.append(&mut data);

        Ok(Some(image::load(
            Cursor::new(bmp_buf),
            image::ImageFormat::BMP,
        )?))
    }

    // BITMAPV5HEADER
    // DWORD        bV5Size;            4   OFFSET 0
    // LONG         bV5Width;           4   OFFSET 4
    // LONG         bV5Height;          4   OFFSET 8
    // WORD         bV5Planes;          2   OFFSET 12
    // WORD         bV5BitCount;        2   OFFSET 14
    // DWORD        bV5Compression;     4   OFFSET 16
    // DWORD        bV5SizeImage;       4   OFFSET 20
    // LONG         bV5XPelsPerMeter;   4   OFFSET 24
    // LONG         bV5YPelsPerMeter;   4   OFFSET 28
    // DWORD        bV5ClrUsed;         4   OFFSET 32

    const BI_BITFIELDS: u32 = 0x0003;
    const V5_COMPRESSION_OFFSET: u64 = 16;
    const V5_CLR_USED_OFFSET: u64 = 32;
    const BIT_MASK_SIZE: u32 = 12;
    const FILE_HEADER_SIZE: u32 = 14;

    // https://itnext.io/bits-to-bitmaps-a-simple-walkthrough-of-bmp-image-format-765dc6857393
    fn compute_bmp_header(content: &[u8]) -> Result<Vec<u8>, std::io::Error> {
        let mut cursor = Cursor::new(content);
        let dib_header_size = cursor.read_u32::<LittleEndian>()?;

        cursor.set_position(V5_COMPRESSION_OFFSET);
        let v5_compession = cursor.read_u32::<LittleEndian>()?;

        // FIXME: compute correct color table size
        cursor.set_position(V5_CLR_USED_OFFSET);
        let color_count = cursor.read_u32::<LittleEndian>()?;
        let sizeof_rgba = 4;

        let mut pixel_data_offset = dib_header_size + color_count * sizeof_rgba;
        if v5_compession == BI_BITFIELDS {
            pixel_data_offset += BIT_MASK_SIZE; //bit masks follow the header
        }

        let mut buf = std::io::BufWriter::new(Vec::new());
        // File Type
        buf.write(b"BM")?;
        // File Size
        buf.write_u32::<LittleEndian>(content.len() as u32 + FILE_HEADER_SIZE)?;
        // Reserved
        buf.write_u16::<LittleEndian>(0)?;
        // Reserved
        buf.write_u16::<LittleEndian>(0)?;
        // the offset of actual pixel data in bytes
        buf.write_u32::<LittleEndian>(FILE_HEADER_SIZE + pixel_data_offset)?;

        Ok(buf.into_inner()?)
    }
}
