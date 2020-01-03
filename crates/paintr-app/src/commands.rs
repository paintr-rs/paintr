use druid::{Command, FileDialogOptions, FileSpec, Selector};
const IMAGE_FILE_TYPE: FileSpec = FileSpec::new("Images", &["bmp", "png", "gif", "jpg", "jpeg"]);

pub(crate) const FILE_EXIT_ACTION: Selector = Selector::new("menu-exit-action");
pub(crate) const FILE_NEW_ACTION: Selector = Selector::new("menu-new-action");
pub(crate) const FILE_NEW_CLIPBOARD_ACTION: Selector = Selector::new("menu-new-clipboard-action");

pub(crate) const EDIT_UNDO_ACTION: Selector = Selector::new("edit-undo-action");
pub(crate) const EDIT_REDO_ACTION: Selector = Selector::new("edit-redo-action");
pub(crate) const EDIT_COPY_ACTION: Selector = Selector::new("edit-copy-action");
pub(crate) const EDIT_PASTE_ACTION: Selector = Selector::new("edit-paste-action");

pub(crate) const ABOUT_TEST_ACTION: Selector = Selector::new("about-test-action");

pub(crate) const NEW_IMAGE_ACTION: Selector = Selector::new("new-image-action");

pub(crate) fn file_open_command() -> Command {
    Command::new(
        druid::commands::SHOW_OPEN_PANEL,
        FileDialogOptions::new().allowed_types(vec![IMAGE_FILE_TYPE]),
    )
}

pub(crate) fn file_save_as_command() -> Command {
    Command::new(
        druid::commands::SHOW_SAVE_PANEL,
        FileDialogOptions::new().allowed_types(vec![IMAGE_FILE_TYPE]),
    )
}
