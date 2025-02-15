use iced::{
    widget::{center, container, mouse_area, opaque, stack},
    Color, Element,
};

use crate::app::Message;

pub fn render_modal<'a>(
    base: Element<'a, Message>,
    modal_content: Element<'a, Message>,
    on_blur: Message,
) -> Element<'a, Message> {
    stack![
        base,
        opaque(
            mouse_area(center(opaque(modal_content)).style(|_theme| {
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
