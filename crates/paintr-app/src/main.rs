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
use widgets::{Canvas, Named, SnackBarContainer};

fn main() {
    let main_window = WindowDesc::new(ui_builder)
        .title(L!("paint-app-name"))
        .menu(menu::make_menu())
        .window_size((800.0, 600.0));

    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .configure_env(|env| {
            env.set(theme::WINDOW_BACKGROUND_COLOR, Color::rgb8(0, 0x77, 0x88));
        })
        .use_simple_logger()
        .launch(State::default())
        .expect("launch failed");
}

struct Delegate;

#[derive(Clone, Data, Default, Lens)]
struct State {
    notifications: Arc<Vec<Arc<String>>>,
    image: Option<(Arc<std::path::PathBuf>, Arc<image::DynamicImage>)>,
}

impl State {
    fn show_notification(&mut self, s: &str) {
        Arc::make_mut(&mut self.notifications).push(Arc::new(s.into()));
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

    fn image_file_name(&self) -> String {
        match &self.image {
            None => "Untitled".into(),
            Some((path, _)) => path.to_string_lossy().into(),
        }
    }
}

impl AppDelegate<State> for Delegate {
    fn event(
        &mut self,
        event: Event,
        data: &mut State,
        _env: &Env,
        ctx: &mut DelegateCtx,
    ) -> Option<Event> {
        match event {
            Event::Command(ref cmd) => match &cmd.selector {
                &commands::FILE_EXIT_ACTION => {
                    ctx.submit_command(druid::commands::CLOSE_WINDOW.into(), None);
                }
                &commands::FILE_NEW_ACTION => match data.do_new_image() {
                    Err(err) => {
                        data.show_notification(err.description());
                    }
                    Ok(_) => {
                        let s = format!("New file created");
                        data.show_notification(&s);
                    }
                },
                &druid::commands::OPEN_FILE => {
                    let info = cmd.get_object::<druid::FileInfo>().expect("api violation");
                    match data.do_open_image(info.path()) {
                        Err(err) => {
                            data.show_notification(err.description());
                        }
                        Ok(_) => {
                            let s = format!("{} opened", info.path().to_str().unwrap_or("????"));
                            data.show_notification(&s);
                        }
                    }
                }
                _ => (),
            },
            Event::KeyUp(_key) => {
                // FIXME: a workaound for druid do not implement Hotkey for menu
                #[cfg(target_os = "windows")]
                {
                    if let Some(cmd) = menu::find_command_by_hotkey(_key) {
                        ctx.submit_command(cmd, None);
                    }
                }
            }

            _ => (),
        };

        Some(event)
    }

    fn window_removed(
        &mut self,
        _id: WindowId,
        _data: &mut State,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        // FIXME: Use commands::QUIT_APP
        // It do not works right now, maybe a druid bug
        Application::quit();
    }
}

fn ui_builder() -> impl Widget<State> {
    let text = L!("paintr-front-page-welcome");
    let label = Label::new(text.clone());

    let image_lens = State::image.map(
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
        |data: &State, &_| !data.image.is_some(),
        Align::centered(Padding::new(10.0, label)),
        Align::centered(Padding::new(
            10.0,
            Named::new(
                Scroll::new(Canvas::new().lens(image_lens)),
                |data: &State, _env: &_| data.image_file_name(),
            ),
        )),
    );

    SnackBarContainer::new(main_content, State::notifications)
}
