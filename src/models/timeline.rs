use crate::models::{Exercise, HealthMetric, Symptom, WeeUrge};

use super::ConsumptionWithItems;
use super::entry::{Entry, EntryData, Event};
use super::poos::Poo;
use super::wees::Wee;

pub struct Timeline(Vec<Entry>);

impl Timeline {
    pub fn new() -> Self {
        Timeline(vec![])
    }

    pub fn add_wees(&mut self, wees: Vec<Wee>) {
        for wee in wees {
            self.add_wee(wee);
        }
    }

    pub fn add_wee(&mut self, wee: Wee) {
        self.0.push(Entry {
            event: Event::Start,
            time: wee.time,
            data: EntryData::Wee(wee),
        });
    }

    pub fn add_wee_urges(&mut self, wee_urges: Vec<WeeUrge>) {
        for wee_urge in wee_urges {
            self.add_wee_urge(wee_urge);
        }
    }

    pub fn add_wee_urge(&mut self, wee_urge: WeeUrge) {
        self.0.push(Entry {
            event: Event::Start,
            time: wee_urge.time,
            data: EntryData::WeeUrge(wee_urge),
        });
    }

    pub fn add_poos(&mut self, poos: Vec<Poo>) {
        for poo in poos {
            self.add_poo(poo);
        }
    }

    pub fn add_poo(&mut self, poo: Poo) {
        self.0.push(Entry {
            event: Event::Start,
            time: poo.time,
            data: EntryData::Poo(poo),
        });
    }

    pub fn add_consumptions(&mut self, consumptions: Vec<ConsumptionWithItems>) {
        for consumption in consumptions {
            self.add_consumption(consumption);
        }
    }

    pub fn add_consumption(&mut self, consumption: ConsumptionWithItems) {
        self.0.push(Entry {
            event: Event::Start,
            time: consumption.consumption.time,
            data: EntryData::Consumption(consumption),
        });
    }

    pub fn add_exercises(&mut self, exercises: Vec<Exercise>) {
        for exercise in exercises {
            self.add_exercise(exercise);
        }
    }

    pub fn add_exercise(&mut self, exercise: Exercise) {
        self.0.push(Entry {
            event: Event::Start,
            time: exercise.time,
            data: EntryData::Exercise(exercise),
        });
    }

    pub fn add_health_metrics(&mut self, health_metrics: Vec<HealthMetric>) {
        for health_metric in health_metrics {
            self.add_health_metric(health_metric);
        }
    }

    pub fn add_health_metric(&mut self, health_metric: HealthMetric) {
        self.0.push(Entry {
            event: Event::Start,
            time: health_metric.time,
            data: EntryData::HealthMetric(health_metric),
        });
    }

    pub fn add_symptoms(&mut self, symptoms: Vec<Symptom>) {
        for symptom in symptoms {
            self.add_symptom(symptom);
        }
    }

    pub fn add_symptom(&mut self, symptom: Symptom) {
        self.0.push(Entry {
            event: Event::Start,
            time: symptom.time,
            data: EntryData::Symptom(symptom),
        });
    }

    pub fn add_refluxs(&mut self, refluxs: Vec<crate::models::Reflux>) {
        for reflux in refluxs {
            self.add_reflux(reflux);
        }
    }

    pub fn add_reflux(&mut self, reflux: crate::models::Reflux) {
        self.0.push(Entry {
            event: Event::Start,
            time: reflux.time,
            data: EntryData::Reflux(reflux),
        });
    }

    pub fn add_notes(&mut self, notes: Vec<crate::models::Note>) {
        for note in notes {
            self.add_note(note);
        }
    }

    pub fn add_note(&mut self, note: crate::models::Note) {
        self.0.push(Entry {
            event: Event::Start,
            time: note.time,
            data: EntryData::Note(note),
        });
    }

    pub fn sort(&mut self) {
        self.0.sort_by(|a, b| a.time.cmp(&b.time));
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Entry> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

// impl IntoIterator for Timeline {
//     type Item = Entry;
//     type IntoIter = std::vec::IntoIter<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }
