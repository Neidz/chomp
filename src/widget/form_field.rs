use std::fmt::{self};

use chrono::{Days, Local, NaiveDate};
use iced::{
    widget::{column, horizontal_space, row, Button, Text, TextInput},
    Alignment, Color, Element,
};

use crate::app::Message;

type Length = usize;

#[derive(Debug, Clone)]
pub enum InputFormFieldError {
    MissingRequiredValue,
    InvalidNumber,
    SmallerThanZero,
    TooShort(Length),
    Custom(String),
}

impl fmt::Display for InputFormFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputFormFieldError::MissingRequiredValue => {
                write!(f, "Field required")
            }
            InputFormFieldError::InvalidNumber => {
                write!(f, "Invalid number")
            }
            InputFormFieldError::SmallerThanZero => {
                write!(f, "Invalid number. Must be at least zero")
            }
            InputFormFieldError::TooShort(min_length) => {
                write!(f, "Must be at least {} characters long", min_length)
            }
            InputFormFieldError::Custom(s) => {
                write!(f, "{}", s)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct InputFormField<T> {
    pub name: String,
    pub placeholder: String,
    pub value: Option<T>,
    pub raw_input: String,
    pub error: Option<InputFormFieldError>,
}

impl<T> InputFormField<T> {
    pub fn new(name: &str, placeholder: &str) -> Self {
        InputFormField {
            name: name.to_string(),
            placeholder: placeholder.to_owned(),
            value: None,
            raw_input: String::new(),
            error: None,
        }
    }

    pub fn new_with_raw_value(name: &str, placeholder: &str, raw_value: &str) -> Self {
        InputFormField {
            name: name.to_string(),
            placeholder: placeholder.to_string(),
            value: None,
            raw_input: raw_value.to_owned(),
            error: None,
        }
    }

    pub fn validate<F>(&mut self, validator: F)
    where
        F: Fn(&str) -> Result<T, InputFormFieldError>,
    {
        self.error = None;
        match validator(&self.raw_input) {
            Ok(valid_value) => self.value = Some(valid_value),
            Err(err) => {
                self.value = None;
                self.error = Some(err);
            }
        }
    }

    pub fn view<F>(&self, handle_message: F) -> Element<Message>
    where
        F: Fn(String) -> Message + 'static,
    {
        let mut column = column![
            Text::new(&self.name),
            TextInput::new(&self.placeholder, &self.raw_input).on_input(handle_message)
        ]
        .spacing(2);

        if let Some(err) = &self.error {
            column = column.push(Text::new(err.to_string()).color(Color::from_rgb(1.0, 0.0, 0.0)));
        }

        column.into()
    }
}

#[derive(Debug, Clone)]
pub struct DayFormField {
    pub name: String,
    pub value: NaiveDate,
    pub error: Option<InputFormFieldError>,
}

impl DayFormField {
    pub fn new(name: &str) -> Self {
        DayFormField {
            name: name.to_string(),
            value: Local::now().date_naive(),
            error: None,
        }
    }

    pub fn view<F>(&self, day_change_message: F) -> Element<Message>
    where
        F: Fn(NaiveDate) -> Message + 'static,
    {
        let today = Local::now().date_naive();
        let tomorrow = today.checked_add_days(Days::new(1)).unwrap();
        let yesterday = today.checked_sub_days(Days::new(1)).unwrap();

        let formatted_day = match self.value {
            d if d == today => "Today".to_string(),
            d if d == tomorrow => "Tomorrow".to_string(),
            d if d == yesterday => "Yesterday".to_string(),
            _ => self.value.format("%Y-%m-%d").to_string(),
        };

        let day_row = row![
            Button::new("<").on_press(day_change_message(
                self.value.checked_sub_days(Days::new(1)).unwrap()
            )),
            horizontal_space(),
            Text::new(formatted_day).size(20),
            horizontal_space(),
            Button::new(">").on_press(day_change_message(
                self.value.checked_add_days(Days::new(1)).unwrap()
            )),
        ]
        .align_y(Alignment::Center)
        .width(220)
        .spacing(10);

        let mut column = column![Text::new(&self.name), day_row].spacing(2);

        if let Some(err) = &self.error {
            column = column.push(Text::new(err.to_string()).color(Color::from_rgb(1.0, 0.0, 0.0)));
        }

        column.into()
    }
}
