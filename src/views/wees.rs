use dioxus::prelude::*;

use crate::{
    components::{
        buttons::{ChangeButton, DeleteButton},
        wees::{self, ActiveDialog, WeeDialog},
    },
    functions::wees::get_wee_by_id,
    models::WeeId,
};

#[component]
pub fn WeeDetail(wee_id: WeeId) -> Element {
    let mut maybe_wee = use_resource(move || async move { get_wee_by_id(wee_id).await });

    let mut active_dialog: Signal<ActiveDialog> = use_signal(|| ActiveDialog::Idle);

    match maybe_wee() {
        Some(Ok(Some(wee))) => {
            let wee_clone_2 = wee.clone();
            let wee_clone_3 = wee.clone();

            rsx! {
                wees::WeeDetail { wee, on_close: move || navigator().go_back() }
                WeeDialog {
                    dialog: active_dialog(),
                    on_change: move |_consumption| {
                        active_dialog.set(ActiveDialog::Idle);
                        maybe_wee.restart();
                    },
                    on_delete: move |_consumption| {
                        active_dialog.set(ActiveDialog::Idle);
                        maybe_wee.restart();
                    },
                    on_close: move || active_dialog.set(ActiveDialog::Idle),
                }
                ChangeButton {
                    on_click: move |_| {
                        active_dialog
                            .set(
                                wees::ActiveDialog::Change(wees::Operation::Update {
                                    wee: wee_clone_2.clone(),
                                }),
                            )
                    },
                    "Edit"
                }
                DeleteButton { on_click: move |_| { active_dialog.set(wees::ActiveDialog::Delete(wee_clone_3.clone())) },
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
