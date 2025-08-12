use dioxus::prelude::*;

use crate::{
    Route,
    components::{
        buttons::{ChangeButton, DeleteButton},
        consumables,
        consumptions::{self, ActiveDialog, ConsumptionDialog, DialogReference, Operation},
    },
    functions::consumptions::{get_child_consumables, get_consumption_by_id},
    models::{Consumable, Consumption, ConsumptionId},
};
