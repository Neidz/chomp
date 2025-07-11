use iced::{daemon::DefaultStyle, widget::container, Background, Color, Theme};

pub enum TableRowStyle {
    Odd,
    Even,
    Footer,
}

impl TableRowStyle {
    pub fn style(&self, theme: &Theme) -> container::Style {
        match self {
            TableRowStyle::Even => {
                let base_text_color = theme.default_style().text_color;

                let background_color = Color::from_rgba(
                    base_text_color.r,
                    base_text_color.g,
                    base_text_color.b,
                    0.02,
                );

                container::Style {
                    background: Some(Background::Color(background_color)),
                    ..container::Style::default()
                }
            }
            TableRowStyle::Odd => container::Style::default(),
            TableRowStyle::Footer => {
                let base_text_color = theme.default_style().text_color;

                let background_color =
                    Color::from_rgba(base_text_color.r, base_text_color.g, base_text_color.b, 0.1);

                container::Style {
                    background: Some(Background::Color(background_color)),
                    ..container::Style::default()
                }
            }
        }
    }
}
