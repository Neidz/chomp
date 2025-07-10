use chrono::NaiveDate;
use iced::{
    widget::{column, row, Button, Text},
    Element, Length, Task,
};

use crate::{
    app::{Context, Message, NextWidget},
    data::{CalorieTarget, DataError},
};

use super::{sidebar::sidebar, DayFormField, InputFormField, InputFormFieldError, Widget};

#[derive(Debug, Clone)]
pub enum CreateCalorieTargetMessage {
    UpdateDay(NaiveDate),
    UpdateCalories(String),
    UpdateFats(String),
    UpdateProteins(String),
    UpdateCarbohydrates(String),
    Submit,
}

impl From<CreateCalorieTargetMessage> for Message {
    fn from(value: CreateCalorieTargetMessage) -> Self {
        Message::CreateCalorieTarget(value)
    }
}

#[derive(Debug)]
pub struct CreateCalorieTarget {
    day: DayFormField,
    calories: InputFormField<f32>,
    fats: InputFormField<f32>,
    proteins: InputFormField<f32>,
    carbohydrates: InputFormField<f32>,
}

impl CreateCalorieTarget {
    pub fn new() -> Self {
        CreateCalorieTarget {
            day: DayFormField::new("Date*"),
            calories: InputFormField::new("Calories* (kcal/day)", "2500.0"),
            fats: InputFormField::new_with_raw_value("Fats* (%)", "20.0", "20"),
            proteins: InputFormField::new_with_raw_value("Proteins* (%)", "30.0", "30"),
            carbohydrates: InputFormField::new_with_raw_value("Carbohydrates* (%)", "50.0", "50"),
        }
    }

    pub fn parse(&mut self) -> Result<CalorieTarget, String> {
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

        Ok(CalorieTarget {
            day: self.day.value,
            calories,
            fats,
            proteins,
            carbohydrates,
        })
    }
}

impl Widget for CreateCalorieTarget {
    fn view(&self) -> Element<Message> {
        let form = column![
            self.day
                .view(|d| { CreateCalorieTargetMessage::UpdateDay(d).into() }),
            self.calories
                .view(|c| { CreateCalorieTargetMessage::UpdateCalories(c).into() }),
            self.fats
                .view(|f| { CreateCalorieTargetMessage::UpdateFats(f).into() }),
            self.proteins
                .view(|p| { CreateCalorieTargetMessage::UpdateProteins(p).into() }),
            self.carbohydrates
                .view(|c| { CreateCalorieTargetMessage::UpdateCarbohydrates(c).into() }),
        ]
        .spacing(10);

        let content = column![
            Text::new("Create calorie target").size(40),
            form,
            Button::new("Create").on_press(CreateCalorieTargetMessage::Submit.into())
        ]
        .spacing(10);

        row![sidebar(), content]
            .height(Length::Fill)
            .padding(20)
            .spacing(20)
            .into()
    }

    fn update(&mut self, ctx: &mut Context, msg: Message) -> Task<Message> {
        if let Message::CreateCalorieTarget(msg) = msg {
            match msg {
                CreateCalorieTargetMessage::UpdateDay(day) => {
                    self.day.value = day;
                }
                CreateCalorieTargetMessage::UpdateCalories(raw_calories) => {
                    self.calories.raw_input = raw_calories;
                }
                CreateCalorieTargetMessage::UpdateFats(raw_fats) => {
                    self.fats.raw_input = raw_fats;
                }
                CreateCalorieTargetMessage::UpdateProteins(raw_proteins) => {
                    self.proteins.raw_input = raw_proteins;
                }
                CreateCalorieTargetMessage::UpdateCarbohydrates(raw_carbohydrates) => {
                    self.carbohydrates.raw_input = raw_carbohydrates;
                }
                CreateCalorieTargetMessage::Submit => {
                    if let Ok(target) = self.parse() {
                        if let Some(err) = ctx.data.calorie_target.create(target).err() {
                            match err {
                                DataError::UniqueConstraintViolation(unique_field)
                                    if unique_field == "calorie_targets.day" =>
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
                            ctx.next_widget = Some(NextWidget::CalorieTargetList);
                        }
                    };
                }
            }
        };

        Task::none()
    }
}
