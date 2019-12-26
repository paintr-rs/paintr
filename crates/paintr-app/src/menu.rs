use crate::commands;
use crate::AppState;
use druid::{Command, HotKey, KeyCode, KeyEvent, MenuDesc, MenuItem, RawMods};

pub(crate) fn make_menu(app: &AppState) -> MenuDesc<AppState> {
    MenuDesc::empty().append(file_menu(app))
}

fn file_menu(app: &AppState) -> MenuDesc<AppState> {
    MenuDesc::new(L!("menu-file-menu"))
        .append(new())
        .append_separator()
        .append(open())
        .append_separator()
        .append(save().disabled_if(|| app.image.is_none()))
        .append_separator()
        .append(exit())
}

macro_rules! register_menu_items {
    ($($name:ident => ($sel:literal, $cmd:expr, $mods:ident, $keycode:ident)),*) => {
        $(
        fn $name() -> MenuItem<AppState> {
            MenuItem::new(L!($sel), $cmd)
                .hotkey(RawMods::$mods, KeyCode::$keycode)
        })*

        // A work around when druid menu hotkey is not implemented yet
        #[cfg(target_os = "windows")]
        pub fn find_command_by_hotkey(key: KeyEvent) -> Option<Command> {
            match key {
                $(
                    _ if HotKey::new(RawMods::$mods, KeyCode::$keycode).matches(key) => {
                        Some($cmd.into())
                    }
                )*
                _ => None,
            }
        }
    }
}

register_menu_items! {
    open => ("menu-file-open", commands::file_open_command(), Ctrl, KeyO),
    new => ("menu-file-new", commands::FILE_NEW_ACTION, Ctrl, KeyN),
    save => ("menu-file-save-as", commands::file_save_as_command(), CtrlShift, KeyS),
    exit => ("menu-file-exit", commands::FILE_EXIT_ACTION, Alt, F4)
}
