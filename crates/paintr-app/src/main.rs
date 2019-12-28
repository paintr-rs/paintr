use druid::piet::Color;
use druid::widget::{Align, Either, Label, Padding, Scroll, WidgetExt};
use druid::{
    theme, AppDelegate, AppLauncher, Application, Data, DelegateCtx, Env, Event, Lens, LensExt,
    LocalizedString, Widget, WindowDesc, WindowId,
};
use paintr::get_image_from_clipboard;

macro_rules! L {
    ($str:literal) => {
        $crate::LocalizedString::new($str)
    };
}

mod commands;
mod menu;
mod widgets;
use std::sync::Arc;
use widgets::{
    notif_bar::{Notification, NotificationContainer},
    Canvas, Named,
};

fn main() {
    let app_state = AppState {
        notifications: Arc::new(Vec::new()),
        image: None,
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

#[derive(Clone, Data, Lens)]
struct AppState {
    notifications: Arc<Vec<Notification>>,
    image: Option<(Arc<std::path::PathBuf>, Arc<image::DynamicImage>)>,
}

impl AppState {
    fn show_notification(&mut self, n: Notification) {
        Arc::make_mut(&mut self.notifications).push(n);
    }

    fn do_open_image(&mut self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        self.image = Some((Arc::new(path.to_owned()), Arc::new(image::open(path)?)));
        Ok(())
    }

    fn do_new_image(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let img = get_image_from_clipboard()?
            .ok_or_else(|| "Clipboard is empty / non-image".to_string())?;

        self.image = Some((
            Arc::new(std::path::Path::new("Untitled").into()),
            Arc::new(img),
        ));
        Ok(())
    }

    fn do_save_as_image(
        &mut self,
        path: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (_, img) = self.image.take().ok_or_else(|| "No image was found.")?;
        img.save(path)?;
        self.image = Some((Arc::new(path.to_path_buf()), img));
        Ok(())
    }

    fn image_file_name(&self) -> String {
        match &self.image {
            None => "Untitled".into(),
            Some((path, _)) => path.to_string_lossy().into(),
        }
    }

    fn update_menu(&self, ctx: &mut DelegateCtx) {
        ctx.submit_command(
            druid::Command::new(druid::commands::SET_MENU, menu::make_menu(self)),
            None,
        );
    }
}

impl Delegate {
    fn handle_command(
        &mut self,
        data: &mut AppState,
        ctx: &mut DelegateCtx,
        cmd: &druid::Command,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match &cmd.selector {
            &commands::FILE_EXIT_ACTION => {
                ctx.submit_command(druid::commands::CLOSE_WINDOW.into(), None);
            }
            &commands::FILE_NEW_ACTION => {
                data.do_new_image()?;
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
                    data.show_notification(Notification::error(err.description()));
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

    let image_lens = AppState::image.map(
        |it| it.clone().map(|it| it.1),
        |to: &mut _, from| {
            if let Some(s) = to.as_mut() {
                if let Some(f) = from {
                    s.1 = f;
                }
            }
        },
    );

    let main_content = Either::new(
        |data: &AppState, &_| !data.image.is_some(),
        Align::centered(Padding::new(10.0, label)),
        Align::centered(Padding::new(
            10.0,
            Named::new(
                Scroll::new(Canvas::new().lens(image_lens)),
                |data: &AppState, _env: &_| data.image_file_name(),
            ),
        )),
    );

    NotificationContainer::new(main_content, AppState::notifications)
}
