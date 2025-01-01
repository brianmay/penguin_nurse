use std::sync::Arc;

use chrono::Local;
use dioxus::prelude::*;

use crate::components::{ChangePoo, DeletePoo, PooOperation};
use crate::dt::get_date_for_entry_dt;
use crate::functions::poos::get_poo_by_id;
use crate::models::{Bristol, Poo, PooId};
use crate::views::event::{event_colour, event_urgency};
use crate::Route;

const POO_SVG: Asset = asset!("/assets/poo.svg");

#[component]
pub fn poo_icon() -> Element {
    rsx! {
        img { class: "w-10 invert inline-block", alt: "Poo", src: POO_SVG }
    }
}

#[component]
pub fn poo_duration(duration: chrono::TimeDelta) -> Element {
    rsx! {
        if duration.num_seconds() == 0 {
            span { class: "text-error", {duration.num_seconds().to_string() + " seconds"} }
        } else if duration.num_seconds() < 60 {
            span { class: "text-success", {duration.num_seconds().to_string() + " seconds"} }
        } else if duration.num_minutes() < 60 {
            span { class: "text-warning", {duration.num_minutes().to_string() + " minutes"} }
        } else if duration.num_hours() < 24 {
            span { class: "text-error", {duration.num_hours().to_string() + " hours"} }
        } else {
            span { class: "text-error", {duration.num_days().to_string() + " days"} }
        }
    }
}

#[component]
pub fn poo_bristol(bristol: Bristol) -> Element {
    let bristol_string = bristol.as_str();

    match bristol {
        Bristol::B0 => rsx! {
            span { class: "text-error", {bristol_string} }
        },
        Bristol::B1 => rsx! {
            span { class: "text-error", {bristol_string} }
        },
        Bristol::B2 => rsx! {
            span { class: "text-success", {bristol_string} }
        },
        Bristol::B3 => rsx! {
            span { class: "text-success", {bristol_string} }
        },
        Bristol::B4 => rsx! {
            span { class: "text-success", {bristol_string} }
        },
        Bristol::B5 => rsx! {
            span { class: "text-warning", {bristol_string} }
        },

        Bristol::B6 => rsx! {
            span { class: "text-warning", {bristol_string} }
        },
        Bristol::B7 => rsx! {
            span { class: "text-error", {bristol_string} }
        },
    }
}

#[component]
pub fn poo_quantity(quantity: i32) -> Element {
    rsx! {
        if quantity == 0 {
            span { class: "text-error", {quantity.to_string() + " out of 5"} }
        } else if quantity < 2 {
            span { class: "text-warning", {quantity.to_string() + " out of 5"} }
        } else {
            span { class: "text-success", {quantity.to_string() + " out of 5"} }
        }
    }
}

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
    let navigator = navigator();

    match poo() {
        Some(Ok(Some(obj))) => {
            let poo_clone_1 = obj.clone();
            let poo_clone_2 = obj.clone();

            rsx! {
                div { class: "p-4",
                    table { class: "table table-striped",
                        tbody {
                            tr {
                                td { "Event" }
                                td { poo_icon {} }
                            }
                            tr {
                                td { "ID" }
                                td { {obj.id.as_inner().to_string()} }
                            }
                            tr {
                                td { "Created" }
                                td { {obj.created_at.with_timezone(&Local).to_string()} }
                            }
                            tr {
                                td { "Updated" }
                                td { {obj.updated_at.with_timezone(&Local).to_string()} }
                            }
                            tr {
                                td { "Colour" }
                                td {
                                    event_colour { colour: obj.colour }
                                }
                            }
                            tr {
                                td { "Urgency" }
                                td {
                                    event_urgency { urgency: obj.urgency }
                                }
                            }
                            tr {
                                td { "Duration" }
                                td {
                                    poo_duration { duration: obj.duration }
                                }
                            }
                            tr {
                                td { "Quantity" }
                                td {
                                    poo_quantity { quantity: obj.quantity }
                                }
                            }
                            tr {
                                td { "Bristol" }
                                td {
                                    poo_bristol { bristol: obj.bristol }
                                }
                            }
                        
                        }
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
