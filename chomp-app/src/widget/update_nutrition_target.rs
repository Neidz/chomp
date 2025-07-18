use chomp_services::NutritionTarget;
use chrono::NaiveDate;
use iced::{
    widget::{column, row, Button, Text},
    Element, Length, Task,
};

use crate::app::{Context, Message, NextWidget};

use super::{sidebar::sidebar, InputFormField, InputFormFieldError, Widget};

#[derive(Debug, Clone)]
pub enum UpdateNutritionTargetMessage {
    UpdateCalories(String),
    UpdateFats(String),
    UpdateProteins(String),
    UpdateCarbohydrates(String),
    Submit,
}

impl From<UpdateNutritionTargetMessage> for Message {
    fn from(value: UpdateNutritionTargetMessage) -> Self {
        Message::UpdateNutritionTarget(value)
    }
}

#[derive(Debug)]
pub struct UpdateNutritionTarget {
    day: NaiveDate,
    calories: InputFormField<f32>,
    fats: InputFormField<f32>,
    proteins: InputFormField<f32>,
    carbohydrates: InputFormField<f32>,
}

impl UpdateNutritionTarget {
    pub fn new(t: NutritionTarget) -> Self {
        UpdateNutritionTarget {
            day: t.day,
            calories: InputFormField::new_with_raw_value(
                "Calories* (kcal/day)",
                "2500.0",
                t.calories.to_string().as_str(),
            ),
            fats: InputFormField::new_with_raw_value(
                "Fats* (g/day)",
                "80.0",
                t.fats.to_string().as_str(),
            ),
            proteins: InputFormField::new_with_raw_value(
                "Proteins* (g/day)",
                "200.0",
                t.proteins.to_string().as_str(),
            ),
            carbohydrates: InputFormField::new_with_raw_value(
                "Carbohydrates* (g/day)",
                "245.0",
                t.carbohydrates.to_string().as_str(),
            ),
        }
    }

    pub fn parse(&mut self) -> Result<NutritionTarget, String> {
        self.calories.validate(|input| {
            if input.is_empty() {
                Err(InputFormFieldError::MissingRequiredValue)
            } else {
                match input.parse::<f32>() {
                    Err(_) => Err(InputFormFieldError::InvalidNumber),
                    Ok(val) if val < 0.0 => Err(InputFormFieldError::SmallerThanZero),
                    Ok(val) => Ok(val),
                }
            }
        });

        self.fats.validate(|input| {
            if input.is_empty() {
                Err(InputFormFieldError::MissingRequiredValue)
            } else {
                match input.parse::<f32>() {
                    Err(_) => Err(InputFormFieldError::InvalidNumber),
                    Ok(val) if val < 0.0 => Err(InputFormFieldError::SmallerThanZero),
                    Ok(val) => Ok(val),
                }
            }
        });

        self.proteins.validate(|input| {
            if input.is_empty() {
                Err(InputFormFieldError::MissingRequiredValue)
            } else {
                match input.parse::<f32>() {
                    Err(_) => Err(InputFormFieldError::InvalidNumber),
                    Ok(val) if val < 0.0 => Err(InputFormFieldError::SmallerThanZero),
                    Ok(val) => Ok(val),
                }
            }
        });

        self.carbohydrates.validate(|input| {
            if input.is_empty() {
                Err(InputFormFieldError::MissingRequiredValue)
            } else {
                match input.parse::<f32>() {
                    Err(_) => Err(InputFormFieldError::InvalidNumber),
                    Ok(val) if val < 0.0 => Err(InputFormFieldError::SmallerThanZero),
                    Ok(val) => Ok(val),
                }
            }
        });

        Ok(NutritionTarget {
            day: self.day,
            calories: self.calories.value.ok_or("validation failed")?,
            fats: self.fats.value.ok_or("validation failed")?,
            proteins: self.proteins.value.ok_or("validation failed")?,
            carbohydrates: self.carbohydrates.value.ok_or("validation failed")?,
        })
    }
}

impl Widget for UpdateNutritionTarget {
    fn view(&self) -> Element<Message> {
        let form = column![
            self.calories
                .view(|c| { UpdateNutritionTargetMessage::UpdateCalories(c).into() }),
            self.fats
                .view(|f| { UpdateNutritionTargetMessage::UpdateFats(f).into() }),
            self.proteins
                .view(|p| { UpdateNutritionTargetMessage::UpdateProteins(p).into() }),
            self.carbohydrates
                .view(|c| { UpdateNutritionTargetMessage::UpdateCarbohydrates(c).into() }),
        ]
        .spacing(10);

        let content = column![
            Text::new(format!(
                "Update nutrition target for {}",
                self.day.format("%Y-%m-%d")
            ))
            .size(40),
            form,
            Button::new("Update").on_press(UpdateNutritionTargetMessage::Submit.into())
        ]
        .spacing(10);

        row![sidebar(), content]
            .height(Length::Fill)
            .padding(20)
            .spacing(20)
            .into()
    }

    fn update(&mut self, ctx: &mut Context, msg: Message) -> Task<Message> {
        if let Message::UpdateNutritionTarget(msg) = msg {
            match msg {
                UpdateNutritionTargetMessage::UpdateCalories(raw_calories) => {
                    self.calories.raw_input = raw_calories;
                }
                UpdateNutritionTargetMessage::UpdateFats(raw_fats) => {
                    self.fats.raw_input = raw_fats;
                }
                UpdateNutritionTargetMessage::UpdateProteins(raw_proteins) => {
                    self.proteins.raw_input = raw_proteins;
                }
                UpdateNutritionTargetMessage::UpdateCarbohydrates(raw_carbohydrates) => {
                    self.carbohydrates.raw_input = raw_carbohydrates;
                }
                UpdateNutritionTargetMessage::Submit => {
                    if let Ok(target) = self.parse() {
                        if let Err(err) = ctx.services.nutrition_target.update(target) {
                            tracing::error!("Failed to update nutrition target: {}", err);
                            std::process::exit(1);
                        }
                        ctx.next_widget = Some(NextWidget::NutritionTargetList);
                    };
                }
            }
        };

        Task::none()
    }
}
