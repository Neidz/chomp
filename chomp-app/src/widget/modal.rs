use iced::{
    widget::{center, container, mouse_area, opaque, stack},
    Color, Element,
};

use crate::app::Message;

pub fn modal<'a>(
    view: Element<'a, Message>,
    modal_view: Element<'a, Message>,
    on_blur: Message,
    modal_active: bool,
) -> Element<'a, Message> {
    if !modal_active {
        return view;
    }

    stack![
        view,
        opaque(
            mouse_area(center(opaque(modal_view)).style(|_theme| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.8,
                            ..Color::BLACK
                        }
                        .into(),
                    ),

                    ..container::Style::default()
                }
            }))
            .on_press(on_blur)
        )
    ]
    .into()
}
