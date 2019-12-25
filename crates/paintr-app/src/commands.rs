use druid::{Command, FileDialogOptions, FileSpec, Selector};
const IMAGE_FILE_TYPE: FileSpec = FileSpec::new("Images", &["png", "gif", "jpg", "jpeg"]);

pub(crate) const FILE_EXIT_ACTION: Selector = Selector::new("menu-exit-action");
pub(crate) const FILE_NEW_ACTION: Selector = Selector::new("menu-new-action");

pub(crate) fn file_open_command() -> Command {
    Command::new(
        druid::commands::SHOW_OPEN_PANEL,
        FileDialogOptions::new().allowed_types(vec![IMAGE_FILE_TYPE]),
    )
}