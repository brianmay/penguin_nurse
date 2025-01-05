use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Serializing Option<Option<String>> does not work as expected. This is a workaround.
#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum Maybe<T> {
    Some(T),
    None,
}

impl<T> From<Option<T>> for Maybe<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => Maybe::Some(value),
            None => Maybe::None,
        }
    }
}

impl<T> From<Maybe<T>> for Option<T> {
    fn from(value: Maybe<T>) -> Self {
        match value {
            Maybe::Some(value) => Some(value),
            Maybe::None => None,
        }
    }
}

impl Maybe<String> {
    pub fn as_deref(&self) -> Option<&str> {
        match self {
            Maybe::Some(value) => Some(value),
            Maybe::None => None,
        }
    }

    pub fn option(self) -> Option<String> {
        self.into()
    }
}

pub type MaybeString = Maybe<String>;
pub type MaybeDateTime = Maybe<DateTime<Utc>>;
pub type MaybeF64 = Maybe<f64>;
