use crate::commands;
use druid::{Command, Data, HotKey, KeyCode, KeyEvent, MenuDesc, MenuItem, RawMods};

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

// A work around when druid menu hotkey is not implemented yet
#[cfg(target_os = "windows")]
pub fn find_command_by_hotkey(key: KeyEvent) -> Option<Command> {
    match key {
        _ if HotKey::new(RawMods::Ctrl, KeyCode::KeyO).matches(key) => {
            Some(commands::file_open_command())
        }
        _ if HotKey::new(RawMods::Ctrl, KeyCode::KeyN).matches(key) => {
            Some(commands::FILE_NEW_ACTION.into())
        }
        _ if HotKey::new(RawMods::Ctrl, KeyCode::F4).matches(key) => {
            Some(commands::FILE_EXIT_ACTION.into())
        }
        _ => None,
    }
}
