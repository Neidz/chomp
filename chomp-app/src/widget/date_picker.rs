use std::fmt::Display;

use chrono::{Datelike, Days, Local, Month, NaiveDate};
use iced::{
    widget::{button, column, container, horizontal_space, pick_list, row, Button, Row, Text},
    Alignment, Color, Element, Length,
};

use crate::{app::Message, widget::form_field::InputFormFieldError};

const CALENDAR_ITEM_WIDTH: f32 = 40.0;
const AMOUNT_OF_ITEMS_PER_ROW: u32 = 7;
const AMOUNT_OF_ROWS: u32 = 6;
const AMOUNT_OF_ITEMS_IN_CALENDAR: u32 = AMOUNT_OF_ROWS * AMOUNT_OF_ITEMS_PER_ROW;
const SPACING: f32 = 2.0;
const SPACING_IN_ROW: f32 = (AMOUNT_OF_ITEMS_PER_ROW as f32 - 1f32) * SPACING;
const MODAL_PADDING: f32 = 30.0;

#[derive(Debug, Clone)]
struct CalendarState {
    year: Option<i32>,
    month: Option<CalendarMonth>,
}

impl CalendarState {
    fn new(date: NaiveDate) -> Self {
        CalendarState {
            year: Some(date.year()),
            month: Some(CalendarMonth::from(date)),
        }
    }

    fn displayed_month_first_day(&self) -> NaiveDate {
        NaiveDate::from_ymd_opt(
            self.year.unwrap(),
            self.month.clone().unwrap().0.number_from_month(),
            1,
        )
        .unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct DatePicker {
    pub name: String,
    pub value: NaiveDate,
    pub error: Option<InputFormFieldError>,
    calendar_state: Option<CalendarState>,
}

impl DatePicker {
    pub fn new(name: &str) -> Self {
        DatePicker {
            name: name.to_string(),
            value: Local::now().date_naive(),
            error: None,
            calendar_state: None,
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
            Button::new(Text::new(formatted_day)).on_press(Message::OpenDatePicker),
            horizontal_space(),
            Button::new(">").on_press(day_change_message(
                self.value.checked_add_days(Days::new(1)).unwrap()
            )),
        ]
        .align_y(Alignment::Center)
        .width(200)
        .spacing(SPACING);

        let mut column = column![Text::new(&self.name), day_row].spacing(2);

        if let Some(err) = &self.error {
            column = column.push(Text::new(err.to_string()).color(Color::from_rgb(1.0, 0.0, 0.0)));
        }

        column.into()
    }

    pub fn view_modal<F>(&self, day_change_message: F) -> Element<Message>
    where
        F: Fn(NaiveDate) -> Message + 'static,
    {
        let calendar_state = match self.calendar_state.as_ref() {
            Some(cs) => cs,
            None => unreachable!(),
        };

        let month_select = pick_list(
            CalendarMonth::all_months(),
            calendar_state.month.clone(),
            |month| Message::DatePickerMonthChange(month),
        );

        let current_year = Local::now().year();
        let year_options: Vec<i32> = (current_year - 20..current_year + 20).collect();

        let year_select = pick_list(year_options, calendar_state.year, |year| {
            Message::DatePickerYearChange(year)
        });

        let month_and_year_row = row![month_select, horizontal_space(), year_select]
            .width(Length::Fill)
            .spacing(SPACING);

        let mut calendar = column![month_and_year_row].spacing(SPACING);

        calendar = calendar.push(row![["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"]
            .into_iter()
            .map(|label| Text::new(label).width(CALENDAR_ITEM_WIDTH).into())
            .collect::<Row<Message>>()
            .spacing(SPACING)]);

        let calendar_displayed_month_first_day = calendar_state.displayed_month_first_day();

        let days_from_prev_month = calendar_displayed_month_first_day
            .weekday()
            .num_days_from_monday();
        let first_day_in_calendar = calendar_displayed_month_first_day
            .checked_sub_days(Days::new(days_from_prev_month as u64))
            .unwrap();

        let days_in_calendar: Vec<NaiveDate> = (0..AMOUNT_OF_ITEMS_IN_CALENDAR)
            .into_iter()
            .map(|amount_of_days| {
                first_day_in_calendar
                    .checked_add_days(Days::new(amount_of_days as u64))
                    .unwrap()
            })
            .collect();

        for days_in_row in days_in_calendar.chunks(7) {
            let mut row = row![].spacing(2);

            for day in days_in_row {
                let currently_selected_day = *day == self.value;
                let currently_displayed_month =
                    day.month0() == calendar_displayed_month_first_day.month0();

                let style = if currently_selected_day {
                    button::success
                } else if currently_displayed_month {
                    button::primary
                } else {
                    button::secondary
                };

                let button = Button::new(Text::new(day.day0() + 1).center())
                    .width(CALENDAR_ITEM_WIDTH)
                    .style(style)
                    .on_press(day_change_message(day.to_owned()));

                row = row.push(button);
            }

            calendar = calendar.push(row);
        }

        container(calendar)
            .width((CALENDAR_ITEM_WIDTH * 7.0) + (2.0 * MODAL_PADDING) + (SPACING_IN_ROW))
            .padding(MODAL_PADDING)
            .style(container::rounded_box)
            .into()
    }

    pub fn handle_message(&mut self, message: Message) {
        match message {
            Message::DatePickerDateChange(date) => {
                self.value = date;
                self.calendar_state = None;
            }
            Message::DatePickerYearChange(year) => {
                if let Some(calendar) = &mut self.calendar_state {
                    calendar.year = Some(year);
                }
            }
            Message::DatePickerMonthChange(month) => {
                if let Some(calendar) = &mut self.calendar_state {
                    calendar.month = Some(month);
                }
            }
            Message::OpenDatePicker => self.calendar_state = Some(CalendarState::new(self.value)),
            Message::CloseDatePicker => self.calendar_state = None,
            _ => {}
        }
    }

    pub fn calendar_open(&self) -> bool {
        self.calendar_state.is_some()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CalendarMonth(Month);

impl CalendarMonth {
    fn all_months() -> Vec<CalendarMonth> {
        vec![
            Month::January,
            Month::February,
            Month::March,
            Month::April,
            Month::May,
            Month::June,
            Month::July,
            Month::August,
            Month::September,
            Month::October,
            Month::November,
            Month::December,
        ]
        .into_iter()
        .map(CalendarMonth)
        .collect()
    }
}

impl Display for CalendarMonth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self.0 {
            Month::January => "January",
            Month::February => "Febuary",
            Month::March => "March",
            Month::April => "April",
            Month::May => "May",
            Month::June => "June",
            Month::July => "July",
            Month::August => "August",
            Month::September => "September",
            Month::October => "October",
            Month::November => "November",
            Month::December => "December",
        };

        write!(f, "{str}")
    }
}

impl From<NaiveDate> for CalendarMonth {
    fn from(value: NaiveDate) -> Self {
        let month = match value.month0() {
            0 => Month::January,
            1 => Month::February,
            2 => Month::March,
            3 => Month::April,
            4 => Month::May,
            5 => Month::June,
            6 => Month::July,
            7 => Month::August,
            8 => Month::September,
            9 => Month::October,
            10 => Month::November,
            11 => Month::December,
            _ => unreachable!(),
        };

        CalendarMonth(month)
    }
}
