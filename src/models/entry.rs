use std::sync::Arc;

use chrono::{DateTime, Utc};

use super::{Poo, Wee};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Event {
    Start,
    // End,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntryData {
    Poo(Arc<Poo>),
    Wee(Arc<Wee>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub event: Event,
    pub time: DateTime<Utc>,
    pub data: EntryData,
}

impl Entry {
    pub fn get_id(&self) -> String {
        match &self.data {
            EntryData::Poo(poo) => format!("poo-{}", poo.id),
            EntryData::Wee(wee) => format!("wee-{}", wee.id),
        }
    }
}
