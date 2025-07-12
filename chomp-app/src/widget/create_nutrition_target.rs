use chomp_services::{NutritionTarget, ServiceError};
use iced::{
    widget::{column, row, Button, Text},
    Element, Length, Task,
};

use crate::app::{Context, Message, NextWidget};

use super::{modal, sidebar, DatePicker, InputFormField, InputFormFieldError, Widget};

#[derive(Debug, Clone)]
pub enum CreateNutritionTargetMessage {
    UpdateCalories(String),
    UpdateFats(String),
    UpdateProteins(String),
    UpdateCarbohydrates(String),
    Submit,
}

impl From<CreateNutritionTargetMessage> for Message {
    fn from(value: CreateNutritionTargetMessage) -> Self {
        Message::CreateNutritionTarget(value)
    }
}

#[derive(Debug)]
pub struct CreateNutritionTarget {
    day: DatePicker,
    calories: InputFormField<f32>,
    fats: InputFormField<f32>,
    proteins: InputFormField<f32>,
    carbohydrates: InputFormField<f32>,
}

impl CreateNutritionTarget {
    pub fn new() -> Self {
        CreateNutritionTarget {
            day: DatePicker::new("Date*"),
            calories: InputFormField::new("Calories* (kcal/day)", "2500.0"),
            fats: InputFormField::new_with_raw_value("Fats* (%)", "20.0", "20"),
            proteins: InputFormField::new_with_raw_value("Proteins* (%)", "30.0", "30"),
            carbohydrates: InputFormField::new_with_raw_value("Carbohydrates* (%)", "50.0", "50"),
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

        let calories = self.calories.value.ok_or("validation failed")?;

        self.fats.validate(|input| {
            if input.is_empty() {
                Err(InputFormFieldError::MissingRequiredValue)
            } else {
                match input.parse::<f32>() {
                    Err(_) => Err(InputFormFieldError::InvalidNumber),
                    Ok(val) if val < 0.0 => Err(InputFormFieldError::SmallerThanZero),
                    Ok(val) if val > 100.0 => Err(InputFormFieldError::Custom(
                        "Invalid number. Must be smaller or equal to 100.0".to_string(),
                    )),
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
                    Ok(val) if val > 100.0 => Err(InputFormFieldError::Custom(
                        "Invalid number. Must be smaller or equal to 100.0".to_string(),
                    )),
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
                    Ok(val) if val > 100.0 => Err(InputFormFieldError::Custom(
                        "Invalid number. Must be smaller or equal to 100.0".to_string(),
                    )),
                    Ok(val) => Ok(val),
                }
            }
        });

        let fats_percentage = self.fats.value.ok_or("validation failed")?;
        let proteins_percentage = self.proteins.value.ok_or("validation failed")?;
        let carbohydrates_percentage = self.carbohydrates.value.ok_or("validation failed")?;

        let sum = fats_percentage + proteins_percentage + carbohydrates_percentage;
        if (sum - 100.0).abs() > 0.01 {
            self.calories.error = Some(InputFormFieldError::Custom(format!(
                "fats, proteins and carbohydrates must sum to 100.0%, current sum : {sum}%"
            )));
            return Err("validation failed".to_string());
        }

        let fats = fats_percentage / 100.0 * calories / 9.0;
        let proteins = proteins_percentage / 100.0 * calories / 4.0;
        let carbohydrates = carbohydrates_percentage / 100.0 * calories / 4.0;

        Ok(NutritionTarget {
            day: self.day.value(),
            calories,
            fats,
            proteins,
            carbohydrates,
        })
    }
}

impl Widget for CreateNutritionTarget {
    fn view(&self) -> Element<Message> {
        let form = column![
            self.day.view(),
            self.calories
                .view(|c| { CreateNutritionTargetMessage::UpdateCalories(c).into() }),
            self.fats
                .view(|f| { CreateNutritionTargetMessage::UpdateFats(f).into() }),
            self.proteins
                .view(|p| { CreateNutritionTargetMessage::UpdateProteins(p).into() }),
            self.carbohydrates
                .view(|c| { CreateNutritionTargetMessage::UpdateCarbohydrates(c).into() }),
        ]
        .spacing(10);

        let content = column![
            Text::new("Create nutrition target").size(40),
            form,
            Button::new("Create").on_press(CreateNutritionTargetMessage::Submit.into())
        ]
        .spacing(10);

        let content_with_sidebar = row![sidebar(), content]
            .height(Length::Fill)
            .padding(20)
            .spacing(20);

        return modal(
            content_with_sidebar.into(),
            self.day.view_modal(),
            Message::CloseDatePicker.into(),
            self.day.calendar_open(),
        )
        .into();
    }

    fn update(&mut self, ctx: &mut Context, msg: Message) -> Task<Message> {
        self.day.handle_message(msg.clone());

        if let Message::CreateNutritionTarget(msg) = msg {
            match msg {
                CreateNutritionTargetMessage::UpdateCalories(raw_calories) => {
                    self.calories.raw_input = raw_calories;
                }
                CreateNutritionTargetMessage::UpdateFats(raw_fats) => {
                    self.fats.raw_input = raw_fats;
                }
                CreateNutritionTargetMessage::UpdateProteins(raw_proteins) => {
                    self.proteins.raw_input = raw_proteins;
                }
                CreateNutritionTargetMessage::UpdateCarbohydrates(raw_carbohydrates) => {
                    self.carbohydrates.raw_input = raw_carbohydrates;
                }
                CreateNutritionTargetMessage::Submit => {
                    if let Ok(target) = self.parse() {
                        if let Some(err) = ctx.services.nutrition_target.create(target).err() {
                            match err {
                                ServiceError::UniqueConstraintViolation(unique_field)
                                    if unique_field == "nutrition_targets.day" =>
                                {
                                    self.day.error = Some(InputFormFieldError::Custom(
                                        "Target with this date already exists".to_string(),
                                    ))
                                }
                                _ => {
                                    eprintln!("Error: {err:?}");
                                }
                            }
                        } else {
                            ctx.next_widget = Some(NextWidget::NutritionTargetList);
                        }
                    };
                }
            }
        };

        Task::none()
    }
}
