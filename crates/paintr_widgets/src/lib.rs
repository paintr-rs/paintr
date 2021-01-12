use druid::{Cursor, Data, Lens};
use paintr_core::{CanvasData, Edit, EditDesc, EditKind, UndoHistory};

pub mod theme_ext;
pub mod widgets;

#[derive(Clone, Data, Lens)]
pub struct EditorState<T> {
    pub canvas: Option<CanvasData>,
    pub history: UndoHistory<CanvasData>,
    pub tool: T,
    pub is_editing: bool,
    pub cursor: Option<Cursor>,
}

impl<T> EditorState<T> {
    pub fn do_edit(&mut self, edit: impl Edit<CanvasData> + 'static, kind: EditKind) -> bool {
        self.is_editing = kind == EditKind::Mergeable;

        let (history, canvas) = (&mut self.history, self.canvas.as_mut());
        if let Some(canvas) = canvas {
            history.edit(canvas, edit, kind);
            true
        } else {
            false
        }
    }

    pub fn do_undo(&mut self) -> Option<EditDesc> {
        if self.is_editing {
            return None;
        }
        let (history, canvas) = (&mut self.history, self.canvas.as_mut()?);
        history.undo(canvas)
    }

    pub fn do_redo(&mut self) -> Option<EditDesc> {
        if self.is_editing {
            return None;
        }

        let (history, canvas) = (&mut self.history, self.canvas.as_mut()?);
        history.redo(canvas)
    }
}
