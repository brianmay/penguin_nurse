use std::num::TryFromIntError;

use dioxus_fullstack::ServerFnError;
use thiserror::Error;

pub mod consumables;
pub mod consumption_consumables;
pub mod consumptions;
pub mod exercises;
pub mod health_metrics;
pub mod nested_consumables;
pub mod notes;
pub mod poos;
pub mod refluxs;
pub mod session;
pub mod symptoms;
pub mod users;
pub mod wee_urges;
pub mod wees;

#[derive(Error, Debug)]
pub enum DatabaseConversionError {
    #[error("database conversion error")]
    InvalidValue,
}

impl From<TryFromIntError> for DatabaseConversionError {
    fn from(_: TryFromIntError) -> Self {
        Self::InvalidValue
    }
}

impl From<DatabaseConversionError> for ServerFnError {
    fn from(err: DatabaseConversionError) -> Self {
        ServerFnError::new(err.to_string())
    }
}
