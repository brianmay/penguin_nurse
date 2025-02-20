use dioxus::prelude::*;

use crate::{
    components::{
        buttons::{ChangeButton, DeleteButton},
        poos::{self, ActiveDialog, PooDialog},
    },
    functions::poos::get_poo_by_id,
    models::PooId,
};

#[component]
pub fn PooDetail(poo_id: PooId) -> Element {
    let mut maybe_poo = use_resource(move || async move { get_poo_by_id(poo_id).await });

    let mut active_dialog: Signal<ActiveDialog> = use_signal(|| ActiveDialog::Idle);

    match maybe_poo() {
        Some(Ok(Some(poo))) => {
            let poo_clone_2 = poo.clone();
            let poo_clone_3 = poo.clone();

            rsx! {
                poos::PooDetail { poo, on_close: move || navigator().go_back() }
                PooDialog {
                    dialog: active_dialog(),
                    on_change: move |_consumption| {
                        active_dialog.set(ActiveDialog::Idle);
                        maybe_poo.restart();
                    },
                    on_delete: move |_consumption| {
                        active_dialog.set(ActiveDialog::Idle);
                        maybe_poo.restart();
                    },
                    on_close: move || active_dialog.set(ActiveDialog::Idle),
                }
                ChangeButton {
                    on_click: move |_| {
                        active_dialog
                            .set(
                                poos::ActiveDialog::Change(poos::Operation::Update {
                                    poo: poo_clone_2.clone(),
                                }),
                            )
                    },
                    "Edit"
                }
                DeleteButton { on_click: move |_| { active_dialog.set(poos::ActiveDialog::Delete(poo_clone_3.clone())) },
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
