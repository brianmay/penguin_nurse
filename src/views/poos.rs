use dioxus::prelude::*;

use crate::{
    Route,
    components::{
        buttons::{ChangeButton, DeleteButton},
        poos::{self, ActiveDialog, DialogReference, Operation, PooDialog},
    },
    functions::poos::get_poo_by_id,
    models::{Poo, PooId},
};

#[component]
pub fn PooDetail(poo_id: PooId, dialog: ReadOnlySignal<Option<DialogReference>>) -> Element {
    let mut maybe_poo = use_resource(move || async move { get_poo_by_id(poo_id).await });

    let active_dialog: Memo<ActiveDialog> = use_memo(move || {
        let Some(dialog) = dialog() else {
            return ActiveDialog::Idle;
        };
        let Some(Ok(Some(poo))) = maybe_poo() else {
            return ActiveDialog::Idle;
        };
        match dialog {
            DialogReference::Update => ActiveDialog::Change(Operation::Update { poo }),
            DialogReference::Delete => ActiveDialog::Delete(poo),
            DialogReference::Idle => ActiveDialog::Idle,
        }
    });

    let navigator = navigator();
    match maybe_poo() {
        Some(Ok(Some(poo))) => {
            rsx! {
                poos::PooDetail { poo }
                PooDialog {
                    dialog: active_dialog(),
                    on_change: move |poo: Poo| {
                        navigator.push(Route::PooDetail {
                            poo_id: poo.id,
                            dialog: DialogReference::Idle
                        });
                        maybe_poo.restart();
                    },
                    on_delete: move |poo: Poo| {
                        navigator.push(Route::PooDetail {
                            poo_id: poo.id,
                            dialog: DialogReference::Idle
                        });
                        maybe_poo.restart();
                    },
                    on_close: move || {
                       navigator.push(Route::PooDetail {
                            poo_id,
                            dialog: DialogReference::Idle
                        });
                    },
                }
                ChangeButton {
                    on_click: move |_| {
                        navigator
                            .push(Route::PooDetail{
                                poo_id,
                                dialog: DialogReference::Update
                            });
                    },
                    "Edit"
                }
                DeleteButton {
                    on_click: move |_| {
                        navigator
                            .push(Route::PooDetail {
                                poo_id,
                                dialog: DialogReference::Delete
                            });

                    },
                    "Delete"
                }
            }
        }
        Some(Ok(None)) => {
            rsx! {
                div { class: "alert alert-error", "Poo not found." }
            }
        }
        Some(Err(err)) => {
            rsx! {
                div { class: "alert alert-error",
                    "Error: "
                    {err.to_string()}
                }
            }
        }
        None => {
            rsx! {
                div { class: "alert alert-info", "Loading..." }
            }
        }
    }
}
