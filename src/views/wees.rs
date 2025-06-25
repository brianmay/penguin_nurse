use dioxus::prelude::*;

use crate::{
    Route,
    components::{
        buttons::{ChangeButton, DeleteButton},
        wees::{self, ActiveDialog, DialogReference, Operation, WeeDialog},
    },
    functions::wees::get_wee_by_id,
    models::WeeId,
};

#[component]
pub fn WeeDetail(wee_id: WeeId, dialog: ReadOnlySignal<Option<DialogReference>>) -> Element {
    let mut maybe_wee = use_resource(move || async move { get_wee_by_id(wee_id).await });

    let active_dialog: Memo<ActiveDialog> = use_memo(move || {
        let Some(dialog) = dialog() else {
            return ActiveDialog::Idle;
        };
        let Some(Ok(Some(wee))) = maybe_wee() else {
            return ActiveDialog::Idle;
        };
        match dialog {
            DialogReference::Update => ActiveDialog::Change(Operation::Update { wee }),
            DialogReference::Delete => ActiveDialog::Delete(wee),
            DialogReference::Idle => ActiveDialog::Idle,
        }
    });

    let navigator = navigator();

    match maybe_wee() {
        Some(Ok(Some(wee))) => {
            rsx! {
                wees::WeeDetail { wee }
                WeeDialog {
                    dialog: active_dialog(),
                    on_change: move |_wee| {
                        navigator.push(Route::WeeDetail {
                            wee_id,
                            dialog: DialogReference::Idle
                        });
                        maybe_wee.restart();
                    },
                    on_delete: move |_wee| {
                        navigator.push(Route::WeeDetail {
                            wee_id,
                            dialog: DialogReference::Idle
                        });
                        maybe_wee.restart();
                    },
                    on_close: move || {
                       navigator.push(Route::WeeDetail {
                            wee_id,
                            dialog: DialogReference::Idle
                        });
                    },
                }
                ChangeButton {
                    on_click: move |_| {
                        navigator
                            .push(Route::WeeDetail {
                                wee_id,
                                dialog: DialogReference::Update
                            });
                    },
                    "Edit"
                }
                DeleteButton {
                    on_click: move |_| {
                        navigator
                            .push(Route::WeeDetail {
                                wee_id,
                                dialog: DialogReference::Delete
                            });

                    },
                    "Delete"
                }
            }
        }
        Some(Ok(None)) => {
            rsx! {
                div { class: "alert alert-error", "Wee not found." }
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
