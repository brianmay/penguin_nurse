use jiff::{Timestamp, ZonedDateTime};

use crate::models::{
    Exercise, ExerciseId, HealthMetric, HealthMetricId, Note, NoteId, Reflux, RefluxId, Symptom,
    SymptomId, WeeUrge, WeeUrgeId,
};

use super::{ConsumptionId, ConsumptionWithItems, Poo, PooId, Wee, WeeId};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EntryId {
    Poo(PooId),
    Wee(WeeId),
    WeeUrge(WeeUrgeId),
    Consumption(ConsumptionId),
    Exercise(ExerciseId),
    HealthMetric(HealthMetricId),
    Symptom(SymptomId),
    Reflux(RefluxId),
    Note(NoteId),
}

impl EntryId {
    pub fn as_str(&self) -> String {
        match self {
            EntryId::Poo(id) => format!("poo-{}", id.as_inner()),
            EntryId::Wee(id) => format!("wee-{}", id.as_inner()),
            EntryId::WeeUrge(id) => format!("wee-urgency-{}", id.as_inner()),
            EntryId::Consumption(id) => format!("consumption-{}", id.as_inner()),
            EntryId::Exercise(id) => format!("exercise-{}", id.as_inner()),
            EntryId::HealthMetric(id) => format!("health-metric-{}", id.as_inner()),
            EntryId::Symptom(id) => format!("symptom-{}", id.as_inner()),
            EntryId::Reflux(id) => format!("reflux-{}", id.as_inner()),
            EntryId::Note(id) => format!("note-{}", id.as_inner()),
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
    WeeUrge(WeeUrge),
    Consumption(ConsumptionWithItems),
    Exercise(Exercise),
    HealthMetric(HealthMetric),
    Symptom(Symptom),
    Reflux(Reflux),
    Note(Note),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub event: Event,
    pub time: ZonedDateTime,
    pub data: EntryData,
}

impl Entry {
    pub fn get_id(&self) -> EntryId {
        match &self.data {
            EntryData::Poo(poo) => EntryId::Poo(poo.id),
            EntryData::Wee(wee) => EntryId::Wee(wee.id),
            EntryData::WeeUrge(wee_urge) => EntryId::WeeUrge(wee_urge.id),
            EntryData::Consumption(consumption) => EntryId::Consumption(consumption.consumption.id),
            EntryData::Exercise(exercise) => EntryId::Exercise(exercise.id),
            EntryData::HealthMetric(health_metric) => EntryId::HealthMetric(health_metric.id),
            EntryData::Symptom(symptom) => EntryId::Symptom(symptom.id),
            EntryData::Reflux(reflux) => EntryId::Reflux(reflux.id),
            EntryData::Note(note) => EntryId::Note(note.id),
        }
    }

    pub fn get_created_at(&self) -> Timestamp {
        match &self.data {
            EntryData::Poo(poo) => poo.created_at,
            EntryData::Wee(wee) => wee.created_at,
            EntryData::WeeUrge(wee_urge) => wee_urge.created_at,
            EntryData::Consumption(consumption) => consumption.consumption.created_at,
            EntryData::Exercise(exercise) => exercise.created_at,
            EntryData::HealthMetric(health_metric) => health_metric.created_at,
            EntryData::Symptom(symptom) => symptom.created_at,
            EntryData::Reflux(reflux) => reflux.created_at,
            EntryData::Note(note) => note.created_at,
        }
    }

    pub fn get_updated_at(&self) -> Timestamp {
        match &self.data {
            EntryData::Poo(poo) => poo.updated_at,
            EntryData::Wee(wee) => wee.updated_at,
            EntryData::WeeUrge(wee_urge) => wee_urge.updated_at,
            EntryData::Consumption(consumption) => consumption.consumption.updated_at,
            EntryData::Exercise(exercise) => exercise.updated_at,
            EntryData::HealthMetric(health_metric) => health_metric.updated_at,
            EntryData::Symptom(symptom) => symptom.updated_at,
            EntryData::Reflux(reflux) => reflux.updated_at,
            EntryData::Note(note) => note.updated_at,
        }
    }
}

impl EntryData {
    pub fn is_complete(&self) -> bool {
        match self {
            EntryData::Poo(poo) => poo.complete,
            EntryData::Wee(wee) => wee.complete,
            EntryData::WeeUrge(_) => true,
            EntryData::Consumption(consumption) => consumption.consumption.complete,
            EntryData::Exercise(exercise) => exercise.complete,
            EntryData::HealthMetric(_) => true,
            EntryData::Symptom(_) => true,
            EntryData::Reflux(reflux) => reflux.complete,
            EntryData::Note(_) => true,
        }
    }
}
