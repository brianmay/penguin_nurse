use std::sync::Arc;

use dioxus::prelude::*;

use crate::components::{ChangeWee, DeleteWee, WeeOperation};
use crate::dt::get_date_for_entry_dt;
use crate::functions::wees::get_wee_by_id;
use crate::models::{Wee, WeeId};
use crate::Route;

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveDialog {
    Idle,
    Change(Wee),
    Delete(Wee),
}

#[component]
pub fn WeeDialog(
    dialog: Signal<ActiveDialog>,
    reload: Callback<()>,
    delete: Callback<()>,
) -> Element {
    match dialog() {
        ActiveDialog::Idle => rsx! {},
        ActiveDialog::Change(wee) => {
            rsx! {
                ChangeWee {
                    op: WeeOperation::Update {
                        wee: Arc::new(wee),
                    },
                    on_cancel: move || dialog.set(ActiveDialog::Idle),
                    on_save: move |_wee| {
                        reload(());
                        dialog.set(ActiveDialog::Idle);
                    },
                }
            }
        }
        ActiveDialog::Delete(wee) => {
            rsx! {
                DeleteWee {
                    wee: Arc::new(wee),
                    on_cancel: move || dialog.set(ActiveDialog::Idle),
                    on_delete: move |_wee| {
                        delete(());
                        dialog.set(ActiveDialog::Idle);
                    },
                }
            }
        }
    }
}

#[component]
pub fn WeeDetail(wee_id: WeeId) -> Element {
    let mut wee = use_resource(move || async move { get_wee_by_id(wee_id).await });
    let mut dialog: Signal<ActiveDialog> = use_signal(|| ActiveDialog::Idle);
    let navigator = use_navigator();

    match wee() {
        Some(Ok(Some(obj))) => {
            let wee_clone_1 = obj.clone();
            let wee_clone_2 = obj.clone();

            rsx! {
                div { class: "p-4",
                    h1 { "Wee Details" }
                    p {
                        "ID: "
                        {obj.id.as_inner().to_string()}
                    }
                    p {
                        "Created: "
                        {obj.created_at.to_string()}
                    }
                    p {
                        "Updated: "
                        {obj.updated_at.to_string()}
                    }
                }

                div { class: "p-4",
                    button {
                        class: "btn btn-secondary me-2 mb-2",
                        onclick: move |_| dialog.set(ActiveDialog::Change(wee_clone_1.clone())),
                        "Change"
                    }
                    button {
                        class: "btn btn-error me-2 mb-2",
                        onclick: move |_| dialog.set(ActiveDialog::Delete(wee_clone_2.clone())),
                        "Delete"
                    }
                }
                div { class: "p-4",
                    button {
                        class: "btn btn-secondary me-2 mb-2",
                        onclick: move |_| {
                            let date = get_date_for_entry_dt(obj.created_at);
                            if let Ok(date) = date {
                                navigator.push(Route::TimelineList { date });
                            }
                        },
                        "Timeline"
                    }
                }

                WeeDialog {
                    dialog,
                    reload: move || (wee.restart()),
                    delete: move || {
                        let date = get_date_for_entry_dt(obj.created_at);
                        if let Ok(date) = date {
                            navigator.push(Route::TimelineList { date });
                        }
                    },
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
