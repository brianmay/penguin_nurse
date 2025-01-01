use std::sync::Arc;

use dioxus::prelude::*;

use crate::components::{ChangePoo, DeletePoo, PooOperation};
use crate::dt::get_date_for_entry_dt;
use crate::functions::poos::get_poo_by_id;
use crate::models::{Poo, PooId};
use crate::Route;

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveDialog {
    Idle,
    Change(Poo),
    Delete(Poo),
}

#[component]
pub fn PooDialog(
    dialog: Signal<ActiveDialog>,
    reload: Callback<()>,
    delete: Callback<()>,
) -> Element {
    match dialog() {
        ActiveDialog::Idle => rsx! {},
        ActiveDialog::Change(poo) => {
            rsx! {
                ChangePoo {
                    op: PooOperation::Update {
                        poo: Arc::new(poo),
                    },
                    on_cancel: move || dialog.set(ActiveDialog::Idle),
                    on_save: move |_poo| {
                        reload(());
                        dialog.set(ActiveDialog::Idle);
                    },
                }
            }
        }
        ActiveDialog::Delete(poo) => {
            rsx! {
                DeletePoo {
                    poo: Arc::new(poo),
                    on_cancel: move || dialog.set(ActiveDialog::Idle),
                    on_delete: move |_poo| {
                        delete(());
                        dialog.set(ActiveDialog::Idle);
                    },
                }
            }
        }
    }
}

#[component]
pub fn PooDetail(poo_id: PooId) -> Element {
    let mut poo = use_resource(move || async move { get_poo_by_id(poo_id).await });
    let mut dialog: Signal<ActiveDialog> = use_signal(|| ActiveDialog::Idle);
    let navigator = use_navigator();

    match poo() {
        Some(Ok(Some(obj))) => {
            let poo_clone_1 = obj.clone();
            let poo_clone_2 = obj.clone();

            rsx! {
                div { class: "p-4",
                    h1 { "Poo Details" }
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
                        onclick: move |_| dialog.set(ActiveDialog::Change(poo_clone_1.clone())),
                        "Change"
                    }
                    button {
                        class: "btn btn-error me-2 mb-2",
                        onclick: move |_| dialog.set(ActiveDialog::Delete(poo_clone_2.clone())),
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

                PooDialog {
                    dialog,
                    reload: move || (poo.restart()),
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
