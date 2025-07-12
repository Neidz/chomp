use chomp_services::Weight;
use chrono::NaiveDate;
use iced::{
    widget::{button, column, row, Button, Container, Scrollable, Text},
    Alignment, Element, Length, Task,
};

use crate::app::{Context, Message, NextWidget};

use super::{sidebar::sidebar, style::TableRowStyle, Widget};

#[derive(Debug, Clone)]
pub enum WeightListMessage {
    RedirectToCreate,
    DeleteWeight(NaiveDate),
}

impl From<WeightListMessage> for Message {
    fn from(value: WeightListMessage) -> Self {
        Message::WeightList(value)
    }
}

#[derive(Debug)]
pub struct WeightList {
    weights: Vec<Weight>,
}

impl WeightList {
    pub fn new(weights: Vec<Weight>) -> Self {
        WeightList { weights }
    }

    fn refresh(&mut self, ctx: &Context) {
        self.weights = ctx.services.weight.list().unwrap_or_default();
    }
}

impl Widget for WeightList {
    fn view(&self) -> Element<Message> {
        let mut table = column![list_header_row()];
        for (i, weight) in self.weights.iter().enumerate() {
            table = table.push(list_row(weight, i % 2 == 0))
        }

        let content = column![
            row![
                Text::new("Weight list").size(40),
                Button::new("+").on_press(WeightListMessage::RedirectToCreate.into())
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            Scrollable::new(table)
        ]
        .spacing(10);

        row![sidebar(), content]
            .height(Length::Fill)
            .padding(20)
            .spacing(20)
            .into()
    }

    fn update(&mut self, ctx: &mut Context, msg: Message) -> Task<Message> {
        if let Message::WeightList(msg) = msg {
            match msg {
                WeightListMessage::RedirectToCreate => {
                    ctx.next_widget = Some(NextWidget::CreateWeight);
                }
                WeightListMessage::DeleteWeight(day) => {
                    if let Err(err) = ctx.services.weight.delete(day) {
                        tracing::error!("Failed to delete weight: {}", err);
                        std::process::exit(1);
                    }
                    self.refresh(ctx);
                }
            }
        };

        Task::none()
    }
}

fn list_header_row() -> Element<'static, Message> {
    let row = row![
        Text::new("Day").width(Length::Fill),
        Text::new("Weight (g)").width(Length::Fill),
        Text::new("Actions").width(Length::Fill)
    ]
    .padding(10)
    .width(Length::Fill);

    Container::new(row).width(Length::Fill).into()
}

fn list_row(w: &Weight, even: bool) -> Element<Message> {
    let row = row![
        Text::new(format!("{}", w.day.format("%Y-%m-%d")),).width(Length::Fill),
        Text::new(format!("{:.1}", w.weight)).width(Length::Fill),
        row![
            Button::new("Update").on_press(Message::ChangeWidget(NextWidget::UpdateWeight(w.day))),
            Button::new("Delete")
                .style(button::danger)
                .on_press(WeightListMessage::DeleteWeight(w.day).into())
        ]
        .spacing(10)
        .width(Length::Fill)
    ]
    .padding(10)
    .width(Length::Fill);

    Container::new(row)
        .width(Length::Fill)
        .style(move |t| {
            if even {
                TableRowStyle::Even.style(t)
            } else {
                TableRowStyle::Odd.style(t)
            }
        })
        .into()
}
