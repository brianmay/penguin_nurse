use chrono::{DateTime, Utc};

use super::{Consumption, ConsumptionId, Poo, PooId, Wee, WeeId};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EntryId {
    Poo(PooId),
    Wee(WeeId),
    Consumption(ConsumptionId),
}

impl EntryId {
    pub fn as_str(&self) -> String {
        match self {
            EntryId::Poo(id) => format!("poo-{}", id.as_inner()),
            EntryId::Wee(id) => format!("wee-{}", id.as_inner()),
            EntryId::Consumption(id) => format!("consumption-{}", id.as_inner()),
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
    Poo(Poo),
    Wee(Wee),
    Consumption(Consumption),
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
            EntryData::Poo(poo) => EntryId::Poo(poo.id),
            EntryData::Wee(wee) => EntryId::Wee(wee.id),
            EntryData::Consumption(consumption) => EntryId::Consumption(consumption.id),
        }
    }
}
