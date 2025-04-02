use chrono::NaiveDate;
use iced::{
    widget::{column, row, Button, Text},
    Element, Length,
};

use crate::{
    app::{Context, Message, NextWidget},
    data::{DataError, Weight},
};

use super::{sidebar::sidebar, DayFormField, InputFormField, InputFormFieldError, Widget};

#[derive(Debug, Clone)]
pub enum CreateWeightMessage {
    UpdateDay(NaiveDate),
    UpdateWeight(String),
    Submit,
}

impl From<CreateWeightMessage> for Message {
    fn from(value: CreateWeightMessage) -> Self {
        Message::CreateWeight(value)
    }
}

#[derive(Debug)]
pub struct CreateWeight {
    day: DayFormField,
    weight: InputFormField<f64>,
}

impl CreateWeight {
    pub fn new() -> Self {
        CreateWeight {
            day: DayFormField::new("Date"),
            weight: InputFormField::new("Weight* (g)", "80.1"),
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
            day: self.day.value,
            weight: self.weight.value.ok_or("validation failed")?,
        })
    }
}

impl Widget for CreateWeight {
    fn view(&self) -> Element<Message> {
        let form = column![
            self.day
                .view(|d| { CreateWeightMessage::UpdateDay(d).into() }),
            self.weight
                .view(|w| { CreateWeightMessage::UpdateWeight(w).into() }),
        ]
        .spacing(10);

        let content = column![
            Text::new("Create weight").size(40),
            form,
            Button::new("Create").on_press(CreateWeightMessage::Submit.into())
        ]
        .spacing(10);

        row![sidebar(), content]
            .height(Length::Fill)
            .padding(20)
            .spacing(20)
            .into()
    }

    fn update(&mut self, ctx: &mut Context, msg: Message) {
        if let Message::CreateWeight(msg) = msg {
            match msg {
                CreateWeightMessage::UpdateDay(day) => {
                    self.day.value = day;
                }
                CreateWeightMessage::UpdateWeight(raw_weight) => {
                    self.weight.raw_input = raw_weight;
                }
                CreateWeightMessage::Submit => {
                    if let Ok(weight) = self.parse() {
                        if let Some(err) = ctx.data.weight.create(weight).err() {
                            match err {
                                DataError::UniqueConstraintViolation(unique_field)
                                    if unique_field == "weights.day" =>
                                {
                                    self.day.error = Some(InputFormFieldError::Custom(
                                        "Weight with this date already exists".to_string(),
                                    ))
                                }
                                _ => {
                                    eprintln!("Error: {:?}", err);
                                }
                            }
                        } else {
                            ctx.next_widget = Some(NextWidget::WeightList);
                        }
                    };
                }
            }
        }
    }
}
