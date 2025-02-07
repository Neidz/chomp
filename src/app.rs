use iced::{
    widget::{column, row, Button, Text},
    Element,
};

#[derive(Debug, Clone)]
pub enum Message {
    Hello,
}

pub struct App {}

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub fn view(&self) -> Element<Message> {
        let calories_content = column![
            Text::new(format!("Current calories",)),
            row![Button::new("Add").on_press(Message::Hello)].spacing(20),
        ]
        .padding(20)
        .spacing(20);

        row![self.sidebar(), calories_content].into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Hello => {
                println!("Hello");
            }
        }
    }

    fn sidebar(&self) -> Element<Message> {
        column![Button::new("Home"), Button::new("Product List")].into()
    }
}
