use druid::Data;
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct EditDesc(String);

impl EditDesc {
    pub(crate) fn new(s: impl Into<String>) -> EditDesc {
        EditDesc(s.into())
    }
}

impl std::fmt::Display for EditDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Edit represent an edit action
pub trait Edit<T: Data>: Debug {
    fn apply(&self, data: &mut T);

    fn execute(&self, data: &mut T) -> T {
        let old = data.clone();
        self.apply(data);
        old
    }

    // FIXME: I dont't like using Any, but let's review it later
    fn merge(&self, _other: &mut dyn Any) -> bool {
        false
    }

    fn description(&self) -> EditDesc;
}

#[derive(Debug, Clone)]
pub struct UndoHistory<T>
where
    T: Data + Debug,
{
    undos: Vec<UndoState<T>>,
    redos: Vec<Arc<dyn Edit<T>>>,
}

impl<T: Data + Debug> Data for UndoHistory<T> {
    fn same(&self, _other: &Self) -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EditKind {
    NonMergeable,
    Mergeable,
}

#[derive(Debug, Clone)]
struct UndoState<T: Data> {
    old: T,
    kind: EditKind,
    edit: Arc<dyn Edit<T>>,
}

impl<T: Data> UndoState<T> {
    fn new(old: T, edit: Arc<dyn Edit<T>>, kind: EditKind) -> UndoState<T> {
        UndoState { old, edit, kind }
    }

    fn undo(self, data: &mut T) -> Arc<dyn Edit<T>> {
        let (edit, old) = (self.edit, self.old);
        *data = old;
        edit
    }
}

impl<T: Data + Debug> UndoHistory<T> {
    /// Add an edit on the undo stack and apply this edit
    pub fn edit(&mut self, data: &mut T, mut edit: impl Edit<T> + 'static, kind: EditKind) {
        let old = edit.execute(data);
        if let Some(last) = self.undos.last_mut() {
            if last.kind == EditKind::Mergeable && last.edit.merge(&mut edit) {
                last.edit = Arc::new(edit);
                last.kind = kind;
                self.redos.clear();
                return;
            }
        }

        self.undos.push(UndoState::new(old, Arc::new(edit), kind));
        self.redos.clear();
    }

    pub fn new() -> UndoHistory<T> {
        UndoHistory { undos: Vec::new(), redos: Vec::new() }
    }

    pub fn undo(&mut self, data: &mut T) -> Option<EditDesc> {
        let edit = self.undos.pop()?.undo(data);
        let desc = edit.description();
        self.redos.push(edit);
        Some(desc)
    }

    pub fn redo(&mut self, data: &mut T) -> Option<EditDesc> {
        let edit = self.redos.pop()?;
        let desc = edit.description();
        let old = edit.execute(data);
        self.undos.push(UndoState::new(old, edit, EditKind::NonMergeable));
        Some(desc)
    }
}

#[cfg(test)]
mod test {
    // FIXME: Add tests for UndoHistory
}
