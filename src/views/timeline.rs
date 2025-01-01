use std::{ops::Deref, sync::Arc};

use chrono::{Local, NaiveDate};
use dioxus::prelude::*;
use tap::Pipe;

use crate::{
    components::{ChangePoo, ChangeWee, PooOperation, WeeOperation},
    dt::get_utc_times_for_date,
    functions::{poos::get_poos_for_time_range, wees::get_wees_for_time_range},
    models::{Entry, EntryData, MaybeString, Timeline, User},
    views::{
        event::{event_colour, event_time, event_urgency},
        poos::{poo_bristol, poo_duration, poo_icon, poo_quantity},
        wees::{wee_duration, wee_icon, wee_mls},
    },
    Route,
};

#[component]
fn EntryRow(entry: Entry, on_click: Callback<Entry>) -> Element {
    rsx! {
        tr {
            class: "hover:bg-gray-500 border-blue-300 m-2 p-2 border-2 h-96 w-48 sm:w-auto sm:border-none sm:h-auto inline-block sm:table-row",
            onclick: move |_| on_click(entry.clone()),
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                event_time { time: entry.time }
            }
            match &entry.data {
                EntryData::Wee(wee) => {
                    rsx! {
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2", wee_icon {} }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            wee_duration { duration: wee.duration }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            event_colour { colour: wee.colour }
                            div { class: "inline-block ml-2 align-top",
                                div {
                                    wee_mls { mls: wee.mls }
                                }
                                div {
                                    event_urgency { urgency: wee.urgency }
                                }
                                if let MaybeString::Some(comments) = &wee.comments {
                                    div { {comments.to_string()} }
                                }
                            }
                        }
                    }
                }
                EntryData::Poo(poo) => {
                    rsx! {
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2", poo_icon {} }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            poo_duration { duration: poo.duration }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            event_colour { colour: poo.colour }
                            div { class: "inline-block ml-2 align-top",
                                div {
                                    poo_bristol { bristol: poo.bristol }
                                }
                                div {
                                    poo_quantity { quantity: poo.quantity }
                                }
                                div {
                                    event_urgency { urgency: poo.urgency }
                                }
                                if let MaybeString::Some(comments) = &poo.comments {
                                    div { {comments.to_string()} }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

enum ActiveDialog {
    ChangeWee(WeeOperation),
    ChangePoo(PooOperation),
    None,
}

#[component]
pub fn TimelineList(date: ReadOnlySignal<NaiveDate>) -> Element {
    let user: Signal<Arc<Option<User>>> = use_context();
    let navigator = navigator();

    let user: &Option<User> = &user.read();
    let Some(user) = user.as_ref() else {
        return rsx! {
            p { class: "alert alert-danger", "You are not logged in." }
        };
    };

    let user_id = user.pipe(|x| x.id);

    let mut active_dialog = use_signal(|| ActiveDialog::None);

    let mut timeline: Resource<Result<Timeline, ServerFnError>> =
        use_resource(move || async move {
            let (start, end) = get_utc_times_for_date(date())?;

            let mut timeline = Timeline::new();
            let wees = get_wees_for_time_range(user_id, start, end).await?;
            timeline.add_wees(wees);

            let poos = get_poos_for_time_range(user_id, start, end).await?;
            timeline.add_poos(poos);

            timeline.sort();

            Ok(timeline)
        });

    rsx! {
        div { class: "ml-2",
            div { class: "mb-2",
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        active_dialog.set(ActiveDialog::ChangeWee(WeeOperation::Create { user_id }))
                    },
                    "Wee"
                }
                button {
                    class: "btn btn-primary ml-2",
                    onclick: move |_| {
                        active_dialog.set(ActiveDialog::ChangePoo(PooOperation::Create { user_id }))
                    },
                    "Poo"
                }
            }

            div { class: "mb-2",
                button {
                    class: "btn btn-primary inline-block mr-2",
                    onclick: move |_| {
                        let new_date = date().pred_opt();
                        if let Some(new_date) = new_date {
                            navigator
                                .push(Route::TimelineList {
                                    date: new_date,
                                });
                        }
                    },
                    "<"
                }
                p { class: "inline-block", {date.to_string()} }
                button {
                    class: "btn btn-primary inline-block ml-2",
                    onclick: move |_| {
                        let new_date = Local::now().date_naive();
                        navigator
                            .push(Route::TimelineList {
                                date: new_date,
                            });
                    },
                    "today"
                }
                button {
                    class: "btn btn-primary inline-block ml-2",
                    onclick: move |_| {
                        let new_date = date().succ_opt();
                        if let Some(new_date) = new_date {
                            navigator
                                .push(Route::TimelineList {
                                    date: new_date,
                                });
                        }
                    },
                    ">"
                }
            }
        }

        match timeline.read().deref() {
            Some(Err(err)) => rsx! {
                div { class: "alert alert-danger",
                    "Error loading timeline"
                    {err.to_string()}
                }
            },
            Some(Ok(timeline)) if timeline.is_empty() => rsx! {
                p { class: "alert alert-info", "No entries found for this date." }
            },
            Some(Ok(timeline)) => rsx! {
                table { class: "block sm:table",
                    thead { class: "hidden sm:table-header-group",
                        tr {
                            th { "When" }
                            th { "What" }
                            th { "How Long" }
                            th { "Details" }
                        }
                    }
                    tbody { class: "block sm:table-row-group",
                        for entry in timeline.iter() {
                            EntryRow {
                                key: "{entry.get_id().as_str()}",
                                entry: entry.clone(),
                                on_click: move |entry: Entry| {
                                    match &entry.data {
                                        EntryData::Wee(wee) => {
                                            navigator.push(Route::WeeDetail { wee_id: wee.id });
                                        }
                                        EntryData::Poo(poo) => {
                                            navigator.push(Route::PooDetail { poo_id: poo.id });
                                        }
                                    }
                                },
                            }
                        }
                    }
                }
            },
            None => {
                rsx! {
                    p { class: "alert alert-info", "Loading..." }
                }
            }
        }

        match &*active_dialog.read() {
            ActiveDialog::ChangeWee(op) => {
                rsx! {
                    ChangeWee {
                        op: op.clone(),
                        on_cancel: move || active_dialog.set(ActiveDialog::None),
                        on_save: move |_wee| {
                            active_dialog.set(ActiveDialog::None);
                            timeline.restart();
                        },
                    }
                }
            }
            ActiveDialog::ChangePoo(op) => {
                rsx! {
                    ChangePoo {
                        op: op.clone(),
                        on_cancel: move || active_dialog.set(ActiveDialog::None),
                        on_save: move |_poo| {
                            active_dialog.set(ActiveDialog::None);
                            timeline.restart();
                        },
                    }
                }
            }
            ActiveDialog::None => {
                rsx! {}
            }
        }
    }
}
