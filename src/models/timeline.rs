use super::entry::{Entry, EntryData, Event};
use super::poos::Poo;
use super::wees::Wee;
use super::ConsumptionWithItems;

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

    pub fn sort(&mut self) {
        self.0.sort_by(|a, b| a.time.cmp(&b.time));
    }

    pub fn iter(&self) -> std::slice::Iter<Entry> {
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
