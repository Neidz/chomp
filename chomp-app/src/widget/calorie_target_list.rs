use chomp_services::CalorieTarget;
use chrono::NaiveDate;
use iced::{
    widget::{column, row, Button, Container, Scrollable, Text},
    Alignment, Element, Length, Task,
};

use crate::app::{Context, Message, NextWidget};

use super::{sidebar, style::TableRowStyle, Widget};

#[derive(Debug, Clone)]
pub enum CalorieTargetListMessage {
    RedirectToCreate,
    DeleteTarget(NaiveDate),
}

impl From<CalorieTargetListMessage> for Message {
    fn from(value: CalorieTargetListMessage) -> Self {
        Message::CalorieTargetList(value)
    }
}

#[derive(Debug)]
pub struct CalorieTargetList {
    targets: Vec<CalorieTarget>,
}

impl CalorieTargetList {
    pub fn new(targets: Vec<CalorieTarget>) -> Self {
        CalorieTargetList { targets }
    }

    fn refresh(&mut self, ctx: &Context) {
        self.targets = ctx.services.calorie_target.list().unwrap_or_default();
    }
}

impl Widget for CalorieTargetList {
    fn view(&self) -> Element<Message> {
        let mut table = column![list_header_row()];
        for (i, target) in self.targets.iter().enumerate() {
            table = table.push(list_row(target, i % 2 == 0))
        }

        let content = column![
            row![
                Text::new("Calorie target list").size(40),
                Button::new("+").on_press(CalorieTargetListMessage::RedirectToCreate.into())
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
        if let Message::CalorieTargetList(msg) = msg {
            match msg {
                CalorieTargetListMessage::RedirectToCreate => {
                    ctx.next_widget = Some(NextWidget::CreateCalorieTarget);
                }
                CalorieTargetListMessage::DeleteTarget(day) => {
                    if let Err(err) = ctx.services.calorie_target.delete(day) {
                        tracing::error!("Failed to delete calorie target: {}", err);
                        panic!();
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
        Text::new("Calories (kcal/day)").width(Length::Fill),
        Text::new("Fats (g/day)").width(Length::Fill),
        Text::new("Proteins (g/day)").width(Length::Fill),
        Text::new("Carbohydrates (g/day)").width(Length::Fill),
        Text::new("Actions").width(Length::Fill)
    ]
    .padding(10)
    .width(Length::Fill);

    Container::new(row).width(Length::Fill).into()
}

fn list_row(t: &CalorieTarget, even: bool) -> Element<Message> {
    let row = row![
        Text::new(format!("{}", t.day.format("%Y-%m-%d")),).width(Length::Fill),
        Text::new(format!("{:.1}", t.calories)).width(Length::Fill),
        Text::new(format!("{:.1}", t.fats)).width(Length::Fill),
        Text::new(format!("{:.1}", t.proteins)).width(Length::Fill),
        Text::new(format!("{:.1}", t.carbohydrates)).width(Length::Fill),
        row![
            Button::new("Update").on_press(Message::ChangeWidget(NextWidget::UpdateCalorieTarget(
                t.day
            ))),
            Button::new("Delete").on_press(CalorieTargetListMessage::DeleteTarget(t.day).into())
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
