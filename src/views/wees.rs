use std::sync::Arc;

use chrono::Local;
use dioxus::prelude::*;

use crate::components::{ChangeWee, DeleteWee, WeeOperation};
use crate::dt::get_date_for_entry_dt;
use crate::functions::wees::get_wee_by_id;
use crate::models::{Wee, WeeId};
use crate::views::event::{event_colour, event_urgency};
use crate::Route;

const WEE_SVG: Asset = asset!("/assets/wee.svg");

#[component]
pub fn wee_icon() -> Element {
    rsx! {
        img { class: "w-10 invert inline-block", alt: "Wee", src: WEE_SVG }
    }
}

#[component]
pub fn wee_duration(duration: chrono::TimeDelta) -> Element {
    rsx! {
        if duration.num_seconds() == 0 {
            span { class: "text-error", {duration.num_seconds().to_string() + " seconds"} }
        } else if duration.num_seconds() < 120 {
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
pub fn wee_mls(mls: i32) -> Element {
    rsx! {
        if mls == 0 {
            span { class: "text-error", {mls.to_string() + " ml"} }
        } else if mls < 100 {
            span { class: "text-warning", {mls.to_string() + " ml"} }
        } else if mls < 500 {
            span { class: "text-success", {mls.to_string() + " ml"} }
        } else {
            span { class: "text-error", {mls.to_string() + " ml"} }
        }
    }
}

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
    let navigator = navigator();

    match wee() {
        Some(Ok(Some(obj))) => {
            let wee_clone_1 = obj.clone();
            let wee_clone_2 = obj.clone();

            rsx! {
                div { class: "p-4",
                    table { class: "table table-striped",
                        tbody {
                            tr {
                                td { "Event" }
                                td { wee_icon {} }
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
                                    wee_duration { duration: obj.duration }
                                }
                            }
                            tr {
                                td { "Mls" }
                                td {
                                    wee_mls { mls: obj.mls }
                                }
                            }
                        }
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
