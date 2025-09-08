use serde::{Deserialize, Serialize};

use crate::models::ConsumableId;
use crate::models::common::MaybeSet;

use super::{Consumable, ConsumptionId};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConsumptionConsumableId(ConsumptionId, ConsumableId);

impl ConsumptionConsumableId {
    pub fn new(parent_id: ConsumptionId, consumable_id: ConsumableId) -> Self {
        Self(parent_id, consumable_id)
    }
    #[cfg(feature = "server")]
    pub fn as_inner(self) -> (ConsumptionId, ConsumableId) {
        (self.0, self.1)
    }
}

#[cfg(feature = "server")]
impl ConsumptionConsumableId {
    pub fn parent_id(&self) -> ConsumptionId {
        self.0
    }
    pub fn child_id(&self) -> ConsumableId {
        self.1
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConsumptionConsumable {
    pub id: ConsumptionConsumableId,
    pub quantity: Option<f64>,
    pub liquid_mls: Option<f64>,
    pub comments: Option<String>,
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
    pub quantity: Option<f64>,
    pub liquid_mls: Option<f64>,
    pub comments: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChangeConsumptionConsumable {
    pub quantity: MaybeSet<Option<f64>>,
    pub liquid_mls: MaybeSet<Option<f64>>,
    pub comments: MaybeSet<Option<String>>,
}
