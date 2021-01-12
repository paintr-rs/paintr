use druid::{
    lens::{self, LensExt},
    Color, Command, Data, Env, Lens, Target, Widget,
};
use druid::{
    widget::{Button, Flex, Label, LabelText, LensWrap, Parse, TextBox, WidgetExt},
    TextAlignment,
};

use crate::commands::NEW_IMAGE_ACTION;
use paintr_widgets::widgets::Modal;

use image::GenericImageView;
use paintr_core::get_image_from_clipboard;

#[derive(Eq, PartialEq, Clone, Debug, Data)]
enum DialogState {
    Opened,
    Closed,
    Cancel,
}

// FIXME: druid Lens and Data proc-macro do not support generic arguments
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Dialog<T: Data> {
    state: DialogState,
    kind: T,
}

impl<T: Data> Dialog<T> {
    fn new(kind: T) -> Self {
        Dialog { state: DialogState::Opened, kind }
    }
}

// We don't need to compare the internal data, as they will change by interaction.
// By not comparing them, the widget will not be recreated.
impl<T: Data> Data for Dialog<T> {
    fn same(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

#[derive(Data, Eq, PartialEq, Clone, Lens, Debug, Default)]
pub struct NewFileSettings {
    pub width: Option<u32>,
    pub height: Option<u32>,
}

fn make_label<T: Data + 'static>(text: impl Into<LabelText<T>>) -> impl Widget<T> {
    let row_padding = (10.0, 2.5);
    Label::new(text).with_text_alignment(TextAlignment::Start).padding(row_padding).fix_width(100.0)
}

macro_rules! dialog_lens {
    ($ty:ident, $name:ident) => {
        druid::lens!(Dialog<$ty>, kind).then($ty::$name)
    };
}

impl NewFileSettings {
    fn widget(&self) -> impl Widget<Dialog<NewFileSettings>> {
        let ok_button =
            Button::new(L!("Ok")).on_click(|_, data: &mut Dialog<NewFileSettings>, _: &Env| {
                if data.kind.width.is_none() || data.kind.height.is_none() {
                    return;
                }
                data.state = DialogState::Closed;
            });

        let cancel_button =
            Button::new(L!("Cancel")).on_click(|_, data: &mut Dialog<NewFileSettings>, _: &Env| {
                data.state = DialogState::Cancel;
            });

        let row_padding = 2.5;

        Flex::column()
            .with_child(
                Flex::row()
                    .with_child(make_label(L!("Width :")))
                    .with_flex_child(
                        Parse::new(TextBox::new().with_placeholder("100"))
                            .padding(row_padding)
                            .lens(dialog_lens!(NewFileSettings, width)),
                        1.0,
                    )
                    .padding((3.0, row_padding)),
            )
            .with_child(
                Flex::row()
                    .with_child(make_label(L!("Height :")))
                    .with_flex_child(
                        Parse::new(TextBox::new().with_placeholder("100"))
                            .padding(row_padding)
                            .lens(dialog_lens!(NewFileSettings, height)),
                        1.0,
                    )
                    .padding((3.0, row_padding)),
            )
            .with_child(
                Flex::row()
                    .with_flex_child(ok_button.padding(5.0).center(), 1.0)
                    .with_flex_child(cancel_button.padding(5.0).center(), 1.0)
                    .padding((3.0, 5.0)),
            )
            .fix_width(300.0)
            .fix_height(100.0)
            .background(Color::grey(0.3))
            .center()
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Data)]
pub enum DialogData {
    NewFileSettings(Dialog<NewFileSettings>),
}

impl Modal for DialogData {
    fn is_closed(&self) -> Option<Command> {
        match self {
            DialogData::NewFileSettings(it) if it.state == DialogState::Closed => {
                Some(Command::new(NEW_IMAGE_ACTION, it.kind.clone(), Target::Auto))
            }
            _ => None,
        }
    }
}

impl DialogData {
    pub fn widget(&self) -> Option<Box<dyn Widget<DialogData>>> {
        match self {
            DialogData::NewFileSettings(dialog) => match dialog.state {
                DialogState::Cancel | DialogState::Closed => None,
                DialogState::Opened => {
                    let w = LensWrap::new(
                        dialog.kind.widget(),
                        lens::Identity.map(
                            |x: &DialogData| match x {
                                DialogData::NewFileSettings(it) => it.clone(),
                            },
                            |x: &mut DialogData, y: Dialog<NewFileSettings>| {
                                *x = DialogData::NewFileSettings(y);
                            },
                        ),
                    );

                    Some(Box::new(w))
                }
            },
        }
    }

    pub fn new_file_settings() -> DialogData {
        let mut nfs = NewFileSettings::default();

        match get_image_from_clipboard() {
            Result::Ok(Some(img)) => {
                nfs.width = Some(img.width());
                nfs.height = Some(img.height());
            }
            _ => (),
        }

        DialogData::NewFileSettings(Dialog::new(nfs))
    }
}
