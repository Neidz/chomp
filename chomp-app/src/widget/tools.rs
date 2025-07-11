use chomp_services::{CreateUpdateProduct, ServiceError, Weight};
use std::{
    fs::File,
    path::{Path, PathBuf},
};

use chrono::NaiveDate;
use csv::Reader;
use iced::{
    widget::{column, row, Button, Text},
    Element,
    Length::{self},
    Task,
};
use rfd::AsyncFileDialog;
use serde::Deserialize;

use crate::app::{Context, Message};

use super::{sidebar::sidebar, Widget};

#[derive(Debug, Deserialize)]
struct FitnotesRecord {
    #[serde(rename = "Date")]
    date: NaiveDate,
    #[serde(rename = "Measurement")]
    measurement: String,
    #[serde(rename = "Value")]
    value: f32,
}

#[derive(Debug, Clone)]
pub enum ToolsMessage {
    PickFitnotesWeightsDataFile,
    LoadFitnotesWeightsData(Option<PathBuf>),
    PickProductsJSONDataFile,
    LoadProductsJSONData(Option<PathBuf>),
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
                .on_press(ToolsMessage::PickFitnotesWeightsDataFile.into())
        ]
        .spacing(2);

        let json_products = column![
            Text::new("Products"),
            Button::new("Load Products From JSON File")
                .on_press(ToolsMessage::PickProductsJSONDataFile.into())
        ]
        .spacing(2);

        let content = column![Text::new("Tools").size(40), fitnotes, json_products].spacing(10);

        row![sidebar(), content]
            .height(Length::Fill)
            .padding(20)
            .spacing(20)
            .into()
    }

    fn update(&mut self, ctx: &mut Context, msg: Message) -> Task<Message> {
        if let Message::Tools(msg) = msg {
            match msg {
                ToolsMessage::PickFitnotesWeightsDataFile => {
                    return Task::perform(pick_fitnotes_weights_data_file(), |file_path| {
                        ToolsMessage::LoadFitnotesWeightsData(file_path).into()
                    });
                }
                ToolsMessage::LoadFitnotesWeightsData(file_path) => {
                    if let Some(path) = file_path {
                        import_fitnotes_weights_data(&path, ctx);
                    }
                }
                ToolsMessage::PickProductsJSONDataFile => {
                    return Task::perform(pick_products_data_file(), |file_path| {
                        ToolsMessage::LoadProductsJSONData(file_path).into()
                    });
                }
                ToolsMessage::LoadProductsJSONData(file_path) => {
                    if let Some(path) = file_path {
                        import_products_data(&path, ctx);
                    }
                }
            }
        };

        Task::none()
    }
}

async fn pick_products_data_file() -> Option<PathBuf> {
    AsyncFileDialog::new()
        .set_title("Select file with products in JSON format...")
        .add_filter("JSON files", &[".json"])
        .pick_file()
        .await
        .map(|handle| handle.path().to_path_buf())
}

fn import_products_data(path: &Path, ctx: &mut Context) {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(err) => {
            tracing::error!("Failed to open products JSON file: {}", err);
            return;
        }
    };

    let products: Result<Vec<CreateUpdateProduct>, _> = serde_json::from_reader(file);

    let products = match products {
        Ok(p) => p,
        Err(err) => {
            tracing::error!("Failed to parse JSON file: {}", err);
            return;
        }
    };

    let mut created = 0;
    let mut skipped = 0;
    let mut failed_to_create = 0;

    for product in products {
        match ctx.services.product.create(product) {
            Ok(_) => {
                created += 1;
            }
            Err(err) => match err {
                ServiceError::UniqueConstraintViolation(unique_field)
                    if unique_field == "products.name" =>
                {
                    skipped += 1;
                }
                _ => {
                    failed_to_create += 1;
                    tracing::error!("Failed to create weight: {}", err);
                }
            },
        }
    }

    tracing::info!(
        "Products: created {}, skipped (exists) {}, failed {}",
        created,
        skipped,
        failed_to_create,
    );
}

async fn pick_fitnotes_weights_data_file() -> Option<PathBuf> {
    AsyncFileDialog::new()
        .set_title("Select fitnotes weights data...")
        .add_filter("CSV files", &["csv"])
        .pick_file()
        .await
        .map(|handle| handle.path().to_path_buf())
}

fn import_fitnotes_weights_data(path: &Path, ctx: &mut Context) {
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

        match ctx.services.weight.create(weight.clone()) {
            Ok(_) => {
                added_records += 1;
            }
            Err(err) => match err {
                ServiceError::UniqueConstraintViolation(unique_field)
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
