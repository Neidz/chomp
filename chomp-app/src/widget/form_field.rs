use std::fmt::{self};

use iced::{
    widget::{column, Text, TextInput},
    Color, Element,
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
                write!(f, "Must be at least {min_length} characters long")
            }
            InputFormFieldError::Custom(s) => {
                write!(f, "{s}")
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
