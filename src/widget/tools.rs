use std::{fs::File, path::Path};

use chrono::NaiveDate;
use csv::Reader;
use iced::{
    widget::{column, row, Button, Text},
    Element,
    Length::{self},
};
use rfd::FileDialog;
use serde::Deserialize;

use crate::{
    app::{Context, Message},
    data::{DataError, Weight},
};

use super::{sidebar::sidebar, Widget};

#[derive(Debug, Deserialize)]
struct FitnotesRecord {
    #[serde(rename = "Date")]
    date: NaiveDate,
    #[serde(rename = "Measurement")]
    measurement: String,
    #[serde(rename = "Value")]
    value: f64,
}

#[derive(Debug, Clone)]
pub enum ToolsMessage {
    LoadFitnotesWeightsData,
}

impl From<ToolsMessage> for Message {
    fn from(value: ToolsMessage) -> Self {
        Message::Tools(value)
    }
}

#[derive(Debug)]
pub struct Tools {}

impl Tools {
    pub fn new() -> Self {
        Tools {}
    }
}

impl Widget for Tools {
    fn view(&self) -> Element<Message> {
        let fitnotes = column![
            Text::new("Fitnotes"),
            Button::new("Load Bodyweights From CSV File")
                .on_press(ToolsMessage::LoadFitnotesWeightsData.into())
        ]
        .spacing(2);

        let content = column![Text::new("Tools").size(40), fitnotes].spacing(10);

        row![sidebar(), content]
            .height(Length::Fill)
            .padding(20)
            .spacing(20)
            .into()
    }

    fn update(&mut self, ctx: &mut Context, msg: Message) {
        if let Message::Tools(msg) = msg {
            match msg {
                ToolsMessage::LoadFitnotesWeightsData => {
                    let file_path = FileDialog::new()
                        .add_filter("CSV files", &["csv"])
                        .pick_file();

                    if let Some(path) = file_path {
                        import_fitnotes_weights(&path, ctx);
                    }
                }
            }
        }
    }
}

fn import_fitnotes_weights(path: &Path, ctx: &mut Context) {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(err) => {
            tracing::error!("Failed to open Fitnotes CSV file: {}", err);
            return;
        }
    };

    let mut rdr = Reader::from_reader(file);

    let mut found_records = 0;
    let mut added_records = 0;
    let mut skipped_adding_because_of_existing = 0;
    let mut failed_to_add_because_of_unexpected_error = 0;
    let mut ignored_records = 0;
    let mut malformed_records = 0;

    for result in rdr.deserialize::<FitnotesRecord>() {
        let record = match result {
            Ok(r) => r,
            Err(err) => {
                malformed_records += 1;
                tracing::error!("Skipping malformed record: {}", err);
                continue;
            }
        };

        if record.measurement != "Bodyweight" {
            ignored_records += 1;
            continue;
        }

        found_records += 1;
        let weight = Weight::new(record.date, record.value);

        match ctx.data.weight.create(weight.clone()) {
            Ok(_) => {
                added_records += 1;
            }
            Err(err) => match err {
                DataError::UniqueConstraintViolation(unique_field)
                    if unique_field == "weights.day" =>
                {
                    skipped_adding_because_of_existing += 1;
                }
                _ => {
                    failed_to_add_because_of_unexpected_error += 1;
                    tracing::error!("Failed to create weight: {}", err);
                }
            },
        }
    }

    tracing::info!(
        "Processed {} records: added {}, skipped (exists) {}, failed {}, ignored {}, malformed {}",
        found_records,
        added_records,
        skipped_adding_because_of_existing,
        failed_to_add_because_of_unexpected_error,
        ignored_records,
        malformed_records
    );
}
