use druid::widget::{Align, Either, Flex, Label, Padding, Scroll, WidgetExt};
use druid::{theme, Color, Env, UnitPoint, Widget};

use crate::widgets::{notif_bar::NotificationContainer, Canvas, ModalContainer, Named};
use crate::AppState;

fn canvas() -> impl Widget<AppState> {
    Either::new(
        |data: &AppState, &_| !data.canvas.is_some(),
        Align::centered(Padding::new(10.0, Label::new(L!("paintr-front-page-welcome")))),
        Align::centered(Padding::new(
            10.0,
            Named::new(
                Scroll::new(Canvas::new().lens(AppState::canvas)),
                |data: &AppState, _env: &_| data.image_file_name(),
            ),
        )),
    )
}

pub(crate) fn ui_builder() -> impl Widget<AppState> {
    let content = Flex::row().with_child(canvas(), 1.0);

    let container = ModalContainer::new(
        NotificationContainer::new(content, AppState::notifications),
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
