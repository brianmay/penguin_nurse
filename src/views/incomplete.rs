use std::ops::Deref;

use dioxus::prelude::*;
use dioxus_fullstack::ServerFnError;
use tap::Pipe;

use crate::{
    components::{
        StrIcon,
        buttons::{ChangeButton, DeleteButton},
        consumptions::{
            ConsumptionDetails, ConsumptionDuration, ConsumptionItemList, ConsumptionTypeIcon,
        },
        events::EventTime,
        exercises::ExerciseTypeIcon,
        exercises::{ExerciseDetails, ExerciseDuration},
        health_metrics::{HealthMetricDetails, HealthMetricIcon, health_metric_title},
        notes::{NoteDetails, note_icon, note_title},
        poos::{PooDetails, PooDuration, PooIcon, poo_title},
        refluxs::{RefluxDetails, reflux_duration, reflux_icon, reflux_title},
        symptoms::{SymptomDetails, symptom_icon, symptom_title},
        timeline::{ActiveDialog, DialogReference, TimelineDialog},
        wee_urges::{WeeUrgeDetails, WeeUrgeIcon, wee_urge_title},
        wees::{WeeDetails, WeeDuration, WeeIcon, wee_title},
    },
    dt::display_date,
    functions::{
        consumptions::{get_consumption_by_id, get_consumptions_incomplete},
        exercises::{get_exercise_by_id, get_exercises_incomplete},
        poos::{get_poo_by_id, get_poos_incomplete},
        refluxs::{get_reflux_by_id, get_refluxs_incomplete},
        wee_urges::get_wee_urge_by_id,
        wees::{get_wee_by_id, get_wees_incomplete},
    },
    models::{Entry, EntryData, EntryId, Timeline},
    use_user,
};

#[component]
fn IncompleteEntryRow(
    entry: ReadSignal<Entry>,
    selected: Signal<Option<EntryId>>,
    on_edit: Callback<DialogReference>,
    on_delete: Callback<DialogReference>,
) -> Element {
    let entry: Entry = entry();
    let id = entry.get_id();
    let update_dialog_reference = DialogReference::get_update_dialog_reference(&entry);
    let delete_dialog_reference = DialogReference::get_delete_dialog_reference(&entry);

    rsx! {
        tr {
            class: "hover:bg-gray-500 border-blue-300 mt-2 mb-2 p-2 border-2 w-full sm:w-auto sm:border-none inline-block sm:table-row",
            onclick: move |_| selected.set(Some(id)),
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                EventTime { time: entry.time }
            }
            match &entry.data {
                EntryData::Wee(wee) => {
                    rsx! {
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            StrIcon { title: wee_title(), icon: WeeIcon() }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            WeeDuration { duration: wee.duration }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            WeeDetails { wee: wee.clone() }
                        }
                    }
                }
                EntryData::WeeUrge(wee_urge) => {
                    rsx! {
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            StrIcon { title: wee_urge_title(), icon: WeeUrgeIcon() }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2" }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            WeeUrgeDetails { wee_urge: wee_urge.clone() }
                        }
                    }
                }
                EntryData::Poo(poo) => {
                    rsx! {
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            StrIcon { title: poo_title(), icon: PooIcon() }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            PooDuration { duration: poo.duration }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            PooDetails { poo: poo.clone() }
                        }
                    }
                }
                EntryData::Consumption(consumption) => {
                    rsx! {
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            StrIcon {
                                title: &consumption.consumption.consumption_type.as_title(),
                                icon: rsx! {
                                    ConsumptionTypeIcon { consumption_type: consumption.consumption.consumption_type }
                                },
                            }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            ConsumptionDuration { duration: consumption.consumption.duration }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            ConsumptionDetails { consumption: consumption.consumption.clone() }
                            if !consumption.items.is_empty() {
                                ConsumptionItemList { list: consumption.items.clone() }
                            }
                        }
                    }
                }
                EntryData::Exercise(exercise) => {
                    rsx! {
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            StrIcon {
                                title: &exercise.exercise_type.as_title(),
                                icon: rsx! {
                                    ExerciseTypeIcon { exercise_type: exercise.exercise_type }
                                },
                            }

                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            ExerciseDuration { duration: exercise.duration }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            ExerciseDetails { exercise: exercise.clone() }
                        }
                    }
                }
                EntryData::HealthMetric(health_metric) => {
                    rsx! {
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            StrIcon { title: health_metric_title(), icon: HealthMetricIcon() }

                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2" }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            HealthMetricDetails { health_metric: health_metric.clone() }
                        }
                    }
                }
                EntryData::Symptom(symptom) => {
                    rsx! {
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            StrIcon { title: symptom_title(), icon: symptom_icon() }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2" }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            SymptomDetails { symptom: symptom.clone() }
                        }
                    }
                }
                EntryData::Reflux(reflux) => {
                    rsx! {
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            StrIcon { title: reflux_title(), icon: reflux_icon() }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            reflux_duration { duration: reflux.duration }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            RefluxDetails { reflux: reflux.clone() }
                        }
                    }
                }
                EntryData::Note(note) => {
                    rsx! {
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            StrIcon { title: note_title(), icon: note_icon() }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2" }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            NoteDetails { note: note.clone() }
                        }
                    }
                }
            }
        }
        if selected() == Some(id) {
            td { colspan: 4, class: "block sm:table-cell",
                div { class: "flex flex-wrap gap-2",
                    ChangeButton {
                        on_click: move |_| on_edit.call(update_dialog_reference.clone()),
                        "Edit"
                    }
                    DeleteButton {
                        on_click: move |_| on_delete.call(delete_dialog_reference.clone()),
                        "Delete"
                    }
                    match &entry.data {
                        EntryData::Consumption(cons_item) => {
                            let cid = cons_item.consumption.id;
                            rsx! {
                                ChangeButton {
                                    on_click: move |_| {
                                        on_edit.call(DialogReference::UpdateIngredients {
                                            consumption_id: cid,
                                        });
                                    },
                                    "Ingredients"
                                }
                            }
                        }
                        _ => rsx! {},
                    }
                }
            }
        }
    }
}

#[component]
pub fn IncompleteList() -> Element {
    let selected: Signal<Option<EntryId>> = use_signal(|| None);
    let mut dialog: Signal<Option<DialogReference>> = use_signal(|| None);
    let user = use_user().ok().flatten();

    let Some(user) = user.as_ref() else {
        return rsx! {
            p { class: "alert alert-error", "You are not logged in." }
        };
    };

    let user_id = user.pipe(|x| x.id);

    let mut timeline: Resource<Result<Timeline, ServerFnError>> =
        use_resource(move || async move {
            let mut timeline = Timeline::new();

            let wees = get_wees_incomplete(user_id).await?;
            timeline.add_wees(wees);

            let poos = get_poos_incomplete(user_id).await?;
            timeline.add_poos(poos);

            let consumptions = get_consumptions_incomplete(user_id).await?;
            timeline.add_consumptions(consumptions);

            let exercises = get_exercises_incomplete(user_id).await?;
            timeline.add_exercises(exercises);

            let refluxs = get_refluxs_incomplete(user_id).await?;
            timeline.add_refluxs(refluxs);

            timeline.sort();

            Ok(timeline)
        });

    let active_dialog: Resource<Result<ActiveDialog, ServerFnError>> =
        use_resource(move || async move {
            let Some(dialog_ref) = dialog() else {
                return Ok(ActiveDialog::Idle);
            };
            match dialog_ref {
                DialogReference::UpdateWee { wee_id } => {
                    let wee = get_wee_by_id(wee_id)
                        .await?
                        .ok_or(ServerFnError::new("Cannot find wee"))?;
                    ActiveDialog::Wee(crate::components::wees::ActiveDialog::Change(
                        crate::components::wees::Operation::Update { wee },
                    ))
                    .pipe(Ok)
                }
                DialogReference::DeleteWee { wee_id } => {
                    let wee = get_wee_by_id(wee_id)
                        .await?
                        .ok_or(ServerFnError::new("Cannot find wee"))?;
                    ActiveDialog::Wee(crate::components::wees::ActiveDialog::Delete(wee)).pipe(Ok)
                }
                DialogReference::UpdateWeeUrge { wee_urge_id } => {
                    let wee_urge = get_wee_urge_by_id(wee_urge_id)
                        .await?
                        .ok_or(ServerFnError::new("Cannot find wee urgency"))?;
                    ActiveDialog::WeeUrge(crate::components::wee_urges::ActiveDialog::Change(
                        crate::components::wee_urges::Operation::Update { wee_urge },
                    ))
                    .pipe(Ok)
                }
                DialogReference::DeleteWeeUrge { wee_urge_id } => {
                    let wee_urge = get_wee_urge_by_id(wee_urge_id)
                        .await?
                        .ok_or(ServerFnError::new("Cannot find wee urgency"))?;
                    ActiveDialog::WeeUrge(crate::components::wee_urges::ActiveDialog::Delete(
                        wee_urge,
                    ))
                    .pipe(Ok)
                }
                DialogReference::UpdatePoo { poo_id } => {
                    let poo = get_poo_by_id(poo_id)
                        .await?
                        .ok_or(ServerFnError::new("Cannot find poo"))?;
                    ActiveDialog::Poo(crate::components::poos::ActiveDialog::Change(
                        crate::components::poos::Operation::Update { poo },
                    ))
                    .pipe(Ok)
                }
                DialogReference::DeletePoo { poo_id } => {
                    let poo = get_poo_by_id(poo_id)
                        .await?
                        .ok_or(ServerFnError::new("Cannot find poo"))?;
                    ActiveDialog::Poo(crate::components::poos::ActiveDialog::Delete(poo)).pipe(Ok)
                }
                DialogReference::UpdateBasic { consumption_id } => {
                    let consumption = get_consumption_by_id(consumption_id)
                        .await?
                        .ok_or(ServerFnError::new("Cannot find consumption"))?;
                    ActiveDialog::Consumption(
                        crate::components::consumptions::ActiveDialog::UpdateBasic(
                            crate::components::consumptions::Operation::Update { consumption },
                        ),
                    )
                    .pipe(Ok)
                }
                DialogReference::DeleteConsumption { consumption_id } => {
                    let consumption = get_consumption_by_id(consumption_id)
                        .await?
                        .ok_or(ServerFnError::new("Cannot find consumption"))?;
                    ActiveDialog::Consumption(
                        crate::components::consumptions::ActiveDialog::Delete(consumption),
                    )
                    .pipe(Ok)
                }
                DialogReference::UpdateIngredients { consumption_id } => {
                    let consumption = get_consumption_by_id(consumption_id)
                        .await?
                        .ok_or(ServerFnError::new("Cannot find consumption"))?;
                    ActiveDialog::Consumption(
                        crate::components::consumptions::ActiveDialog::UpdateIngredients(
                            consumption,
                        ),
                    )
                    .pipe(Ok)
                }
                DialogReference::IngredientUpdateBasic {
                    parent_id,
                    consumable_id,
                } => {
                    let parent = get_consumption_by_id(parent_id)
                        .await?
                        .ok_or(ServerFnError::new("Cannot find consumption"))?;
                    let consumable =
                        crate::functions::consumables::get_consumable_by_id(consumable_id)
                            .await?
                            .ok_or(ServerFnError::new("Cannot find consumable"))?;
                    ActiveDialog::Consumption(
                        crate::components::consumptions::ActiveDialog::NestedIngredient(
                            parent, consumable,
                        ),
                    )
                    .pipe(Ok)
                }
                DialogReference::IngredientUpdateIngredients {
                    parent_id,
                    consumable_id,
                } => {
                    let parent = get_consumption_by_id(parent_id)
                        .await?
                        .ok_or(ServerFnError::new("Cannot find consumption"))?;
                    let consumable =
                        crate::functions::consumables::get_consumable_by_id(consumable_id)
                            .await?
                            .ok_or(ServerFnError::new("Cannot find consumable"))?;
                    ActiveDialog::Consumption(
                        crate::components::consumptions::ActiveDialog::NestedIngredients(
                            parent, consumable,
                        ),
                    )
                    .pipe(Ok)
                }
                DialogReference::UpdateExercise { exercise_id } => {
                    let exercise = get_exercise_by_id(exercise_id)
                        .await?
                        .ok_or(ServerFnError::new("Cannot find exercise"))?;
                    ActiveDialog::Exercise(crate::components::exercises::ActiveDialog::Change(
                        crate::components::exercises::Operation::Update { exercise },
                    ))
                    .pipe(Ok)
                }
                DialogReference::DeleteExercise { exercise_id } => {
                    let exercise = get_exercise_by_id(exercise_id)
                        .await?
                        .ok_or(ServerFnError::new("Cannot find exercise"))?;
                    ActiveDialog::Exercise(crate::components::exercises::ActiveDialog::Delete(
                        exercise,
                    ))
                    .pipe(Ok)
                }
                DialogReference::UpdateReflux { reflux_id } => {
                    let reflux = get_reflux_by_id(reflux_id)
                        .await?
                        .ok_or(ServerFnError::new("Cannot find reflux"))?;
                    ActiveDialog::Reflux(crate::components::refluxs::ActiveDialog::Change(
                        crate::components::refluxs::Operation::Update { reflux },
                    ))
                    .pipe(Ok)
                }
                DialogReference::DeleteReflux { reflux_id } => {
                    let reflux = get_reflux_by_id(reflux_id)
                        .await?
                        .ok_or(ServerFnError::new("Cannot find reflux"))?;
                    ActiveDialog::Reflux(crate::components::refluxs::ActiveDialog::Delete(reflux))
                        .pipe(Ok)
                }
                _ => Ok(ActiveDialog::Idle),
            }
        });

    let on_edit = Callback::new(move |dialog_ref: DialogReference| {
        dialog.set(Some(dialog_ref));
    });

    let on_delete = Callback::new(move |dialog_ref: DialogReference| {
        dialog.set(Some(dialog_ref));
    });

    let on_close = Callback::new(move |_| {
        dialog.set(None);
    });

    rsx! {
        div { class: "ml-2 mr-2",
            div { class: "font-bold text-lg", "Incomplete Entries" }
            div { class: "mb-2 text-sm text-gray-600 dark:text-gray-400",
                "Incomplete entries grouped by date"
            }
        }

        match timeline.read().deref() {
            Some(Err(err)) => rsx! {
                div { class: "alert alert-error",
                    "Error loading incomplete entries: "
                    {err.to_string()}
                }
            },
            Some(Ok(timeline)) if timeline.is_empty() => rsx! {
                p { class: "alert alert-info", "No incomplete entries found." }
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
                            for (date, entries) in timeline.grouped_by_date() {
                                tr { class: "text-center font-bold bg-gray-700",
                                    td { colspan: 4,
                                        {display_date(date)}
                                    }
                                }
                                for entry in entries {
                                    IncompleteEntryRow {
                                        key: "{entry.get_id().as_str()}",
                                        entry: entry.clone(),
                                        selected,
                                        on_edit: on_edit,
                                        on_delete: on_delete,
                                    }
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

        match active_dialog.read().deref() {
            Some(Err(err)) => rsx! {
                div { class: "alert alert-error",
                    "Error loading dialog: "
                    {err.to_string()}
                }
            },
            Some(Ok(dialog)) => rsx! {
                TimelineDialog {
                    dialog: dialog.clone(),
                    on_change: move || { timeline.restart() },
                    replace_dialog: |_| {},
                    show_consumption_update_basic: move |consumption: crate::models::Consumption| {
                        on_edit.call(DialogReference::UpdateBasic {
                            consumption_id: consumption.id,
                        });
                    },
                    show_consumption_update_ingredients: move |consumption: crate::models::Consumption| {
                        on_edit.call(DialogReference::UpdateIngredients {
                            consumption_id: consumption.id,
                        });
                    },
                    show_consumption_ingredient_update_basic: move |(consumption, consumable): (crate::models::Consumption, crate::models::Consumable)| {
                        on_edit.call(DialogReference::IngredientUpdateBasic {
                            parent_id: consumption.id,
                            consumable_id: consumable.id,
                        });
                    },
                    show_consumption_ingredient_update_ingredients: move |(consumption, consumable): (crate::models::Consumption, crate::models::Consumable)| {
                        on_edit.call(DialogReference::IngredientUpdateIngredients {
                            parent_id: consumption.id,
                            consumable_id: consumable.id,
                        });
                    },
                    on_close,
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
