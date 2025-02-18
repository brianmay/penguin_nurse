use std::ops::Deref;

use chrono::{Local, NaiveDate, Utc};
use dioxus::prelude::*;
use tap::Pipe;

use crate::{
    components::{
        consumptions::{self, consumption_duration, consumption_icon},
        events::{event_colour, event_time, event_urgency},
        poos::{self, poo_bristol, poo_duration, poo_icon, poo_quantity},
        timeline::{ActiveDialog, TimelineDialog},
        wees::{self, wee_duration, wee_icon, wee_mls},
    },
    dt::{get_date_for_dt, get_utc_times_for_date},
    functions::{
        consumptions::get_consumptions_for_time_range, poos::get_poos_for_time_range,
        wees::get_wees_for_time_range,
    },
    models::{Entry, EntryData, EntryId, Maybe, Timeline},
    use_user, Route,
};

#[component]
fn EntryRow(
    entry: Entry,
    dialog: Signal<ActiveDialog>,
    selected: Signal<Option<EntryId>>,
) -> Element {
    let id = entry.get_id();

    rsx! {
        tr {
            class: "hover:bg-gray-500 border-blue-300 m-2 p-2 border-2 h-96 w-48 sm:w-auto sm:border-none sm:h-auto inline-block sm:table-row",
            onclick: move |_| selected.set(Some(id)),
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
                                if let Maybe::Some(comments) = &wee.comments {
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
                                if let Maybe::Some(comments) = &poo.comments {
                                    div { {comments.to_string()} }
                                }
                            }
                        }
                    }
                }
                EntryData::Consumption(consumption) => {
                    rsx! {
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2", consumption_icon {} }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            consumption_duration { duration: consumption.consumption.duration }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            if let Maybe::Some(liquid_mls) = &consumption.consumption.liquid_mls {
                                div {
                                    "Liquid: "
                                    {liquid_mls.to_string()}
                                    "ml"
                                }
                            }
                            if !consumption.items.is_empty() {
                                p { "Ingredients:" }
                                ul { class: "list-disc ml-4",
                                    for item in &consumption.items {
                                        li {
                                            if let Maybe::Some(quantity) = item.nested.quantity {
                                                span {
                                                    {quantity.to_string()}
                                                    {item.consumable.unit.postfix()}
                                                    " "
                                                }
                                            }
                                            {item.consumable.name.clone()}
                                            if let Maybe::Some(brand) = &item.consumable.brand {
                                                ", "
                                                {brand.clone()}
                                            }
                                            if let Maybe::Some(dt) = &item.consumable.created {
                                                {dt.with_timezone(&Local).format(" %Y-%m-%d").to_string()}
                                            }
                                            if let Maybe::Some(comments) = &item.nested.comments {
                                                " ("
                                                {comments.to_string()}
                                                ")"
                                            }
                                            if let Maybe::Some(liquid_mls) = item.nested.liquid_mls {
                                                span {
                                                    " Liquid: "
                                                    {liquid_mls.to_string()}
                                                    "ml"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            if let Maybe::Some(comments) = &consumption.consumption.comments {
                                div { {comments.to_string()} }
                            }
                        }
                    }
                }
            }
        }
        if selected() == Some(id) {
            td { colspan: 4, class: "block sm:table-cell",
                match entry.data {
                    EntryData::Wee(wee) => {
                        let wee_clone_1 = wee.clone();
                        let wee_clone_2 = wee.clone();
                        rsx! {
                            button {
                                class: "btn btn-primary m-1",
                                onclick: move |_| {
                                    dialog.set(ActiveDialog::Wee(wees::ActiveDialog::Details(wee_clone_1.clone())))
                                },
                                "Details"
                            }
                            button {
                                class: "btn btn-primary m-1",
                                onclick: move |_| {
                                    dialog
                                        .set(
                                            ActiveDialog::Wee(
                                                wees::ActiveDialog::Change(wees::Operation::Update {
                                                    wee: wee_clone_2.clone(),
                                                }),
                                            ),
                                        )
                                },
                                "Edit"
                            }
                            button {
                                class: "btn btn-secondary m-1",
                                onclick: move |_| { dialog.set(ActiveDialog::Wee(wees::ActiveDialog::Delete(wee.clone()))) },
                                "Delete"
                            }
                        }
                    }
                    EntryData::Poo(poo) => {
                        let poo_clone_1 = poo.clone();
                        let poo_clone_2 = poo.clone();
                        rsx! {
                            button {
                                class: "btn btn-primary m-1",
                                onclick: move |_| {
                                    dialog.set(ActiveDialog::Poo(poos::ActiveDialog::Details(poo_clone_1.clone())))
                                },
                                "Details"
                            }
                            button {
                                class: "btn btn-primary m-1",
                                onclick: move |_| {
                                    dialog
                                        .set(
                                            ActiveDialog::Poo(
                                                poos::ActiveDialog::Change(poos::Operation::Update {
                                                    poo: poo_clone_2.clone(),
                                                }),
                                            ),
                                        )
                                },
                                "Edit"
                            }
                            button {
                                class: "btn btn-secondary m-1",
                                onclick: move |_| { dialog.set(ActiveDialog::Poo(poos::ActiveDialog::Delete(poo.clone()))) },
                                "Delete"
                            }
                        }
                    }
                    EntryData::Consumption(consumption) => {
                        let consumption_clone_1 = consumption.consumption.clone();
                        let consumption_clone_2 = consumption.consumption.clone();
                        let consumption_clone_3 = consumption.consumption.clone();
                        let consumption = consumption.consumption;
                        rsx! {
                            button {
                                class: "btn btn-primary m-1",
                                onclick: move |_| {
                                    dialog
                                        .set(
                                            ActiveDialog::Consumption(
                                                consumptions::ActiveDialog::Details(consumption_clone_1.clone()),
                                            ),
                                        )
                                },
                                "Details"
                            }
                            button {
                                class: "btn btn-primary m-1",
                                onclick: move |_| {
                                    dialog
                                        .set(
                                            ActiveDialog::Consumption(
                                                consumptions::ActiveDialog::Consumption(consumption_clone_2.clone()),
                                            ),
                                        )
                                },
                                "Ingredients"
                            }
                            button {
                                class: "btn btn-primary m-1",
                                onclick: move |_| {
                                    dialog
                                        .set(
                                            ActiveDialog::Consumption(
                                                consumptions::ActiveDialog::Change(consumptions::Operation::Update {
                                                    consumption: consumption_clone_3.clone(),
                                                }),
                                            ),
                                        )
                                },
                                "Edit"
                            }
                            button {
                                class: "btn btn-secondary m-1",
                                onclick: move |_| {
                                    dialog
                                        .set(
                                            ActiveDialog::Consumption(
                                                consumptions::ActiveDialog::Delete(consumption.clone()),
                                            ),
                                        )
                                },
                                "Delete"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn TimelineList(date: ReadOnlySignal<NaiveDate>) -> Element {
    let navigator = navigator();
    let selected: Signal<Option<EntryId>> = use_signal(|| None);
    let user = use_user().ok().flatten();

    let Some(user) = user.as_ref() else {
        return rsx! {
            p { class: "alert alert-danger", "You are not logged in." }
        };
    };

    let user_id = user.pipe(|x| x.id);

    let mut dialog = use_signal(|| ActiveDialog::Idle);

    let mut timeline: Resource<Result<Timeline, ServerFnError>> =
        use_resource(move || async move {
            let (start, end) = get_utc_times_for_date(date())?;

            let mut timeline = Timeline::new();
            let wees = get_wees_for_time_range(user_id, start, end).await?;
            timeline.add_wees(wees);

            let poos = get_poos_for_time_range(user_id, start, end).await?;
            timeline.add_poos(poos);

            let consumptions = get_consumptions_for_time_range(user_id, start, end).await?;
            timeline.add_consumptions(consumptions);

            timeline.sort();

            Ok(timeline)
        });

    rsx! {
        div { class: "ml-2",
            div { class: "mb-2",
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        dialog
                            .set(
                                ActiveDialog::Wee(
                                    wees::ActiveDialog::Change(wees::Operation::Create { user_id }),
                                ),
                            )
                    },
                    "Wee"
                }
                button {
                    class: "btn btn-primary ml-2",
                    onclick: move |_| {
                        dialog
                            .set(
                                ActiveDialog::Poo(
                                    poos::ActiveDialog::Change(poos::Operation::Create { user_id }),
                                ),
                            )
                    },
                    "Poo"
                }
                button {
                    class: "btn btn-primary ml-2",
                    onclick: move |_| {
                        dialog
                            .set(
                                ActiveDialog::Consumption(
                                    consumptions::ActiveDialog::Change(consumptions::Operation::Create {
                                        user_id,
                                    }),
                                ),
                            )
                    },
                    "Consumption"
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
                        let new_date = get_date_for_dt(Utc::now());
                        if let Ok(new_date) = new_date {
                            navigator
                                .push(Route::TimelineList {
                                    date: new_date,
                                });
                        }
                    },
                    "Today"
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
                                selected,
                                dialog,
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

        TimelineDialog { dialog, on_change: move || timeline.restart() }
    }
}
