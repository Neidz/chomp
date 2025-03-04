use iced::{
    event::Status,
    mouse::{Cursor, Interaction},
    widget::{
        canvas::{self, Cache, Event, Geometry},
        column, row, Canvas, Text,
    },
    Element,
    Length::{self, Fill},
    Rectangle, Renderer, Size, Theme,
};

use crate::app::{Context, Message};

use super::{sidebar::sidebar, Widget};

#[derive(Debug, Clone)]
pub enum DashboardMessage {}

impl From<DashboardMessage> for Message {
    fn from(value: DashboardMessage) -> Self {
        Message::Dashboard(value)
    }
}

#[derive(Debug)]
pub struct Dashboard {
    cache: Cache,
}

impl Dashboard {
    pub fn new() -> Self {
        Dashboard {
            cache: Cache::new(),
        }
    }
}

impl Widget for Dashboard {
    fn view(&self) -> Element<Message> {
        let canvas = Canvas::new(self).width(Fill).height(Fill);
        let content = column![Text::new("Dashboard").size(40), canvas].spacing(10);

        row![sidebar(), content]
            .height(Length::Fill)
            .padding(20)
            .spacing(20)
            .into()
    }

    fn update(&mut self, _ctx: &mut Context, _msg: Message) {}
}

impl canvas::Program<Message> for Dashboard {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        _event: Event,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> (Status, Option<Message>) {
        (Status::Captured, None)
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let text_color = theme.extended_palette().primary.base.color;
        let graph_fill = canvas::Fill {
            style: canvas::Style::Solid(text_color),
            ..Default::default()
        };

        let graph = self.cache.draw(renderer, bounds.size(), |frame| {
            frame.fill_rectangle(frame.center(), Size::new(10.0, 10.0), graph_fill);
        });
        vec![graph]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> Interaction {
        Interaction::None
    }
}
