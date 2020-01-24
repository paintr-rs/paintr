macro_rules! L {
    ($str:literal) => {
        $crate::LocalizedString::new($str)
    };
}

mod commands;
mod dialogs;
mod menu;
mod theme_ext;
mod tools;
mod ui;
mod widgets;

use druid::{
    theme, AppDelegate, AppLauncher, Application, Color, Data, DelegateCtx, Env, Event, Lens,
    LocalizedString, WindowDesc, WindowId,
};
use paintr::{
    actions::Paste, get_image_from_clipboard, put_image_to_clipboard, CanvasData, CopyMode, Edit,
    EditDesc, EditKind, UndoHistory,
};

use std::sync::Arc;

use dialogs::DialogData;
use tools::ToolKind;
use ui::ui_builder;
use widgets::notif_bar::Notification;

fn main() {
    let app_state = AppState {
        notifications: Arc::new(Vec::new()),
        modal: None,
        editor: EditorState {
            canvas: None,
            history: UndoHistory::new(),
            tool: ToolKind::Select,
            is_editing: false,
        },
    };

    let main_window = WindowDesc::new(ui_builder)
        .title(L!("paint-app-name"))
        .menu(menu::make_menu(&app_state))
        .window_size((800.0, 600.0));

    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .configure_env(|env, _| {
            env.set(theme::WINDOW_BACKGROUND_COLOR, Color::rgb8(0, 0x77, 0x88));
            theme_ext::init(env);
        })
        // .use_simple_logger()
        .launch(app_state)
        .expect("launch failed");
}

struct Delegate;

type Error = Box<dyn std::error::Error>;

#[derive(Clone, Data, Lens)]
pub struct EditorState {
    canvas: Option<CanvasData>,
    history: UndoHistory<CanvasData>,
    tool: ToolKind,
    is_editing: bool,
}

impl EditorState {
    fn do_edit(&mut self, edit: impl Edit<CanvasData> + 'static, kind: EditKind) -> bool {
        self.is_editing = kind == EditKind::Mergeable;

        let (history, canvas) = (&mut self.history, self.canvas.as_mut());
        if let Some(canvas) = canvas {
            history.edit(canvas, edit, kind);
            true
        } else {
            false
        }
    }

    fn do_undo(&mut self) -> Option<EditDesc> {
        if self.is_editing {
            return None;
        }
        let (history, canvas) = (&mut self.history, self.canvas.as_mut()?);
        history.undo(canvas)
    }

    fn do_redo(&mut self) -> Option<EditDesc> {
        if self.is_editing {
            return None;
        }

        let (history, canvas) = (&mut self.history, self.canvas.as_mut()?);
        history.redo(canvas)
    }
}

#[derive(Clone, Data, Lens)]
struct AppState {
    notifications: Arc<Vec<Notification>>,
    modal: Option<DialogData>,
    editor: EditorState,
}

const NEW_FILE_NAME: &str = "Untitled";

fn to_rgba(img: image::DynamicImage) -> image::DynamicImage {
    image::DynamicImage::ImageRgba8(match img {
        image::DynamicImage::ImageRgba8(img) => img,
        _ => img.to_rgba(),
    })
}

impl AppState {
    fn show_notification(&mut self, n: Notification) {
        Arc::make_mut(&mut self.notifications).push(n);
    }

    fn do_open_image(&mut self, path: &std::path::Path) -> Result<(), Error> {
        let img = image::open(path)?;
        self.editor.canvas = Some(CanvasData::new(path, to_rgba(img)));
        Ok(())
    }

    fn do_new_image_from_clipboard(&mut self) -> Result<(), Error> {
        let img = get_image_from_clipboard()?
            .ok_or_else(|| "Clipboard is empty / non-image".to_string())?;
        self.editor.canvas = Some(CanvasData::new(NEW_FILE_NAME, to_rgba(img)));
        Ok(())
    }

    fn do_new_image(&mut self, info: &dialogs::NewFileSettings) -> Result<(), Error> {
        let (w, h) = (
            info.width.expect("It must be valid after dialog closed."),
            info.height.expect("It must be valid after dialog closed."),
        );
        // Fill with white color
        let img = image::ImageBuffer::from_fn(w, h, |_, _| {
            image::Rgba([0xff_u8, 0xff_u8, 0xff_u8, 0xff_u8])
        });

        self.editor.canvas =
            Some(CanvasData::new(NEW_FILE_NAME, image::DynamicImage::ImageRgba8(img)));
        Ok(())
    }

    fn do_save_as_image(&mut self, path: &std::path::Path) -> Result<(), Error> {
        let canvas = self.editor.canvas.as_mut().ok_or_else(|| "No image was found.")?;
        canvas.save(path)?;
        Ok(())
    }

    fn do_copy(&mut self) -> Result<bool, Error> {
        let img = self.editor.canvas.as_ref().and_then(|canvas| {
            canvas.selection().map(|sel| sel.copy(canvas.merged(), CopyMode::Shrink))
        });

        let img = match img.flatten() {
            None => return Ok(false),
            Some(img) => img,
        };

        put_image_to_clipboard(&img)?;
        Ok(true)
    }

    fn do_paste(&mut self) -> Result<bool, Error> {
        let img = get_image_from_clipboard()?;
        let img = match img {
            Some(img) => img,
            None => return Ok(false),
        };
        let img = to_rgba(img);
        Ok(self.editor.do_edit(Paste::new(img), EditKind::NonMergeable))
    }

    fn image_file_name(&self) -> String {
        match &self.editor.canvas {
            None => NEW_FILE_NAME.to_owned(),
            Some(canvas) => canvas.path().to_string_lossy().into(),
        }
    }

    fn update_menu(&self, ctx: &mut DelegateCtx) {
        ctx.submit_command(
            druid::Command::new(druid::commands::SET_MENU, menu::make_menu(self)),
            None,
        );
    }

    fn status(&self) -> Option<String> {
        Some(self.editor.canvas.as_ref()?.selection()?.description())
    }
}

impl Delegate {
    fn handle_command(
        &mut self,
        data: &mut AppState,
        ctx: &mut DelegateCtx,
        cmd: &druid::Command,
    ) -> Result<(), Error> {
        match &cmd.selector {
            &commands::FILE_EXIT_ACTION => {
                ctx.submit_command(druid::commands::CLOSE_WINDOW, None);
            }
            &commands::FILE_NEW_ACTION => {
                data.modal = Some(DialogData::new_file_settings());
                data.update_menu(ctx);
            }
            &commands::FILE_NEW_CLIPBOARD_ACTION => {
                data.do_new_image_from_clipboard()?;
                data.show_notification(Notification::info("New file created"));
                data.update_menu(ctx);
            }
            &druid::commands::OPEN_FILE => {
                let info = cmd
                    .get_object::<druid::FileInfo>()
                    .ok_or_else(|| "api violation".to_string())?;
                data.do_open_image(info.path())?;
                data.show_notification(Notification::info(format!(
                    "{} opened",
                    data.image_file_name()
                )));
                data.update_menu(ctx);
            }
            &druid::commands::SAVE_FILE => {
                let info = cmd
                    .get_object::<druid::FileInfo>()
                    .ok_or_else(|| "api violation".to_string())?;
                data.do_save_as_image(info.path())?;
                data.show_notification(Notification::info(format!(
                    "{} saved",
                    data.image_file_name()
                )));
                data.update_menu(ctx);
            }
            &commands::EDIT_UNDO_ACTION => {
                if let Some(desc) = data.editor.do_undo() {
                    data.show_notification(Notification::info(format!("Undo {}", desc)));
                }
            }
            &commands::EDIT_REDO_ACTION => {
                if let Some(desc) = data.editor.do_redo() {
                    data.show_notification(Notification::info(format!("Redo {}", desc)));
                }
            }
            &commands::EDIT_COPY_ACTION => {
                if data.do_copy()? {
                    data.show_notification(Notification::info("Copied"));
                }
            }
            &commands::EDIT_PASTE_ACTION => {
                if data.do_paste()? {
                    data.show_notification(Notification::info("Pasted"));
                }
            }
            &commands::NEW_IMAGE_ACTION => {
                let info = cmd
                    .get_object::<dialogs::NewFileSettings>()
                    .ok_or_else(|| "api violation".to_string())?;

                data.do_new_image(info)?;
                data.show_notification(Notification::info("New file created"));
                data.update_menu(ctx);
            }
            _ => (),
        }

        Ok(())
    }
}

impl AppDelegate<AppState> for Delegate {
    fn event(
        &mut self,
        event: Event,
        data: &mut AppState,
        _env: &Env,
        ctx: &mut DelegateCtx,
    ) -> Option<Event> {
        match event {
            Event::TargetedCommand(_, ref cmd) => {
                if let Err(err) = self.handle_command(data, ctx, cmd) {
                    data.show_notification(Notification::error(err.to_string()));
                }
            }

            _ => (),
        };

        Some(event)
    }

    fn window_removed(
        &mut self,
        _id: WindowId,
        _data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        // FIXME: Use commands::QUIT_APP
        // It do not works right now, maybe a druid bug
        Application::quit();
    }
}
