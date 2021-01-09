use druid::{theme, Color, Env, Widget};
use druid::{
    widget::{Align, Either, Flex, Label, Padding, Scroll, Svg, WidgetExt},
    TextAlignment,
};

use crate::widgets::{notif_bar::NotificationContainer, Editor, ModalContainer, Named, RadioGroup};
use crate::{AppState, EditorState, ToolKind};

fn canvas() -> impl Widget<AppState> {
    let editor_lens = AppState::editor;

    Either::new(
        |data: &AppState, &_| !data.editor.canvas.is_some(),
        Align::centered(Padding::new(10.0, Label::new(L!("paintr-front-page-welcome")))),
        Align::centered(Padding::new(
            10.0,
            Named::new(
                Scroll::new(Editor::new().lens(editor_lens)),
                |data: &AppState, _env: &_| data.image_file_name(),
            ),
        )),
    )
}

fn toolbar() -> impl Widget<AppState> {
    let button_size = 24.0;

    let move_tool_icon = include_str!("assets/icons/move_tool.svg");
    let rect_marquee_tool_icon = include_str!("assets/icons/rect_marquee_tool.svg");

    let buttons: Vec<(Box<dyn Widget<_>>, _)> = vec![
        (
            Box::new(
                Svg::new(move_tool_icon.parse().unwrap())
                    .fix_width(button_size)
                    .fix_height(button_size),
            ),
            ToolKind::Move,
        ),
        (
            Box::new(
                Svg::new(rect_marquee_tool_icon.parse().unwrap())
                    .fix_width(button_size)
                    .fix_height(button_size),
            ),
            ToolKind::Select,
        ),
    ];

    RadioGroup::new(buttons).lens(EditorState::tool).lens(AppState::editor).padding(5.0)
}

pub(crate) fn ui_builder() -> impl Widget<AppState> {
    let content = Flex::row().with_child(toolbar()).with_flex_child(canvas(), 1.0);

    let container = ModalContainer::new(
        NotificationContainer::new(content, AppState::notifications),
        |modal, _| modal.widget(),
        AppState::modal,
    );

    Flex::column().with_flex_child(container, 1.0).with_child(
        Label::new(|data: &AppState, _env: &Env| data.status().unwrap_or_default())
            .with_text_alignment(TextAlignment::End)
            .padding((5.0, 3.0))
            .background(Color::rgb(0.5, 0.3, 0.5))
            .env_scope(|env, _| {
                env.set(theme::TEXT_SIZE_NORMAL, 12.0);
            }),
    )
}
