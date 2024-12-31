use std::sync::Arc;

use chrono::{DateTime, Utc};

use super::{Poo, PooId, Wee, WeeId};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EntryId {
    PooId(PooId),
    WeeId(WeeId),
}

impl EntryId {
    pub fn as_str(&self) -> String {
        match self {
            EntryId::PooId(id) => format!("poo-{}", id.as_inner()),
            EntryId::WeeId(id) => format!("wee-{}", id.as_inner()),
        }
    }
}

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
    pub fn get_id(&self) -> EntryId {
        match &self.data {
            EntryData::Poo(poo) => EntryId::PooId(poo.id),
            EntryData::Wee(wee) => EntryId::WeeId(wee.id),
        }
    }
}
