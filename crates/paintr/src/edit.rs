use druid::Data;

use std::sync::Arc;

/// Edit represent an edit action
pub trait Edit<T: Data> {
    fn apply(&self, data: &mut T);

    fn execute(&self, data: &mut T) -> T {
        let old = data.clone();
        self.apply(data);
        old
    }

    fn description(&self) -> String;
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
    pub fn push(&mut self, edit: Arc<dyn Edit<T>>, data: T) {
        self.undos.push((edit, data));
    }

    pub fn new() -> UndoHistory<T> {
        UndoHistory { undos: Vec::new(), redos: Vec::new() }
    }

    pub fn undo(&mut self) -> Option<(T, String)> {
        let (edit, old) = self.undos.pop()?;
        let desc = edit.description();
        self.redos.push(edit);
        Some((old, desc))
    }

    pub fn redo(&mut self) -> Option<Arc<dyn Edit<T>>> {
        self.redos.pop()
    }
}
