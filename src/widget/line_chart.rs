use iced::{
    event::Status,
    mouse::{Cursor, Interaction},
    widget::canvas::{self, Cache, Event, Geometry, Path, Stroke},
    Point, Rectangle, Renderer, Size, Theme,
};

use crate::app::Message;

#[derive(Debug)]
pub struct LineChart {
    pub cache: Cache,
}

#[allow(unused)]
impl LineChart {
    pub fn new() -> Self {
        LineChart {
            cache: Cache::new(),
        }
    }
}

impl canvas::Program<Message> for LineChart {
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
            let start = Point {
                x: frame.center().x - 30.0,
                y: frame.center().y - 30.0,
            };
            let end = Point {
                x: frame.center().x + 30.0,
                y: frame.center().y + 30.0,
            };

            let path = Path::line(start, end);
            let stroke = Stroke::default().with_width(4.0);
            frame.stroke(&path, stroke);
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
