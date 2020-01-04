use druid::Data;

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
pub trait Edit<T: Data> {
    fn apply(&self, data: &mut T);

    fn execute(&self, data: &mut T) -> T {
        let old = data.clone();
        self.apply(data);
        old
    }

    fn description(&self) -> EditDesc;
}

#[derive(Clone)]
pub struct UndoHistory<T>
where
    T: Data,
{
    undos: Vec<UndoState<T>>,
    redos: Vec<Arc<dyn Edit<T>>>,
}

impl<T: Data> Data for UndoHistory<T> {
    fn same(&self, _other: &Self) -> bool {
        true
    }
}

#[derive(Clone)]
struct UndoState<T: Data> {
    old: T,
    edit: Arc<dyn Edit<T>>,
}

impl<T: Data> UndoState<T> {
    fn new(old: T, edit: Arc<dyn Edit<T>>) -> UndoState<T> {
        UndoState { old, edit }
    }

    fn undo(self, data: &mut T) -> Arc<dyn Edit<T>> {
        let (edit, old) = (self.edit, self.old);
        *data = old;
        edit
    }
}

impl<T: Data> UndoHistory<T> {
    pub fn edit(&mut self, data: &mut T, edit: impl Edit<T> + 'static) {
        self.edit_inner(data, Arc::new(edit));
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
        self.edit_inner(data, edit);
        Some(desc)
    }

    fn edit_inner(&mut self, data: &mut T, edit: Arc<dyn Edit<T>>) {
        let old = edit.execute(data);
        self.undos.push(UndoState::new(old, edit));
    }
}
