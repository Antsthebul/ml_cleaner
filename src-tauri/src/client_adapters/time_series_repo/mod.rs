use std::{fmt::Debug, str::FromStr};

use chrono::{DateTime, Utc};

use crate::client_adapters::utils::{parse_value_from_regex, ParseError};
use futures::prelude::*;
use influxdb2::{models::Query, Client, FromDataPoint};
use influxdb2_derive::WriteDataPoint;
use regex::Regex;
use serde::Serialize;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct TSDBClientError(String);

#[derive(Default, Serialize, Clone, WriteDataPoint)]
#[measurement = "training_data"]
pub struct TrainingData {
    #[influxdb(field)]
    epoch: i64,
    #[influxdb(timestamp)]
    time: i64,
    #[influxdb(field)]
    train_acc: String,
    #[influxdb(field)]
    test_acc: String,
    #[influxdb(field)]
    train_loss: f64,
    #[influxdb(field)]
    val_loss: f64,
    #[influxdb(tag)]
    dir_name: String,
    #[influxdb(tag)]
    metadata: Option<String>,
    #[influxdb(field)]
    duration: f64,
}
impl TrainingData {
    pub fn parse(text: &str) -> Result<Self, ParseError> {
        let pattern = format!(r#"(["a-z\d\._\'%]+)\s*,?"#);
        let epoch_re = Regex::new(&format!(r"epoch={}", pattern)).unwrap();
        let train_acc_re = Regex::new(&format!(r"train_acc={}", pattern)).unwrap();
        let test_acc_re = Regex::new(&format!(r"test_acc={}", pattern)).unwrap();
        let time_re = Regex::new(&format!("time={}", r#"([\d\.]+)sec"#)).unwrap();
        let train_loss_re = Regex::new(&format!(r"train_loss={}", pattern)).unwrap();
        let val_loss_re = Regex::new(&format!(r"val_loss={}", pattern)).unwrap();
        let dir_name_re = Regex::new(&format!(r"dir_name={}", pattern)).unwrap();

        Ok(TrainingData {
            time: Utc::now().timestamp_nanos_opt().unwrap(),
            epoch: parse_value_from_regex(epoch_re, &text)?,
            train_acc: parse_value_from_regex(train_acc_re, &text)?,
            test_acc: parse_value_from_regex(test_acc_re, &text)?,
            duration: parse_value_from_regex(time_re, &text)?,
            train_loss: parse_value_from_regex(train_loss_re, &text)?,
            val_loss: parse_value_from_regex(val_loss_re, &text)?,
            dir_name: parse_value_from_regex(dir_name_re, &text)?,
            metadata: None,
        })
    }
}

pub async fn insert_record(training_data: &TrainingData) -> Result<(), TSDBClientError> {
    let client = Client::new(
        "http://host.docker.internal:8086",
        "org",
        "aLT23G4KUIkAznnGtPQkxlkO5z7OREwI0ECUrZg7cpXqo6xi_XUqMW6qROGWPg_5JpmbXc7XKwvhoiKHhSVHxw==",
    );
    client
        .write("bucket", stream::iter(vec![training_data.clone()]))
        .await
        .map_err(|err| TSDBClientError(err.to_string()))?;
    Ok(())
}

pub async fn insert_record_from_str(text: &str) -> Result<(), TSDBClientError> {
    let data = TrainingData::parse(text)
        .map_err(|err| TSDBClientError(format!("failed to insert record. {:?}", err)))?;

    Ok(insert_record(&data).await?)
}
