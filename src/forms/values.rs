use chrono::{DateTime, Local, TimeDelta, Utc};
use palette::RgbHue;
use thiserror::Error;

use crate::models::{MaybeDateTime, MaybeString};

#[derive(Error, Debug)]
pub enum FieldValueError {
    #[error("Invalid value")]
    InvalidValue,
}

pub trait FieldValue: Sized {
    fn as_string(&self) -> String;
    fn from_string(value: &str) -> Result<Self, FieldValueError>;
}

impl FieldValue for String {
    fn as_string(&self) -> String {
        self.clone()
    }

    fn from_string(value: &str) -> Result<Self, FieldValueError> {
        Ok(value.to_string())
    }
}

impl FieldValue for RgbHue<f32> {
    fn as_string(&self) -> String {
        self.into_inner().to_string()
    }

    fn from_string(value: &str) -> Result<Self, FieldValueError> {
        match value.parse() {
            Ok(value) if (0.0..=360.0).contains(&value) => Ok(RgbHue::new(value)),
            _ => Err(FieldValueError::InvalidValue),
        }
    }
}

impl FieldValue for MaybeString {
    fn as_string(&self) -> String {
        match self {
            MaybeString::Some(value) => value.clone(),
            MaybeString::None => "".to_string(),
        }
    }

    fn from_string(value: &str) -> Result<Self, FieldValueError> {
        if value.is_empty() {
            Ok(MaybeString::None)
        } else {
            Ok(MaybeString::Some(value.to_string()))
        }
    }
}

impl FieldValue for MaybeDateTime {
    fn as_string(&self) -> String {
        match self {
            MaybeDateTime::Some(value) => value.with_timezone(&Local).to_rfc3339(),
            MaybeDateTime::None => "".to_string(),
        }
    }

    fn from_string(value: &str) -> Result<Self, FieldValueError> {
        if value.is_empty() {
            Ok(MaybeDateTime::None)
        } else {
            match DateTime::parse_from_rfc3339(value) {
                Ok(time) => Ok(MaybeDateTime::Some(time.with_timezone(&Utc))),
                Err(_) => Err(FieldValueError::InvalidValue),
            }
        }
    }
}

impl FieldValue for DateTime<Utc> {
    fn as_string(&self) -> String {
        self.with_timezone(&Local).to_rfc3339()
    }

    fn from_string(value: &str) -> Result<Self, FieldValueError> {
        match DateTime::parse_from_rfc3339(value) {
            Ok(time) => Ok(time.with_timezone(&Utc)),
            Err(_) => Err(FieldValueError::InvalidValue),
        }
    }
}

impl FieldValue for TimeDelta {
    fn as_string(&self) -> String {
        self.num_seconds().to_string()
    }

    fn from_string(value: &str) -> Result<Self, FieldValueError> {
        match value.parse() {
            Ok(duration) if duration >= 0 => Ok(TimeDelta::seconds(duration)),
            _ => Err(FieldValueError::InvalidValue),
        }
    }
}

impl FieldValue for f32 {
    fn as_string(&self) -> String {
        self.to_string()
    }
    fn from_string(value: &str) -> Result<Self, FieldValueError> {
        match value.parse() {
            Ok(value) => Ok(value),
            Err(_) => Err(FieldValueError::InvalidValue),
        }
    }
}

impl FieldValue for f64 {
    fn as_string(&self) -> String {
        self.to_string()
    }
    fn from_string(value: &str) -> Result<Self, FieldValueError> {
        match value.parse() {
            Ok(value) => Ok(value),
            Err(_) => Err(FieldValueError::InvalidValue),
        }
    }
}

impl FieldValue for i32 {
    fn as_string(&self) -> String {
        self.to_string()
    }
    fn from_string(value: &str) -> Result<Self, FieldValueError> {
        match value.parse() {
            Ok(value) => Ok(value),
            Err(_) => Err(FieldValueError::InvalidValue),
        }
    }
}
