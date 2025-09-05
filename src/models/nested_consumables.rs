use serde::{Deserialize, Serialize};

use crate::models::MaybeSet;

use super::{Consumable, ConsumableId};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NestedConsumableId(ConsumableId, ConsumableId);

impl NestedConsumableId {
    pub fn new(parent_id: ConsumableId, consumable_id: ConsumableId) -> Self {
        Self(parent_id, consumable_id)
    }
    #[cfg(feature = "server")]
    pub fn as_inner(self) -> (ConsumableId, ConsumableId) {
        (self.0, self.1)
    }
}

// impl FromStr for NestedConsumableId {
//     type Err = std::num::ParseIntError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(Self(s.parse()?))
//     }
// }

// impl std::fmt::Display for NestedConsumableId {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.0.fmt(f)
//     }
// }

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NestedConsumable {
    pub id: NestedConsumableId,
    pub quantity: Option<f64>,
    pub liquid_mls: Option<f64>,
    pub comments: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConsumableItem {
    pub nested: NestedConsumable,
    pub consumable: Consumable,
}

impl ConsumableItem {
    pub fn new(nested: NestedConsumable, consumable: Consumable) -> Self {
        Self { nested, consumable }
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NewNestedConsumable {
    pub id: NestedConsumableId,
    pub quantity: Option<f64>,
    pub liquid_mls: Option<f64>,
    pub comments: Option<String>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChangeNestedConsumable {
    pub quantity: MaybeSet<Option<f64>>,
    pub liquid_mls: MaybeSet<Option<f64>>,
    pub comments: MaybeSet<Option<String>>,
}
