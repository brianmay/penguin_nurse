use serde::{Deserialize, Serialize};

use super::{consumables::ConsumableId, Consumable, ConsumptionId, MaybeF64, MaybeString};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConsumptionConsumableId(ConsumptionId, ConsumableId);

impl ConsumptionConsumableId {
    pub fn new(parent_id: ConsumptionId, consumable_id: ConsumableId) -> Self {
        Self(parent_id, consumable_id)
    }
    pub fn as_inner(self) -> (ConsumptionId, ConsumableId) {
        (self.0, self.1)
    }
}

// impl FromStr for ConsumptionConsumableId {
//     type Err = std::num::ParseIntError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(Self(s.parse()?))
//     }
// }

// impl std::fmt::Display for ConsumptionConsumableId {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.0.fmt(f)
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConsumptionConsumable {
    pub id: ConsumptionConsumableId,
    pub quantity: MaybeF64,
    pub liquid_mls: MaybeF64,
    pub comments: MaybeString,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConsumptionItem {
    pub nested: ConsumptionConsumable,
    pub consumable: Consumable,
}

impl ConsumptionItem {
    pub fn new(nested: ConsumptionConsumable, consumable: Consumable) -> Self {
        Self { nested, consumable }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NewConsumptionConsumable {
    pub id: ConsumptionConsumableId,
    pub quantity: MaybeF64,
    pub liquid_mls: MaybeF64,
    pub comments: MaybeString,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UpdateConsumptionConsumable {
    pub quantity: Option<MaybeF64>,
    pub liquid_mls: Option<MaybeF64>,
    pub comments: Option<MaybeString>,
}
