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

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
use windows as sys;

#[cfg(target_os = "linux")]
use linux as sys;

#[cfg(target_os = "macos")]
use macos as sys;

pub use sys::*;
