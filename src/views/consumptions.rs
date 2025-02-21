use dioxus::prelude::*;

use crate::{
    components::{
        buttons::{ChangeButton, DeleteButton},
        consumptions::{self, ActiveDialog, ConsumptionDialog},
    },
    functions::consumptions::{get_child_consumables, get_consumption_by_id},
    models::ConsumptionId,
};

#[component]
pub fn ConsumptionDetail(consumption_id: ReadOnlySignal<ConsumptionId>) -> Element {
    let mut maybe_consumption =
        use_resource(move || async move { get_consumption_by_id(consumption_id()).await });

    let mut maybe_items =
        use_resource(move || async move { get_child_consumables(consumption_id()).await });

    let mut active_dialog: Signal<ActiveDialog> = use_signal(|| ActiveDialog::Idle);

    match (maybe_consumption(), maybe_items()) {
        (Some(Ok(Some(consumption))), Some(Ok(items))) => {
            let consumption_clone_2 = consumption.clone();
            let consumption_clone_3 = consumption.clone();
            let consumption_clone_4 = consumption.clone();

            rsx! {
                consumptions::ConsumptionDetail { consumption, list: items }
                ConsumptionDialog {
                    dialog: active_dialog(),
                    on_change: move |_consumption| {
                        active_dialog.set(ActiveDialog::Idle);
                        maybe_consumption.restart();
                        maybe_items.restart();
                    },
                    on_delete: move |_consumption| {
                        active_dialog.set(ActiveDialog::Idle);
                        maybe_consumption.restart();
                        maybe_items.restart();
                    },
                    select_dialog: move |dialog| active_dialog.set(dialog),
                }
                ChangeButton {
                    on_click: move |_| {
                        active_dialog.set(ActiveDialog::Consumption(consumption_clone_2.clone()))
                    },
                    "Ingredients"
                }
                ChangeButton {
                    on_click: move |_| {
                        active_dialog
                            .set(
                                consumptions::ActiveDialog::Change(consumptions::Operation::Update {
                                    consumption: consumption_clone_3.clone(),
                                }),
                            )
                    },
                    "Edit"
                }
                DeleteButton {
                    on_click: move |_| {
                        active_dialog
                            .set(consumptions::ActiveDialog::Delete(consumption_clone_4.clone()))
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
