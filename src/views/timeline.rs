use std::ops::Deref;

use chrono::{NaiveDate, Utc};
use dioxus::prelude::{server_fn::error::NoCustomError, *};
use tap::Pipe;

use crate::{
    Route,
    components::{
        buttons::{ChangeButton, CreateButton, DeleteButton, NavButton},
        consumptions::{self, ConsumptionItemList, consumption_duration, consumption_icon},
        events::{event_colour, event_time, event_urgency},
        poos::{self, poo_bristol, poo_duration, poo_icon, poo_quantity},
        timeline::{ActiveDialog, DialogReference, TimelineDialog},
        wees::{self, wee_duration, wee_icon, wee_mls},
    },
    dt::{display_date, get_date_for_dt, get_utc_times_for_date},
    functions::{
        consumptions::{get_consumption_by_id, get_consumptions_for_time_range},
        poos::{get_poo_by_id, get_poos_for_time_range},
        wees::{get_wee_by_id, get_wees_for_time_range},
    },
    models::{Consumption, Entry, EntryData, EntryId, Maybe, Timeline},
    use_user,
};

#[component]
fn EntryRow(
    entry: Entry,
    date: ReadOnlySignal<NaiveDate>,
    selected: Signal<Option<EntryId>>,
) -> Element {
    let id = entry.get_id();
    let navigator = navigator();

    rsx! {
        tr {
            class: "hover:bg-gray-500 border-blue-300 mt-2 mb-2 p-2 border-2 w-full sm:w-auto sm:border-none inline-block sm:table-row",
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
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2", consumption_icon {
                            consumption_type: consumption.consumption.consumption_type
                        } }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            consumption_duration { duration: consumption.consumption.duration }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            { consumption.consumption.consumption_type.to_string() }
                            if let Maybe::Some(liquid_mls) = &consumption.consumption.liquid_mls {
                                div {
                                    "Liquid: "
                                    {liquid_mls.to_string()}
                                    "ml"
                                }
                            }
                            if !consumption.items.is_empty() {
                                ConsumptionItemList { list: consumption.items.clone() }
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
                        rsx! {
                            NavButton {
                                on_click: move |_| {
                                    navigator.push(Route::WeeDetail { wee_id: wee.id, dialog: wees::DialogReference::Idle });
                                },
                                "Details"
                            }
                            ChangeButton {
                                on_click: move |_| {
                                    navigator
                                    .push(Route::TimelineList {
                                        date: date(),
                                        dialog: DialogReference::UpdateWee { wee_id: wee.id }
                                    });
                                },
                                "Edit"
                            }
                            DeleteButton {
                                on_click: move |_| {
                                    navigator
                                    .push(Route::TimelineList {
                                        date: date(),
                                        dialog: DialogReference::DeleteWee{ wee_id: wee.id }
                                    });
                                },
                                "Delete"
                            }
                        }
                    }
                    EntryData::Poo(poo) => {
                        rsx! {
                            NavButton {
                                on_click: move |_| {
                                    navigator.push(Route::PooDetail { poo_id: poo.id, dialog: poos::DialogReference::Idle });
                                },
                                "Details"
                            }
                            ChangeButton {
                                on_click: move |_| {
                                    navigator
                                    .push(Route::TimelineList {
                                        date: date(),
                                        dialog: DialogReference::UpdatePoo{ poo_id: poo.id }
                                    });
                                },
                                "Edit"
                            }
                            DeleteButton {
                                on_click: move |_| {
                                    navigator
                                    .push(Route::TimelineList {
                                        date: date(),
                                        dialog: DialogReference::DeletePoo{ poo_id: poo.id }
                                    });
                                 },
                                "Delete"
                            }
                        }
                    }
                    EntryData::Consumption(consumption) => {
                        let consumption = consumption.consumption;
                        rsx! {
                            NavButton {
                                on_click: move |_| {
                                    navigator
                                        .push(Route::ConsumptionDetail {
                                            consumption_id: consumption.id,
                                            dialog: consumptions::DialogReference::Idle
                                        });
                                },
                                "Details"
                            }
                            ChangeButton {
                                on_click: move |_| {
                                    navigator
                                    .push(Route::TimelineList {
                                        date: date(),
                                        dialog: DialogReference::UpdateConsumptionIngredients{ consumption_id: consumption.id }
                                    });
                                },
                                "Ingredients"
                            }
                            ChangeButton {
                                on_click: move |_| {
                                    navigator
                                    .push(Route::TimelineList {
                                        date: date(),
                                        dialog: DialogReference::UpdateConsumption{ consumption_id: consumption.id }
                                    });
                                },
                                "Edit"
                            }
                            DeleteButton {
                                on_click: move |_| {
                                    navigator
                                    .push(Route::TimelineList {
                                        date: date(),
                                        dialog: DialogReference::DeleteConsumption{ consumption_id: consumption.id }
                                    });
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
pub fn TimelineList(
    date: ReadOnlySignal<NaiveDate>,
    dialog: ReadOnlySignal<Option<DialogReference>>,
) -> Element {
    let navigator = navigator();
    let selected: Signal<Option<EntryId>> = use_signal(|| None);
    let user = use_user().ok().flatten();

    let Some(user) = user.as_ref() else {
        return rsx! {
            p { class: "alert alert-error", "You are not logged in." }
        };
    };

    let user_id = user.pipe(|x| x.id);

    let dialog: Resource<Result<ActiveDialog, ServerFnError>> = use_resource(move || async move {
        let Some(dialog) = dialog() else {
            return Ok(ActiveDialog::Idle);
        };
        match dialog {
            DialogReference::CreateWee { user_id } => {
                ActiveDialog::Wee(wees::ActiveDialog::Change(wees::Operation::Create {
                    user_id,
                }))
                .pipe(Ok)
            }
            DialogReference::UpdateWee { wee_id } => {
                let wee = get_wee_by_id(wee_id).await?.ok_or(
                    ServerFnError::<NoCustomError>::ServerError("Cannot find wee".to_string()),
                )?;
                ActiveDialog::Wee(wees::ActiveDialog::Change(wees::Operation::Update { wee }))
                    .pipe(Ok)
            }
            DialogReference::DeleteWee { wee_id } => {
                let wee = get_wee_by_id(wee_id).await?.ok_or(
                    ServerFnError::<NoCustomError>::ServerError("Cannot find wee".to_string()),
                )?;
                ActiveDialog::Wee(wees::ActiveDialog::Delete(wee)).pipe(Ok)
            }
            DialogReference::CreatePoo { user_id } => {
                ActiveDialog::Poo(poos::ActiveDialog::Change(poos::Operation::Create {
                    user_id,
                }))
                .pipe(Ok)
            }
            DialogReference::UpdatePoo { poo_id } => {
                let poo = get_poo_by_id(poo_id).await?.ok_or(
                    ServerFnError::<NoCustomError>::ServerError("Cannot find poo".to_string()),
                )?;
                ActiveDialog::Poo(poos::ActiveDialog::Change(poos::Operation::Update { poo }))
                    .pipe(Ok)
            }
            DialogReference::DeletePoo { poo_id } => {
                let poo = get_poo_by_id(poo_id).await?.ok_or(
                    ServerFnError::<NoCustomError>::ServerError("Cannot find poo".to_string()),
                )?;
                ActiveDialog::Poo(poos::ActiveDialog::Delete(poo)).pipe(Ok)
            }
            DialogReference::CreateConsumption { user_id } => ActiveDialog::Consumption(
                consumptions::ActiveDialog::Change(consumptions::Operation::Create { user_id }),
            )
            .pipe(Ok),
            DialogReference::UpdateConsumption { consumption_id } => {
                let consumption =
                    get_consumption_by_id(consumption_id)
                        .await?
                        .ok_or(ServerFnError::<NoCustomError>::ServerError(
                            "Cannot find consumption".to_string(),
                        ))?;
                ActiveDialog::Consumption(consumptions::ActiveDialog::Change(
                    consumptions::Operation::Update { consumption },
                ))
                .pipe(Ok)
            }
            DialogReference::UpdateConsumptionIngredients { consumption_id } => {
                let consumption =
                    get_consumption_by_id(consumption_id)
                        .await?
                        .ok_or(ServerFnError::<NoCustomError>::ServerError(
                            "Cannot find consumption".to_string(),
                        ))?;
                ActiveDialog::Consumption(consumptions::ActiveDialog::Ingredients(consumption))
                    .pipe(Ok)
            }
            DialogReference::DeleteConsumption { consumption_id } => {
                let consumption =
                    get_consumption_by_id(consumption_id)
                        .await?
                        .ok_or(ServerFnError::<NoCustomError>::ServerError(
                            "Cannot find consumption".to_string(),
                        ))?;
                ActiveDialog::Consumption(consumptions::ActiveDialog::Delete(consumption)).pipe(Ok)
            }
            DialogReference::Idle => Ok(ActiveDialog::Idle),
        }
    });

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
        div { class: "ml-2 mr-2",
            div { class: "mb-2",
                CreateButton {
                    on_click: move |_| {
                        navigator.push(Route::TimelineList {
                            date: date(),
                            dialog: DialogReference::CreateWee { user_id }
                        });
                    },
                    "Wee"
                }
                CreateButton {
                    on_click: move |_| {
                        navigator.push(Route::TimelineList {
                            date: date(),
                            dialog: DialogReference::CreatePoo{ user_id }
                        });
                    },
                    "Poo"
                }
                CreateButton {
                    on_click: move |_| {
                        navigator.push(Route::TimelineList {
                            date: date(),
                            dialog: DialogReference::CreateConsumption{ user_id }
                        });
                    },
                    "Consumption"
                }
            }

            div { class: "mb-2",
                NavButton {
                    on_click: move |_| {
                        let new_date = date().pred_opt();
                        if let Some(new_date) = new_date {
                            navigator
                                .push(Route::TimelineList {
                                    date: new_date,
                                    dialog: DialogReference::Idle
                                });
                        }
                    },
                    "<"
                }
                NavButton {
                    on_click: move |_| {
                        let new_date = get_date_for_dt(Utc::now());
                            navigator
                                .push(Route::TimelineList {
                                    date: new_date,
                                    dialog: DialogReference::Idle
                                });
                    },
                    "Today"
                }
                NavButton {
                    on_click: move |_| {
                        let new_date = date().succ_opt();
                        if let Some(new_date) = new_date {
                            navigator
                                .push(Route::TimelineList {
                                    date: new_date,
                                    dialog: DialogReference::Idle
                                });
                        }
                    },
                    ">"
                }
                p { class: "inline-block", {display_date(date())} }
            }
        }

        match timeline.read().deref() {
            Some(Err(err)) => rsx! {
                div { class: "alert alert-error",
                    "Error loading timeline: "
                    {err.to_string()}
                }
            },
            Some(Ok(timeline)) if timeline.is_empty() => rsx! {
                p { class: "alert alert-info", "No entries found for this date." }
            },
            Some(Ok(timeline)) => rsx! {
                div { class: "ml-2 mr-2 sm:ml-0 sm:mr-0",
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
                                    date: date(),
                                    selected,
                                }
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

        match dialog.read().deref() {
            Some(Err(err)) => rsx! {
                div { class: "alert alert-error",
                    "Error loading dialog: "
                    {err.to_string()}
                }
            },
            Some(Ok(dialog)) => rsx! {
                TimelineDialog {
                    dialog: dialog.clone(),
                    on_change: move || timeline.restart(),
                    show_consumption_edit: move |consumption: Consumption| {
                        navigator
                            .push(Route::TimelineList {
                                date: date(),
                                dialog: DialogReference::UpdateConsumption { consumption_id: consumption.id }
                            });
                    },
                    show_consumption_ingredients: move |consumption: Consumption| {
                        navigator
                            .push(Route::TimelineList {
                                date: date(),
                                dialog: DialogReference::UpdateConsumptionIngredients { consumption_id: consumption.id }
                            });
                    },
                    on_close: move || {
                            navigator
                                .push(Route::TimelineList {
                                    date: date(),
                                    dialog: DialogReference::Idle
                                });

                    }
                }
            },
            None => {
                rsx! {
                    p { class: "alert alert-info", "Loading..." }
                }
            }
        }
    }
}
