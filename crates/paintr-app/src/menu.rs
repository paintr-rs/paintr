use crate::commands;
use druid::{Data, KeyCode, MenuDesc, MenuItem, RawMods};

pub(crate) fn make_menu<T: Data>() -> MenuDesc<T> {
    MenuDesc::empty().append(file_menu())
}

fn file_menu<T: Data>() -> MenuDesc<T> {
    MenuDesc::new(L!("menu-file-menu"))
        .append(new())
        .append(open())
        .append_separator()
        .append(exit())
}

fn open<T: Data>() -> MenuItem<T> {
    MenuItem::new(L!("menu-file-open"), commands::file_open_command())
        .hotkey(RawMods::Ctrl, KeyCode::KeyO)
}

fn new<T: Data>() -> MenuItem<T> {
    MenuItem::new(L!("menu-file-new"), commands::FILE_NEW_ACTION)
        .hotkey(RawMods::Ctrl, KeyCode::KeyN)
}

fn exit<T: Data>() -> MenuItem<T> {
    MenuItem::new(L!("menu-file-exit"), commands::FILE_EXIT_ACTION)
        .hotkey(RawMods::Alt, KeyCode::F4)
}
