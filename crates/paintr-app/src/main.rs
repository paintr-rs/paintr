use druid::piet::Color;
use druid::widget::{Align, Either, Flex, Label, Padding, Scroll, WidgetExt};
use druid::{
    theme, AppDelegate, AppLauncher, Application, Data, DelegateCtx, Env, Event, Lens,
    LocalizedString, Widget, WindowDesc, WindowId,
};

macro_rules! L {
    ($str:literal) => {
        $crate::LocalizedString::new($str)
    };
}

mod commands;
mod menu;
mod widgets;
use widgets::{Canvas, SnackBarContainer};

use std::sync::Arc;

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
        .launch(State::default())
        .expect("launch failed");
}

struct Delegate;

#[derive(Clone, Data, Default, Lens)]
struct State {
    notifications: Arc<Vec<Arc<String>>>,
    image: Option<Arc<image::DynamicImage>>,
}

impl State {
    fn show_notification(&mut self, s: &str) {
        Arc::make_mut(&mut self.notifications).push(Arc::new(s.into()));
    }

    fn do_open_image(&mut self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        self.image = Some(Arc::new(image::open(path)?));
        Ok(())
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

    let main_content = Either::new(
        |data: &State, &_| !data.image.is_some(),
        Align::centered(Padding::new(5.0, label)),
        Align::centered(Padding::new(
            5.0,
            Scroll::new(Canvas::new().lens(State::image)),
        )),
    );

    SnackBarContainer::new(
        Flex::column().with_child(main_content, 1.0),
        State::notifications,
    )
}
