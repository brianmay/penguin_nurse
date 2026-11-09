use serde::{Deserialize, Serialize};

use crate::models::ConsumableId;
use crate::models::common::MaybeSet;

use super::{Consumable, ScheduleId};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScheduleConsumableId(ScheduleId, ConsumableId);

impl ScheduleConsumableId {
    pub fn new(parent_id: ScheduleId, consumable_id: ConsumableId) -> Self {
        Self(parent_id, consumable_id)
    }
    #[cfg(feature = "server")]
    pub fn as_inner(self) -> (ScheduleId, ConsumableId) {
        (self.0, self.1)
    }
}

#[cfg(feature = "server")]
impl ScheduleConsumableId {
    pub fn parent_id(&self) -> ScheduleId {
        self.0
    }
    pub fn child_id(&self) -> ConsumableId {
        self.1
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ScheduleConsumable {
    pub id: ScheduleConsumableId,
    pub quantity: Option<bigdecimal::BigDecimal>,
    pub liquid_mls: Option<bigdecimal::BigDecimal>,
    pub comments: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ScheduleItem {
    pub nested: ScheduleConsumable,
    pub consumable: Consumable,
}

impl ScheduleItem {
    pub fn new(nested: ScheduleConsumable, consumable: Consumable) -> Self {
        Self { nested, consumable }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NewScheduleConsumable {
    pub id: ScheduleConsumableId,
    pub quantity: Option<bigdecimal::BigDecimal>,
    pub liquid_mls: Option<bigdecimal::BigDecimal>,
    pub comments: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChangeScheduleConsumable {
    pub quantity: MaybeSet<Option<bigdecimal::BigDecimal>>,
    pub liquid_mls: MaybeSet<Option<bigdecimal::BigDecimal>>,
    pub comments: MaybeSet<Option<String>>,
}
