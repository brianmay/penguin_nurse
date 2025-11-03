#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_fullstack::ServerFnError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
#[error("{0}")]
pub struct ValidationError(pub String);

// pub enum Saving {
//     No,
//     Yes,
//     Finished(Result<(), EditError>),
// }

#[derive(Error, Debug)]
pub enum EditError {
    #[error("{0}")]
    Server(ServerFnError),

    #[error(transparent)]
    Validation(#[from] ValidationError),
}
