#[macro_export]
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

mod canvas;
mod clipboard;
mod edit;
mod image_utils;
mod paintable;
mod plane;
mod selections;

#[cfg(test)]
mod test_utils;

pub use canvas::CanvasData;
pub use clipboard::{get_image_from_clipboard, put_image_to_clipboard, ClipboardError};
pub use edit::{Edit, EditDesc, EditKind, UndoHistory};
pub use paintable::Paintable;
pub use selections::{CopyMode, Selection};
pub mod actions;
pub use image_utils::colors;

pub mod lens;
