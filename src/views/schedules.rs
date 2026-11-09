use std::ops::Deref;

use chrono::Local;
use dioxus::prelude::*;
use dioxus_fullstack::ServerFnError;
use dioxus_router::navigator;
use penguin_nurse::validation::schedule_errors;
use tap::Pipe;

use crate::{
    Route,
    components::{
        StrIcon,
        buttons::{ChangeButton, CreateButton, DeleteButton},
        consumptions::ConsumptionTypeIcon,
        events::Markdown,
        schedules::{
            ActiveDialog, ListDialogReference, Operation, ScheduleAddToConsumption, ScheduleDialog,
            ScheduleDue, ScheduleErrors, ScheduleItemList, ScheduleItemSummary,
        },
    },
    forms::Dialog,
    functions::schedules::{get_schedule_by_id, get_schedules, get_today_schedule_status},
    models::{Schedule, ScheduleId, ScheduleItem, ScheduleStatus, ScheduleWithItems},
    use_user,
};

/// How often the due/upcoming lists refresh, in seconds.
const STATUS_REFRESH_INTERVAL_SECS: u64 = 30 * 60;

#[component]
fn EntryRow(
    schedule_with_items: ScheduleWithItems,
    selected: Signal<Option<ScheduleId>>,
) -> Element {
    let schedule = schedule_with_items.schedule;
    let items = schedule_with_items.items;
    let id = schedule.id;
    let errors = schedule_errors(&schedule, Some(&items));

    let navigator = navigator();
    rsx! {
        tr {
            class: "hover:bg-gray-500 border-blue-300 mt-2 mb-2 p-2 border-2 w-full sm:w-auto sm:border-none inline-block sm:table-row",
            class: if !schedule.active { "text-gray-400" } else { "" },
            onclick: move |_| { selected.set(Some(id)) },
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                span { class: "sm:hidden", "Time: " }
                {schedule.name()}
            }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                span { class: "sm:hidden", "Type: " }
                {schedule.consumption_type.as_title()}
            }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                ScheduleItemList { list: items.clone() }
            }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                if let Some(comments) = &schedule.comments {
                    Markdown { content: comments.to_string() }
                }
            }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                span { class: "sm:hidden", "Status: " }
                if schedule.active { "Active" } else { "Inactive" }
            }
        }

        {
            errors
                .into_iter()
                .map(|error| {
                    rsx! {
                        tr {
                            td { colspan: "5", class: "block sm:table-cell",
                                div { class: "text-error", {error} }
                            }
                        }
                    }
                })
        }

        if selected() == Some(id) {
            tr {
                td { colspan: "5", class: "block sm:table-cell",
                    div { class: "flex flex-wrap gap-2",
                        ChangeButton {
                            on_click: move |_| {
                                navigator
                                    .push(Route::ScheduleList {
                                        dialog: ListDialogReference::UpdateBasic {
                                            schedule_id: id,
                                        },
                                    });
                            },
                            "Edit"
                        }
                        ChangeButton {
                            on_click: move |_| {
                                navigator
                                    .push(Route::ScheduleList {
                                        dialog: ListDialogReference::UpdateIngredients {
                                            schedule_id: id,
                                        },
                                    });
                            },
                            "Ingredients"
                        }
                        DeleteButton {
                            on_click: move |_| {
                                navigator
                                    .push(Route::ScheduleList {
                                        dialog: ListDialogReference::Delete {
                                            schedule_id: id,
                                        },
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

#[component]
pub fn ScheduleList(dialog: ReadSignal<Option<ListDialogReference>>) -> Element {
    let user = use_user().ok().flatten();

    let Some(user) = user.as_ref() else {
        return rsx! {
            p { class: "alert alert-error", "You are not logged in." }
        };
    };

    let user_id = user.pipe(|x| x.id);

    // The time is taken inside the resource so that restarts pick up a fresh
    // "now" and the due/upcoming split rolls over during the day.
    let mut status: Resource<Result<Vec<ScheduleStatus>, ServerFnError>> =
        use_resource(move || async move {
            let now = Local::now().fixed_offset();
            get_today_schedule_status(user_id, now).await
        });

    // Refresh the due/upcoming lists every 30 minutes so they roll over if the
    // page is left open. Only on the client; the server renders once.
    use_hook(move || {
        if cfg!(target_arch = "wasm32") {
            spawn(async move {
                loop {
                    gloo_timers::future::sleep(std::time::Duration::from_secs(
                        STATUS_REFRESH_INTERVAL_SECS,
                    ))
                    .await;
                    status.restart();
                }
            });
        }
    });

    let mut schedules: Resource<Result<Vec<ScheduleWithItems>, ServerFnError>> =
        use_resource(move || async move { get_schedules(user_id).await });

    let active_dialog: Resource<Result<ActiveDialog, ServerFnError>> =
        use_resource(move || async move {
            let Some(dialog) = dialog() else {
                return Ok(ActiveDialog::Idle);
            };
            match dialog {
                ListDialogReference::Create => {
                    ActiveDialog::Change(Operation::Create { user_id }).pipe(Ok)
                }
                ListDialogReference::UpdateBasic { schedule_id } => {
                    let schedule = get_schedule_by_id(schedule_id)
                        .await?
                        .ok_or(ServerFnError::new("Cannot find schedule"))?;
                    ActiveDialog::Change(Operation::Update { schedule }).pipe(Ok)
                }
                ListDialogReference::UpdateIngredients { schedule_id } => {
                    let schedule = get_schedule_by_id(schedule_id)
                        .await?
                        .ok_or(ServerFnError::new("Cannot find schedule"))?;
                    ActiveDialog::Ingredients(schedule).pipe(Ok)
                }
                ListDialogReference::Delete { schedule_id } => {
                    let schedule = get_schedule_by_id(schedule_id)
                        .await?
                        .ok_or(ServerFnError::new("Cannot find schedule"))?;
                    ActiveDialog::Delete(schedule).pipe(Ok)
                }
                ListDialogReference::Idle => Ok(ActiveDialog::Idle),
            }
        });

    let mut picker: Signal<Option<(Schedule, Vec<ScheduleItem>)>> = use_signal(|| None);

    let on_recorded = Callback::new(move |()| {
        status.restart();
    });

    let on_add_existing = Callback::new(move |(schedule, items): (Schedule, Vec<ScheduleItem>)| {
        picker.set(Some((schedule, items)));
    });

    let navigator = navigator();

    rsx! {
        div { class: "ml-2 mr-2",
            div { class: "font-bold text-lg", "Due" }
            div { class: "mb-2 text-sm text-gray-600 dark:text-gray-400",
                "Scheduled ingredients due today but not yet recorded in a consumption"
            }
        }

        match status.read().deref() {
            Some(Err(err)) => rsx! {
                div { class: "alert alert-error",
                    "Error loading schedules: "
                    {err.to_string()}
                }
            },
            Some(Ok(list)) => {
                let due: Vec<&ScheduleStatus> = list
                    .iter()
                    .filter(|s| {
                        s.due == crate::models::ScheduleDue::Due
                            && s.items.iter().any(|item| !item.recorded)
                    })
                    .collect();
                if due.is_empty() {
                    rsx! {
                        p { class: "alert alert-success",
                            "Nothing due - all scheduled ingredients recorded."
                        }
                    }
                } else {
                    rsx! {
                        div { class: "ml-2 mr-2 sm:ml-0 sm:mr-0",
                            for s in due {
                                ScheduleDue {
                                    key: "{s.schedule.id}",
                                    status: s.clone(),
                                    on_recorded,
                                    on_add_existing,
                                }
                            }
                        }
                    }
                }
            }
            None => {
                rsx! {
                    p { class: "alert alert-info", "Loading..." }
                }
            }
        }

        div { class: "ml-2 mr-2 mt-4",
            div { class: "font-bold text-lg", "Upcoming" }
            div { class: "mb-2 text-sm text-gray-600 dark:text-gray-400",
                "Scheduled ingredients later today"
            }
        }

        match status.read().deref() {
            Some(Err(err)) => rsx! {
                div { class: "alert alert-error",
                    "Error loading schedules: "
                    {err.to_string()}
                }
            },
            Some(Ok(list)) => {
                let upcoming: Vec<&ScheduleStatus> = list
                    .iter()
                    .filter(|s| s.due == crate::models::ScheduleDue::Upcoming)
                    .collect();
                if upcoming.is_empty() {
                    rsx! {
                        p { class: "alert alert-info", "No upcoming schedules today." }
                    }
                } else {
                    rsx! {
                        div { class: "ml-2 mr-2 sm:ml-0 sm:mr-0",
                            for s in upcoming {
                                div {
                                    class: "p-4 mb-2 border-2 rounded-lg border-gray-500",
                                    key: "{s.schedule.id}",
                                    div { class: "flex items-center gap-2",
                                        span { class: "font-bold text-lg", {s.schedule.name()} }
                                        StrIcon {
                                            title: s.schedule.consumption_type.as_title(),
                                            icon: rsx! {
                                                ConsumptionTypeIcon {
                                                    consumption_type: s.schedule.consumption_type,
                                                }
                                            },
                                        }
                                    }
                                    if let Some(comments) = &s.schedule.comments {
                                        Markdown { content: comments.to_string() }
                                    }
                                    ScheduleErrors {
                                        schedule: s.schedule.clone(),
                                        items: Some(
                                            s.items.iter().map(|item| item.item.clone()).collect(),
                                        ),
                                    }
                                    for item in &s.items {
                                        ScheduleItemSummary {
                                            key: "{item.item.consumable.id}",
                                            item: item.item.clone(),
                                            recorded: item.recorded,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            None => {
                rsx! {
                    p { class: "alert alert-info", "Loading..." }
                }
            }
        }

        div { class: "ml-2 mr-2 mt-4",
            div { class: "font-bold text-lg", "All Schedules" }
            div { class: "mb-2",
                CreateButton {
                    on_click: move |_| {
                        navigator
                            .push(Route::ScheduleList {
                                dialog: ListDialogReference::Create,
                            });
                    },
                    "Create"
                }
            }
        }

        match schedules.read().deref() {
            Some(Err(err)) => rsx! {
                div { class: "alert alert-error",
                    "Error loading schedules: "
                    {err.to_string()}
                }
            },
            Some(Ok(list)) if list.is_empty() => rsx! {
                p { class: "alert alert-info", "No schedules found." }
            },
            Some(Ok(list)) => {
                let selected: Signal<Option<ScheduleId>> = use_signal(|| None);
                rsx! {
                    div { class: "ml-2 mr-2 sm:ml-0 sm:mr-0",
                        table { class: "block sm:table",
                            thead { class: "hidden sm:table-header-group",
                                tr {
                                    th { "Time" }
                                    th { "Type" }
                                    th { "Ingredients" }
                                    th { "Comments" }
                                    th { "Status" }
                                }
                            }
                            tbody { class: "block sm:table-row-group",
                                for schedule_with_items in list {
                                    EntryRow {
                                        key: "{schedule_with_items.schedule.id}",
                                        schedule_with_items: schedule_with_items.clone(),
                                        selected,
                                    }
                                }
                            }
                        }
                    }
                }
            }
            None => {
                rsx! {
                    p { class: "alert alert-info", "Loading..." }
                }
            }
        }

        match active_dialog.read().deref() {
            Some(Err(err)) => rsx! {
                div { class: "alert alert-error",
                    "Error loading dialog: "
                    {err.to_string()}
                }
            },
            Some(Ok(dialog)) => rsx! {
                ScheduleDialog {
                    dialog: dialog.clone(),
                    on_change: move |_schedule: Schedule| {
                        status.restart();
                        schedules.restart();
                    },
                    on_change_ingredients: move |_schedule: Schedule| {
                        status.restart();
                        schedules.restart();
                    },
                    on_delete: move |_schedule| {
                        status.restart();
                        schedules.restart();
                    },
                    show_update_basic: move |schedule: Schedule| {
                        navigator
                            .push(Route::ScheduleList {
                                dialog: ListDialogReference::UpdateBasic {
                                    schedule_id: schedule.id,
                                },
                            });
                    },
                    show_update_ingredients: move |schedule: Schedule| {
                        navigator
                            .push(Route::ScheduleList {
                                dialog: ListDialogReference::UpdateIngredients {
                                    schedule_id: schedule.id,
                                },
                            });
                    },
                    on_close: move |()| {
                        navigator
                            .push(Route::ScheduleList {
                                dialog: ListDialogReference::Idle,
                            });
                    },
                }
            },
            None => {
                rsx! {
                    p { class: "alert alert-info", "Loading..." }
                }
            }
        }

        if let Some(selection) = picker() {
            Dialog {
                ScheduleAddToConsumption {
                    schedule: selection.0.clone(),
                    items: selection.1.clone(),
                    on_close: move |_| {
                        picker.set(None);
                    },
                    on_added: move |()| {
                        picker.set(None);
                        status.restart();
                    },
                }
            }
        }
    }
}
