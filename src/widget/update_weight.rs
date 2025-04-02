use chrono::NaiveDate;
use iced::{
    widget::{column, row, Button, Text},
    Element, Length,
};

use crate::{
    app::{Context, Message, NextWidget},
    data::Weight,
};

use super::{sidebar::sidebar, InputFormField, InputFormFieldError, Widget};

#[derive(Debug, Clone)]
pub enum UpdateWeightMessage {
    UpdateWeight(String),
    Submit,
}

impl From<UpdateWeightMessage> for Message {
    fn from(value: UpdateWeightMessage) -> Self {
        Message::UpdateWeight(value)
    }
}

#[derive(Debug)]
pub struct UpdateWeight {
    day: NaiveDate,
    weight: InputFormField<f64>,
}

impl UpdateWeight {
    pub fn new(w: Weight) -> Self {
        UpdateWeight {
            day: w.day,
            weight: InputFormField::new_with_raw_value(
                "Weight* (g)",
                "80.1",
                w.weight.to_string().as_str(),
            ),
        }
    }

    pub fn parse(&mut self) -> Result<Weight, String> {
        self.weight.validate(|input| {
            if input.is_empty() {
                Err(InputFormFieldError::MissingRequiredValue)
            } else {
                match input.parse::<f64>() {
                    Err(_) => Err(InputFormFieldError::InvalidNumber),
                    Ok(val) if val < 0.0 => Err(InputFormFieldError::SmallerThanZero),
                    Ok(val) => Ok(val),
                }
            }
        });

        Ok(Weight {
            day: self.day,
            weight: self.weight.value.ok_or("validation failed")?,
        })
    }
}

impl Widget for UpdateWeight {
    fn view(&self) -> Element<Message> {
        let form = column![self
            .weight
            .view(|w| { UpdateWeightMessage::UpdateWeight(w).into() }),]
        .spacing(10);

        let content = column![
            Text::new(format!("Update weight for {}", self.day.format("%Y-%m-%d"))).size(40),
            form,
            Button::new("Update").on_press(UpdateWeightMessage::Submit.into())
        ]
        .spacing(10);

        row![sidebar(), content]
            .height(Length::Fill)
            .padding(20)
            .spacing(20)
            .into()
    }

    fn update(&mut self, ctx: &mut Context, msg: Message) {
        if let Message::UpdateWeight(msg) = msg {
            match msg {
                UpdateWeightMessage::UpdateWeight(raw_weight) => {
                    self.weight.raw_input = raw_weight;
                }
                UpdateWeightMessage::Submit => {
                    if let Ok(weight) = self.parse() {
                        if let Err(err) = ctx.data.weight.update(weight) {
                            tracing::error!("Failed to update weight: {}", err);
                            panic!();
                        }
                        ctx.next_widget = Some(NextWidget::WeightList);
                    };
                }
            }
        }
    }
}
