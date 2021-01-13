use crate::commands;
use crate::AppState;
use druid::{KbKey, MenuDesc, MenuItem, RawMods};

pub(crate) fn make_menu(app: &AppState) -> MenuDesc<AppState> {
    MenuDesc::empty().append(file_menu(app)).append(edit_menu(app)).append(about_menu(app))
}

fn file_menu(app: &AppState) -> MenuDesc<AppState> {
    MenuDesc::new(L!("menu-file-menu"))
        .append(new())
        .append(new_from_clipboard())
        .append_separator()
        .append(open())
        .append_separator()
        .append(save().disabled_if(|| app.editor.canvas.is_none()))
        .append_separator()
        .append(exit())
}

fn edit_menu(_app: &AppState) -> MenuDesc<AppState> {
    MenuDesc::new(L!("menu-edit-menu"))
        .append(undo())
        .append(redo())
        .append_separator()
        .append(copy())
        .append(paste())
}

fn about_menu(_app: &AppState) -> MenuDesc<AppState> {
    MenuDesc::new(L!("menu-about-menu")).append(about())
}

macro_rules! register_menu_items {
    ($($name:ident => ($sel:literal, $cmd:expr $(, $mods:ident, $keycode:expr)? )),*) => {
        $(
        fn $name() -> MenuItem<AppState> {
            MenuItem::new(L!($sel), $cmd)
                $( .hotkey(RawMods::$mods, $keycode) )?
        })*
    }
}

register_menu_items! {
    // files
    open => ("menu-file-open", commands::file_open_command(), Ctrl, KbKey::Character("O".to_string())),
    new => ("menu-file-new", commands::FILE_NEW_ACTION, Ctrl, KbKey::Character("N".to_string())),
    new_from_clipboard => ("menu-file-new-clipboard", commands::FILE_NEW_CLIPBOARD_ACTION),
    save => ("menu-file-save-as", commands::file_save_as_command(), CtrlShift, KbKey::Character("S".to_string())),
    // edit
    undo => ("menu-edit-undo", commands::EDIT_UNDO_ACTION, Ctrl, KbKey::Character("Z".to_string())),
    redo => ("menu-edit-redo", commands::EDIT_REDO_ACTION, CtrlShift, KbKey::Character("Z".to_string())),
    copy => ("menu-edit-copy", commands::EDIT_COPY_ACTION, Ctrl, KbKey::Character("C".to_string())),
    paste => ("menu-edit-paste", commands::EDIT_PASTE_ACTION, Ctrl, KbKey::Character("V".to_string())),

    // about
    about => ("menu-about-test", commands::ABOUT_TEST_ACTION)
}

#[cfg(target_os = "windows")]
register_menu_items! {
    exit => ("menu-file-exit", commands::FILE_EXIT_ACTION, Alt, KbKey::F4)
}

#[cfg(target_os = "linux")]
register_menu_items! {
    exit => ("menu-file-exit", commands::FILE_EXIT_ACTION, Ctrl, KbKey::Character("Q".to_string()))
}

#[cfg(target_os = "macos")]
register_menu_items! {
    exit => ("menu-file-exit", commands::FILE_EXIT_ACTION, Ctrl, KbKey::Character("Q".to_string()))
}
