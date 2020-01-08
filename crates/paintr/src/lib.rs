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
mod paintable;
mod plane;
mod selections;

pub use canvas::{CanvasData, Paste};
pub use clipboard::{get_image_from_clipboard, put_image_to_clipboard, ClipboardError};
pub use edit::{Edit, EditDesc, UndoHistory};
pub use paintable::Paintable;
pub use selections::Selection;

pub mod lens;
