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

#[component]
pub fn ConsumptionDetail(
    consumption_id: ReadOnlySignal<ConsumptionId>,
    dialog: ReadOnlySignal<Option<DialogReference>>,
) -> Element {
    let mut maybe_consumption =
        use_resource(move || async move { get_consumption_by_id(consumption_id()).await });

    let mut maybe_items =
        use_resource(move || async move { get_child_consumables(consumption_id()).await });

    let active_dialog: Memo<ActiveDialog> = use_memo(move || {
        let Some(dialog) = dialog() else {
            return ActiveDialog::Idle;
        };
        let Some(Ok(Some(consumption))) = maybe_consumption() else {
            return ActiveDialog::Idle;
        };
        match dialog {
            DialogReference::Update => ActiveDialog::Change(Operation::Update { consumption }),
            DialogReference::Ingredients => ActiveDialog::Ingredients(consumption),
            DialogReference::Delete => ActiveDialog::Delete(consumption),
            DialogReference::Idle => ActiveDialog::Idle,
        }
    });

    let navigator = navigator();
    match (maybe_consumption(), maybe_items()) {
        (Some(Ok(Some(consumption))), Some(Ok(items))) => {
            let consumption_id = consumption.id;
            rsx! {
                consumptions::ConsumptionDetail { consumption, list: items }
                ConsumptionDialog {
                    dialog: active_dialog(),
                    on_change: move |_consumption: Consumption| {
                        maybe_consumption.restart();
                        maybe_items.restart();
                    },
                    on_change_ingredients: move |_consumption| {
                        maybe_consumption.restart();
                        maybe_items.restart();
                    },
                    on_delete: move |_consumption: Consumption| {
                        maybe_consumption.restart();
                        maybe_items.restart();
                    },
                    on_close: move || {
                        navigator
                            .push(Route::ConsumptionDetail {
                                consumption_id,
                                dialog: consumptions::DialogReference::Idle
                            });
                    },
                    show_edit: move |consumption: Consumption| {
                        navigator
                            .push(Route::ConsumptionDetail {
                                consumption_id: consumption.id,
                                dialog: consumptions::DialogReference::Update
                            });
                    },
                    show_ingredients: move |consumption: Consumption| {
                        navigator
                            .push(Route::ConsumptionDetail {
                                consumption_id: consumption.id,
                                dialog: consumptions::DialogReference::Ingredients
                            });
                    },
                    show_nested_ingredient: move |(_consumption, consumable): (Consumption, Consumable)| {
                        navigator
                            .push(Route::ConsumableDetail {
                                consumable_id: consumable.id,
                                dialog: consumables::DetailsDialogReference::Update { }
                            });
                    },
                    show_nested_ingredients: move |(_consumption, consumable): (Consumption, Consumable)| {
                        navigator
                            .push(Route::ConsumableDetail {
                                consumable_id: consumable.id,
                                dialog: consumables::DetailsDialogReference::Update { }
                            });
                    },
                }
                ChangeButton {
                    on_click: move |_| {
                        navigator
                            .push(Route::ConsumptionDetail {
                                consumption_id,
                                dialog: consumptions::DialogReference::Ingredients
                            });
                    },
                    "Ingredients"
                }
                ChangeButton {
                    on_click: move |_| {
                        navigator
                            .push(Route::ConsumptionDetail {
                                consumption_id,
                                dialog: consumptions::DialogReference::Update
                            });
                    },
                    "Edit"
                }
                DeleteButton {
                    on_click: move |_| {
                        navigator
                            .push(Route::ConsumptionDetail {
                                consumption_id,
                                dialog: consumptions::DialogReference::Delete
                            });
                    },
                    "Delete"
                }
            }
        }
        (Some(Ok(None)), _) => {
            rsx! {
                div { class: "alert alert-error", "Consumption not found." }
            }
        }
        (Some(Err(err)), _) | (_, Some(Err(err))) => {
            rsx! {
                div { class: "alert alert-error",
                    "Error: "
                    {err.to_string()}
                }
            }
        }
        (None, _) | (_, None) => {
            rsx! {
                div { class: "alert alert-info", "Loading..." }
            }
        }
    }
}
