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
    undos: Vec<(Arc<dyn Edit<T>>, T)>,
    redos: Vec<Arc<dyn Edit<T>>>,
}

impl<T: Data> Data for UndoHistory<T> {
    fn same(&self, _other: &Self) -> bool {
        true
    }
}

impl<T: Clone + Data> UndoHistory<T> {
    pub fn push(&mut self, data: &mut T, edit: impl Edit<T> + 'static) {
        self.push_inner(data, Arc::new(edit));
    }

    pub fn new() -> UndoHistory<T> {
        UndoHistory { undos: Vec::new(), redos: Vec::new() }
    }

    pub fn undo(&mut self, data: &mut T) -> Option<EditDesc> {
        let (edit, old) = self.undos.pop()?;
        let desc = edit.description();
        self.redos.push(edit);
        *data = old;
        Some(desc)
    }

    pub fn redo(&mut self, data: &mut T) -> Option<EditDesc> {
        let edit = self.redos.pop()?;
        let desc = edit.description();
        self.push_inner(data, edit);
        Some(desc)
    }

    fn push_inner(&mut self, data: &mut T, edit: Arc<dyn Edit<T>>) {
        let old = edit.execute(data);
        self.undos.push((edit, old));
    }
}
