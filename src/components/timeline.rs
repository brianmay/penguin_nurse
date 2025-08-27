use std::{num::ParseIntError, str::FromStr};

use dioxus::{prelude::*, router::ToQueryArgument};
use tap::Pipe;
use thiserror::Error;

use crate::{
    components::{consumptions::ConsumptionDialog, poos::PooDialog, wees::WeeDialog},
    models::{
        Consumable, ConsumableId, Consumption, ConsumptionId, Poo, PooId, UserId, Wee, WeeId,
    },
};

use super::{consumptions, poos, wees};

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveDialog {
    Wee(wees::ActiveDialog),
    Poo(poos::ActiveDialog),
    Consumption(consumptions::ActiveDialog),
    Idle,
}

#[derive(Error, Debug)]
pub enum DialogReferenceError {
    #[error("Invalid integer")]
    ParseIntError(#[from] ParseIntError),

    #[error("Invalid reference2")]
    ReferenceError,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum DialogReference {
    CreateWee {
        user_id: UserId,
    },
    UpdateWee {
        wee_id: WeeId,
    },
    DeleteWee {
        wee_id: WeeId,
    },
    CreatePoo {
        user_id: UserId,
    },
    UpdatePoo {
        poo_id: PooId,
    },
    DeletePoo {
        poo_id: PooId,
    },
    CreateConsumption {
        user_id: UserId,
    },
    UpdateBasic {
        consumption_id: ConsumptionId,
    },
    UpdateIngredients {
        consumption_id: ConsumptionId,
    },
    IngredientUpdateBasic {
        parent_id: ConsumptionId,
        consumable_id: ConsumableId,
    },
    IngredientUpdateIngredients {
        parent_id: ConsumptionId,
        consumable_id: ConsumableId,
    },
    DeleteConsumption {
        consumption_id: ConsumptionId,
    },
    #[default]
    Idle,
}

impl ToQueryArgument for DialogReference {
    fn display_query_argument(
        &self,
        query_name: &str,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}={}", query_name, self.to_string())
    }
}

impl FromStr for DialogReference {
    type Err = DialogReferenceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split("-").collect::<Vec<_>>();
        match split[..] {
            ["wee", "create", id] => {
                let user_id = UserId::new(id.parse()?);
                Self::CreateWee { user_id }
            }
            ["wee", "update", id] => {
                let wee_id = WeeId::new(id.parse()?);
                Self::UpdateWee { wee_id }
            }
            ["wee", "delete", id] => {
                let wee_id = WeeId::new(id.parse()?);
                Self::DeleteWee { wee_id }
            }
            ["poo", "create", id] => {
                let user_id = UserId::new(id.parse()?);
                Self::CreatePoo { user_id }
            }
            ["poo", "update", poo_id] => {
                let poo_id = PooId::new(poo_id.parse()?);
                Self::UpdatePoo { poo_id }
            }
            ["poo", "delete", id] => {
                let poo_id = PooId::new(id.parse()?);
                Self::DeletePoo { poo_id }
            }
            ["consumption", "create", id] => {
                let user_id = UserId::new(id.parse()?);
                Self::CreateConsumption { user_id }
            }
            ["consumption", "update", id] => {
                let consumption_id = ConsumptionId::new(id.parse()?);
                Self::UpdateBasic { consumption_id }
            }
            ["consumption_ingredients", "update", id] => {
                let consumption_id = ConsumptionId::new(id.parse()?);
                Self::UpdateIngredients { consumption_id }
            }
            [
                "consumption_ingredients",
                "nested_ingredient",
                parent_id,
                id,
            ] => {
                let parent_id = ConsumptionId::new(parent_id.parse()?);
                let consumable_id = ConsumableId::new(id.parse()?);
                Self::IngredientUpdateBasic {
                    parent_id,
                    consumable_id,
                }
            }
            [
                "consumption_ingredients",
                "nested_ingredients",
                parent_id,
                id,
            ] => {
                let parent_id = ConsumptionId::new(parent_id.parse()?);
                let consumable_id = ConsumableId::new(id.parse()?);
                Self::IngredientUpdateIngredients {
                    parent_id,
                    consumable_id,
                }
            }
            ["consumption", "delete", id] => {
                let consumption_id = ConsumptionId::new(id.parse()?);
                Self::DeleteConsumption { consumption_id }
            }
            [""] | [] => Self::Idle,
            _ => return Err(DialogReferenceError::ReferenceError),
        }
        .pipe(Ok)
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for DialogReference {
    fn to_string(&self) -> String {
        match self {
            DialogReference::CreateWee { user_id } => format!("wee-create-{user_id}"),
            DialogReference::UpdateWee { wee_id } => format!("wee-update-{wee_id}"),
            DialogReference::DeleteWee { wee_id } => format!("wee-delete-{wee_id}"),
            DialogReference::CreatePoo { user_id } => format!("poo-create-{user_id}"),
            DialogReference::UpdatePoo { poo_id } => format!("poo-update-{poo_id}"),
            DialogReference::DeletePoo { poo_id } => format!("poo-delete-{poo_id}"),
            DialogReference::CreateConsumption { user_id } => {
                format!("consumption-create-{user_id}")
            }
            DialogReference::UpdateBasic { consumption_id } => {
                format!("consumption-update-{consumption_id}")
            }
            DialogReference::UpdateIngredients { consumption_id } => {
                format!("consumption_ingredients-update-{consumption_id}")
            }
            DialogReference::IngredientUpdateBasic {
                parent_id,
                consumable_id,
            } => {
                format!("consumption_ingredients-nested_ingredient-{parent_id}-{consumable_id}")
            }
            DialogReference::IngredientUpdateIngredients {
                parent_id,
                consumable_id,
            } => {
                format!("consumption_ingredients-nested_ingredients-{parent_id}-{consumable_id}")
            }
            DialogReference::DeleteConsumption { consumption_id } => {
                format!("consumption-delete-{consumption_id}")
            }
            DialogReference::Idle => String::new(),
        }
    }
}

#[component]
pub fn TimelineDialog(
    dialog: ReadOnlySignal<ActiveDialog>,
    on_change: Callback<()>,
    on_close: Callback<()>,
    replace_dialog: Callback<DialogReference>,
    show_consumption_update_basic: Callback<Consumption>,
    show_consumption_update_ingredients: Callback<Consumption>,
    show_consumption_ingredient_update_basic: Callback<(Consumption, Consumable)>,
    show_consumption_ingredient_update_ingredients: Callback<(Consumption, Consumable)>,
) -> Element {
    match dialog() {
        ActiveDialog::Wee(wee_dialog) => {
            rsx! {
                WeeDialog {
                    dialog: wee_dialog,
                    on_close,
                    on_change: move |wee: Wee| {
                        replace_dialog(DialogReference::UpdateWee { wee_id: wee.id });
                        on_change(());
                        on_close(());
                    },
                    on_delete: move |_wee| {
                        on_change(());
                        on_close(());
                    },
                }
            }
        }
        ActiveDialog::Poo(poo_dialog) => {
            rsx! {
                PooDialog {
                    dialog: poo_dialog,
                    on_close,
                    on_change: move |poo: Poo| {
                        replace_dialog(DialogReference::UpdatePoo{ poo_id: poo.id });
                        on_change(());
                        on_close(());
                   },
                    on_delete: move |_poo| {
                        on_change(());
                        on_close(());
                    },
                }
            }
        }
        ActiveDialog::Consumption(consumption_dialog) => {
            rsx! {
                ConsumptionDialog {
                    dialog: consumption_dialog,
                    show_update_basic: show_consumption_update_basic,
                    show_update_ingredients: show_consumption_update_ingredients,
                    show_ingredient_update_basic: show_consumption_ingredient_update_basic,
                    show_ingredient_update_ingredients: show_consumption_ingredient_update_ingredients,
                    on_change: move |consumption: Consumption| {
                        replace_dialog(DialogReference::UpdateBasic{ consumption_id: consumption.id });
                        on_change(());
                    },
                    on_change_ingredients: move |_consumption| {
                        on_change(());
                    },
                    on_delete: move |_consumption| {
                        on_change(());
                    },
                    on_close,
                }
            }
        }
        ActiveDialog::Idle => {
            rsx! {}
        }
    }
}
