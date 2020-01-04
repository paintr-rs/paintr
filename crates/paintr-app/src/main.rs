use druid::widget::{Align, Either, Flex, Label, Padding, Scroll, WidgetExt};
use druid::{
    theme, AppDelegate, AppLauncher, Application, Color, Data, DelegateCtx, Env, Event, Lens,
    LensExt, LocalizedString, UnitPoint, Widget, WindowDesc, WindowId,
};
use paintr::{
    get_image_from_clipboard, put_image_to_clipboard, CanvasData, Edit, EditDesc, Paste,
    UndoHistory,
};

macro_rules! L {
    ($str:literal) => {
        $crate::LocalizedString::new($str)
    };
}

mod commands;
mod dialogs;
mod menu;
mod widgets;

use dialogs::DialogData;
use std::sync::Arc;
use widgets::{
    notif_bar::{Notification, NotificationContainer},
    Canvas, ModalContainer, Named,
};

fn main() {
    let app_state = AppState {
        notifications: Arc::new(Vec::new()),
        canvas: None,
        modal: None,
        history: UndoHistory::new(),
    };

    let main_window = WindowDesc::new(ui_builder)
        .title(L!("paint-app-name"))
        .menu(menu::make_menu(&app_state))
        .window_size((800.0, 600.0));

    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .configure_env(|env| {
            env.set(theme::WINDOW_BACKGROUND_COLOR, Color::rgb8(0, 0x77, 0x88));
        })
        // .use_simple_logger()
        .launch(app_state)
        .expect("launch failed");
}

struct Delegate;

type Error = Box<dyn std::error::Error>;

#[derive(Clone, Data, Lens)]
struct AppState {
    notifications: Arc<Vec<Notification>>,
    canvas: Option<CanvasData>,
    modal: Option<DialogData>,
    history: UndoHistory<CanvasData>,
}

impl AppState {
    fn show_notification(&mut self, n: Notification) {
        Arc::make_mut(&mut self.notifications).push(n);
    }

    fn do_open_image(&mut self, path: &std::path::Path) -> Result<(), Error> {
        let img = image::open(path)?;
        let img = match img {
            image::DynamicImage::ImageRgba8(_) => img,
            _ => image::DynamicImage::ImageRgba8(img.to_rgba()),
        };

        self.canvas = Some(CanvasData::new(path.to_owned(), Arc::new(img)));
        Ok(())
    }

    fn do_new_image_from_clipboard(&mut self) -> Result<(), Error> {
        let img = get_image_from_clipboard()?
            .ok_or_else(|| "Clipboard is empty / non-image".to_string())?;

        self.canvas = Some(CanvasData::new(std::path::Path::new("Untitled").into(), Arc::new(img)));
        Ok(())
    }

    fn do_new_image(&mut self, info: &dialogs::NewFileSettings) -> Result<(), Error> {
        let (w, h) = (
            info.width.expect("It must be valid after dialog closed."),
            info.height.expect("It must be valid after dialog closed."),
        );

        let img = image::ImageBuffer::from_fn(w, h, |_, _| {
            image::Rgba([0xff_u8, 0xff_u8, 0xff_u8, 0xff_u8])
        });

        let path = std::path::Path::new("Untitled");

        self.canvas =
            Some(CanvasData::new(path.into(), Arc::new(image::DynamicImage::ImageRgba8(img))));
        Ok(())
    }

    fn do_save_as_image(&mut self, path: &std::path::Path) -> Result<(), Error> {
        let canvas = self.canvas.as_ref().ok_or_else(|| "No image was found.")?;
        let saved = canvas.save(path)?;
        let canvas = CanvasData::new(path.to_path_buf(), saved);
        self.canvas = Some(canvas);
        Ok(())
    }

    fn do_copy(&mut self) -> Result<bool, Error> {
        let img = self
            .canvas
            .as_ref()
            .and_then(|canvas| canvas.selection().map(|sel| sel.image(canvas.image())));

        let img = match img {
            None => return Ok(false),
            Some(img) => img,
        };

        put_image_to_clipboard(&img)?;
        Ok(true)
    }

    fn do_edit(&mut self, edit: impl Edit<CanvasData> + 'static) -> bool {
        let (history, canvas) = (&mut self.history, self.canvas.as_mut());
        if let Some(canvas) = canvas {
            history.push(canvas, edit);
            true
        } else {
            false
        }
    }

    fn do_undo(&mut self) -> Option<EditDesc> {
        let (history, canvas) = (&mut self.history, self.canvas.as_mut()?);
        history.undo(canvas)
    }

    fn do_redo(&mut self) -> Option<EditDesc> {
        let (history, canvas) = (&mut self.history, self.canvas.as_mut()?);
        history.redo(canvas)
    }

    fn do_paste(&mut self) -> Result<bool, Error> {
        let img = get_image_from_clipboard()?;
        let img = match img {
            Some(img) => img,
            None => return Ok(false),
        };

        Ok(self.do_edit(Paste::new(img)))
    }

    fn image_file_name(&self) -> String {
        match &self.canvas {
            None => "Untitled".into(),
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
        Some(self.canvas.as_ref()?.selection()?.description())
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
                ctx.submit_command(druid::commands::CLOSE_WINDOW.into(), None);
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
                if let Some(desc) = data.do_undo() {
                    data.show_notification(Notification::info(format!("Undo {}", desc)));
                }
            }
            &commands::EDIT_REDO_ACTION => {
                if let Some(desc) = data.do_redo() {
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
            Event::Command(ref cmd) => {
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

fn ui_builder() -> impl Widget<AppState> {
    let text = L!("paintr-front-page-welcome");
    let label = Label::new(text.clone());

    let image_lens = AppState::canvas.map(
        |it| it.clone(),
        |to: &mut _, from| {
            if let Some(s) = to.as_mut() {
                if let Some(f) = from {
                    *s = f;
                }
            }
        },
    );

    let main_content = Either::new(
        |data: &AppState, &_| !data.canvas.is_some(),
        Align::centered(Padding::new(10.0, label)),
        Align::centered(Padding::new(
            10.0,
            Named::new(Scroll::new(Canvas::new().lens(image_lens)), |data: &AppState, _env: &_| {
                data.image_file_name()
            }),
        )),
    );

    let container = ModalContainer::new(
        NotificationContainer::new(main_content, AppState::notifications),
        |modal, _| modal.widget(),
        AppState::modal,
    );

    Flex::column().with_child(container, 1.0).with_child(
        Label::new(|data: &AppState, _env: &Env| data.status().unwrap_or_default())
            .align(UnitPoint::RIGHT)
            .padding((5.0, 3.0))
            .background(Color::rgb(0.5, 0.3, 0.5))
            .env_scope(|env, _| {
                env.set(theme::TEXT_SIZE_NORMAL, 12.0);
            }),
        0.0,
    )
}
