use chrono::{DateTime, FixedOffset};

use crate::{
    components::timeline::DialogReference,
    models::{
        Exercise, ExerciseId, HealthMetric, HealthMetricId, Note, NoteId, Reflux, RefluxId,
        Symptom, SymptomId, WeeUrge, WeeUrgeId,
    },
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
    pub time: DateTime<FixedOffset>,
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

    pub fn get_update_dialog_reference(&self) -> DialogReference {
        match &self.data {
            EntryData::Poo(poo) => DialogReference::UpdatePoo { poo_id: poo.id },
            EntryData::Wee(wee) => DialogReference::UpdateWee { wee_id: wee.id },
            EntryData::WeeUrge(wee_urge) => DialogReference::UpdateWeeUrge {
                wee_urge_id: wee_urge.id,
            },
            EntryData::Consumption(consumption_with_items) => DialogReference::UpdateBasic {
                consumption_id: consumption_with_items.consumption.id,
            },
            EntryData::Exercise(exercise) => DialogReference::UpdateExercise {
                exercise_id: exercise.id,
            },
            EntryData::HealthMetric(health_metric) => DialogReference::UpdateHealthMetric {
                health_metric_id: health_metric.id,
            },
            EntryData::Symptom(symptom) => DialogReference::UpdateSymptom {
                symptom_id: symptom.id,
            },
            EntryData::Reflux(reflux) => DialogReference::UpdateReflux {
                reflux_id: reflux.id,
            },
            EntryData::Note(note) => DialogReference::UpdateNote { note_id: note.id },
        }
    }

    pub fn get_delete_dialog_reference(&self) -> DialogReference {
        match &self.data {
            EntryData::Poo(poo) => DialogReference::DeletePoo { poo_id: poo.id },
            EntryData::Wee(wee) => DialogReference::DeleteWee { wee_id: wee.id },
            EntryData::WeeUrge(wee_urge) => DialogReference::DeleteWeeUrge {
                wee_urge_id: wee_urge.id,
            },
            EntryData::Consumption(consumption_with_items) => DialogReference::DeleteConsumption {
                consumption_id: consumption_with_items.consumption.id,
            },
            EntryData::Exercise(exercise) => DialogReference::DeleteExercise {
                exercise_id: exercise.id,
            },
            EntryData::HealthMetric(health_metric) => DialogReference::DeleteHealthMetric {
                health_metric_id: health_metric.id,
            },
            EntryData::Symptom(symptom) => DialogReference::DeleteSymptom {
                symptom_id: symptom.id,
            },
            EntryData::Reflux(reflux) => DialogReference::DeleteReflux {
                reflux_id: reflux.id,
            },
            EntryData::Note(note) => DialogReference::DeleteNote { note_id: note.id },
        }
    }
}
