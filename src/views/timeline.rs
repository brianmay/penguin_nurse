use std::ops::Deref;

use chrono::{NaiveDate, Utc};
use dioxus::prelude::*;
use dioxus_fullstack::ServerFnError;
use dioxus_router::navigator;
use tap::Pipe;

use crate::{
    Route,
    components::{
        StrIcon,
        buttons::{ChangeButton, CreateButton, DeleteButton, NavButton},
        consumptions::{
            self, ConsumptionDetails, ConsumptionItemList, ConsumptionTypeIcon,
            consumption_duration,
        },
        events::EventTime,
        exercises::{ExerciseDetails, ExerciseTypeIcon},
        health_metrics::{HealthMetricDetails, HealthMetricIcon, health_metric_title},
        notes::{NoteDetails, note_icon, note_title},
        poos::{self, PooDetails, PooDuration, PooIcon, poo_title},
        refluxs::{RefluxDetails, reflux_duration, reflux_icon, reflux_title},
        symptoms::{SymptomDetails, symptom_icon, symptom_title},
        timeline::{ActiveDialog, DialogReference, TimelineDialog},
        wee_urges::{self, WeeUrgeDetails, WeeUrgeIcon, wee_urge_title},
        wees::{self, WeeDetails, WeeDuration, WeeIcon, wee_title},
    },
    dt::{display_date, get_date_for_dt, get_utc_times_for_date},
    functions::{
        consumables::get_consumable_by_id,
        consumptions::{get_consumption_by_id, get_consumptions_for_time_range},
        exercises::{get_exercise_by_id, get_exercises_for_time_range},
        health_metrics::{get_health_metric_by_id, get_health_metrics_for_time_range},
        notes::{get_note_by_id, get_notes_for_time_range},
        poos::{get_poo_by_id, get_poos_for_time_range},
        refluxs::{get_reflux_by_id, get_refluxs_for_time_range},
        symptoms::{get_symptom_by_id, get_symptoms_for_time_range},
        wee_urges::{get_wee_urge_by_id, get_wee_urges_for_time_range},
        wees::{get_wee_by_id, get_wees_for_time_range},
    },
    models::{Consumable, Consumption, Entry, EntryData, EntryId, Timeline},
    use_user,
};

#[component]
fn EntryRow(
    entry: ReadSignal<Entry>,
    date: ReadSignal<NaiveDate>,
    selected: Signal<Option<EntryId>>,
) -> Element {
    let navigator = navigator();
    let entry: Entry = entry();
    let id = entry.get_id();
    let update_dialog_reference = entry.get_update_dialog_reference();
    let delete_dialog_reference = entry.get_delete_dialog_reference();

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
                            consumption_duration { duration: consumption.consumption.duration }
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


                            WeeDuration { duration: exercise.duration }
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
        if let EntryData::Consumption(consumption) = &entry.data {
            {
                let total_nested_mls: f64 = consumption
                    .items
                    .iter()
                    .filter_map(|ci| ci.nested.liquid_mls)
                    .sum();
                match (consumption.consumption.liquid_mls, total_nested_mls) {
                    (
                        Some(consumption_mls),
                        total_nested_mls,
                    ) if (consumption_mls - total_nested_mls).abs() > f64::EPSILON => {
                        rsx! {
                            tr {
                                td { colspan: 4, class: "block sm:table-cell",
                                    div { class: "text-warning",
                                        "(Warning: Liquid ml total from ingredients "
                                        {format!("{}ml", total_nested_mls)}
                                        " does not match consumption liquid ml "
                                        {format!("{}ml", consumption_mls)}
                                        ")"
                                    }
                                }
                            }
                        }
                    }
                    (None, total_nested_mls) if total_nested_mls > 0.0 => {
                        rsx! {
                            tr {
                                td { colspan: 4, class: "block sm:table-cell",
                                    div { class: "text-warning",
                                        "(Warning: Liquid ml total from ingredients "
                                        {format!("{}ml", total_nested_mls)}
                                        " but consumption has no liquid ml set)"
                                    }
                                }
                            }
                        }
                    }
                    _ => rsx! {},
                }
            }
        }
        if selected() == Some(id) {
            td { colspan: 4, class: "block sm:table-cell",
                div { class: "flex flex-wrap gap-2",
                    ChangeButton {
                        on_click: move |_| {
                            navigator
                                .push(Route::TimelineList {
                                    date: date(),
                                    dialog: update_dialog_reference.clone(),
                                });
                        },
                        "Edit"
                    }
                    DeleteButton {
                        on_click: move |_| {
                            navigator
                                .push(Route::TimelineList {
                                    date: date(),
                                    dialog: delete_dialog_reference.clone(),
                                });
                        },
                        "Delete"
                    }
                    match entry.data {
                        EntryData::Consumption(consumption) => {
                            let consumption = consumption.consumption;
                            rsx! {
                                ChangeButton {
                                    on_click: move |_| {
                                        navigator
                                            .push(Route::TimelineList {
                                                date: date(),
                                                dialog: DialogReference::UpdateIngredients {
                                                    consumption_id: consumption.id,
                                                },
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
pub fn TimelineList(
    date: ReadSignal<NaiveDate>,
    dialog: ReadSignal<Option<DialogReference>>,
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
                let wee = get_wee_by_id(wee_id)
                    .await?
                    .ok_or(ServerFnError::new("Cannot find wee"))?;
                ActiveDialog::Wee(wees::ActiveDialog::Change(wees::Operation::Update { wee }))
                    .pipe(Ok)
            }
            DialogReference::DeleteWee { wee_id } => {
                let wee = get_wee_by_id(wee_id)
                    .await?
                    .ok_or(ServerFnError::new("Cannot find wee"))?;
                ActiveDialog::Wee(wees::ActiveDialog::Delete(wee)).pipe(Ok)
            }
            DialogReference::CreateWeeUrge { user_id } => ActiveDialog::WeeUrge(
                wee_urges::ActiveDialog::Change(wee_urges::Operation::Create { user_id }),
            )
            .pipe(Ok),
            DialogReference::UpdateWeeUrge { wee_urge_id } => {
                let wee_urge = get_wee_urge_by_id(wee_urge_id)
                    .await?
                    .ok_or(ServerFnError::new("Cannot find wee urgency"))?;
                ActiveDialog::WeeUrge(wee_urges::ActiveDialog::Change(
                    wee_urges::Operation::Update { wee_urge },
                ))
                .pipe(Ok)
            }
            DialogReference::DeleteWeeUrge { wee_urge_id } => {
                let wee_urge = get_wee_urge_by_id(wee_urge_id)
                    .await?
                    .ok_or(ServerFnError::new("Cannot find wee urgency"))?;
                ActiveDialog::WeeUrge(wee_urges::ActiveDialog::Delete(wee_urge)).pipe(Ok)
            }
            DialogReference::CreatePoo { user_id } => {
                ActiveDialog::Poo(poos::ActiveDialog::Change(poos::Operation::Create {
                    user_id,
                }))
                .pipe(Ok)
            }
            DialogReference::UpdatePoo { poo_id } => {
                let poo = get_poo_by_id(poo_id)
                    .await?
                    .ok_or(ServerFnError::new("Cannot find poo"))?;
                ActiveDialog::Poo(poos::ActiveDialog::Change(poos::Operation::Update { poo }))
                    .pipe(Ok)
            }
            DialogReference::DeletePoo { poo_id } => {
                let poo = get_poo_by_id(poo_id)
                    .await?
                    .ok_or(ServerFnError::new("Cannot find poo"))?;
                ActiveDialog::Poo(poos::ActiveDialog::Delete(poo)).pipe(Ok)
            }
            DialogReference::CreateConsumption { user_id } => {
                ActiveDialog::Consumption(consumptions::ActiveDialog::UpdateBasic(
                    consumptions::Operation::Create { user_id },
                ))
                .pipe(Ok)
            }
            DialogReference::UpdateBasic { consumption_id } => {
                let consumption = get_consumption_by_id(consumption_id)
                    .await?
                    .ok_or(ServerFnError::new("Cannot find consumption"))?;
                ActiveDialog::Consumption(consumptions::ActiveDialog::UpdateBasic(
                    consumptions::Operation::Update { consumption },
                ))
                .pipe(Ok)
            }
            DialogReference::UpdateIngredients { consumption_id } => {
                let consumption = get_consumption_by_id(consumption_id)
                    .await?
                    .ok_or(ServerFnError::new("Cannot find consumption"))?;
                ActiveDialog::Consumption(consumptions::ActiveDialog::UpdateIngredients(
                    consumption,
                ))
                .pipe(Ok)
            }
            DialogReference::IngredientUpdateBasic {
                parent_id,
                consumable_id,
            } => {
                let parent = get_consumption_by_id(parent_id)
                    .await?
                    .ok_or(ServerFnError::new("Cannot find consumption"))?;
                let consumable = get_consumable_by_id(consumable_id)
                    .await?
                    .ok_or(ServerFnError::new("Cannot find consumption"))?;
                ActiveDialog::Consumption(consumptions::ActiveDialog::NestedIngredient(
                    parent, consumable,
                ))
                .pipe(Ok)
            }
            DialogReference::IngredientUpdateIngredients {
                parent_id,
                consumable_id,
            } => {
                let parent = get_consumption_by_id(parent_id)
                    .await?
                    .ok_or(ServerFnError::new("Cannot find consumption"))?;
                let consumable = get_consumable_by_id(consumable_id)
                    .await?
                    .ok_or(ServerFnError::new("Cannot find consumption"))?;
                ActiveDialog::Consumption(consumptions::ActiveDialog::NestedIngredients(
                    parent, consumable,
                ))
                .pipe(Ok)
            }
            DialogReference::DeleteConsumption { consumption_id } => {
                let consumption = get_consumption_by_id(consumption_id)
                    .await?
                    .ok_or(ServerFnError::new("Cannot find consumption"))?;
                ActiveDialog::Consumption(consumptions::ActiveDialog::Delete(consumption)).pipe(Ok)
            }
            DialogReference::CreateExercise { user_id } => {
                ActiveDialog::Exercise(crate::components::exercises::ActiveDialog::Change(
                    crate::components::exercises::Operation::Create { user_id },
                ))
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
                ActiveDialog::Exercise(crate::components::exercises::ActiveDialog::Delete(exercise))
                    .pipe(Ok)
            }
            DialogReference::CreateHealthMetric { user_id } => {
                ActiveDialog::HealthMetric(crate::components::health_metrics::ActiveDialog::Change(
                    crate::components::health_metrics::Operation::Create { user_id },
                ))
                .pipe(Ok)
            }
            DialogReference::UpdateHealthMetric { health_metric_id } => {
                let health_metric = get_health_metric_by_id(health_metric_id)
                    .await?
                    .ok_or(ServerFnError::new("Cannot find health metric"))?;
                ActiveDialog::HealthMetric(crate::components::health_metrics::ActiveDialog::Change(
                    crate::components::health_metrics::Operation::Update { health_metric },
                ))
                .pipe(Ok)
            }
            DialogReference::DeleteHealthMetric { health_metric_id } => {
                let health_metric = get_health_metric_by_id(health_metric_id)
                    .await?
                    .ok_or(ServerFnError::new("Cannot find health metric"))?;
                ActiveDialog::HealthMetric(crate::components::health_metrics::ActiveDialog::Delete(
                    health_metric,
                ))
                .pipe(Ok)
            }
            DialogReference::CreateSymptom { user_id } => {
                ActiveDialog::Symptom(crate::components::symptoms::ActiveDialog::Change(
                    crate::components::symptoms::Operation::Create { user_id },
                ))
                .pipe(Ok)
            }
            DialogReference::UpdateSymptom { symptom_id } => {
                let symptom = get_symptom_by_id(symptom_id)
                    .await?
                    .ok_or(ServerFnError::new("Cannot find symptom"))?;
                ActiveDialog::Symptom(crate::components::symptoms::ActiveDialog::Change(
                    crate::components::symptoms::Operation::Update { symptom },
                ))
                .pipe(Ok)
            }
            DialogReference::DeleteSymptom { symptom_id } => {
                let symptom = get_symptom_by_id(symptom_id)
                    .await?
                    .ok_or(ServerFnError::new("Cannot find symptom"))?;
                ActiveDialog::Symptom(crate::components::symptoms::ActiveDialog::Delete(symptom))
                    .pipe(Ok)
            }
            DialogReference::CreateReflux { user_id } => {
                ActiveDialog::Reflux(crate::components::refluxs::ActiveDialog::Change(
                    crate::components::refluxs::Operation::Create { user_id },
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
            DialogReference::CreateNote { user_id } => {
                ActiveDialog::Note(crate::components::notes::ActiveDialog::Change(
                    crate::components::notes::Operation::Create { user_id },
                ))
                .pipe(Ok)
            }
            DialogReference::UpdateNote { note_id } => {
                let note = get_note_by_id(note_id)
                    .await?
                    .ok_or(ServerFnError::new("Cannot find note"))?;
                ActiveDialog::Note(crate::components::notes::ActiveDialog::Change(
                    crate::components::notes::Operation::Update { note },
                ))
                .pipe(Ok)
            }
            DialogReference::DeleteNote { note_id } => {
                let note = get_note_by_id(note_id)
                    .await?
                    .ok_or(ServerFnError::new("Cannot find note"))?;
                ActiveDialog::Note(crate::components::notes::ActiveDialog::Delete(note)).pipe(Ok)
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

            let wee_urgencies = get_wee_urges_for_time_range(user_id, start, end).await?;
            timeline.add_wee_urges(wee_urgencies);

            let poos = get_poos_for_time_range(user_id, start, end).await?;
            timeline.add_poos(poos);

            let consumptions = get_consumptions_for_time_range(user_id, start, end).await?;
            timeline.add_consumptions(consumptions);

            let exercises = get_exercises_for_time_range(user_id, start, end).await?;
            timeline.add_exercises(exercises);

            let health_metrics = get_health_metrics_for_time_range(user_id, start, end).await?;
            timeline.add_health_metrics(health_metrics);

            let symptoms = get_symptoms_for_time_range(user_id, start, end).await?;
            timeline.add_symptoms(symptoms);

            let refluxs = get_refluxs_for_time_range(user_id, start, end).await?;
            timeline.add_refluxs(refluxs);

            let notes = get_notes_for_time_range(user_id, start, end).await?;
            timeline.add_notes(notes);

            timeline.sort();

            Ok(timeline)
        });

    rsx! {
        div { class: "ml-2 mr-2",
            div { class: "font-bold text-lg", "Inputs" }
            div { class: "mb-2 flex flex-wrap gap-2",
                CreateButton {
                    on_click: move |_| {
                        navigator
                            .push(Route::TimelineList {
                                date: date(),
                                dialog: DialogReference::CreateConsumption {
                                    user_id,
                                },
                            });
                    },
                    "Consumption"
                }
                CreateButton {
                    on_click: move |_| {
                        navigator
                            .push(Route::TimelineList {
                                date: date(),
                                dialog: DialogReference::CreateExercise {
                                    user_id,
                                },
                            });
                    },
                    "Exercise"
                }
                CreateButton {
                    on_click: move |_| {
                        navigator
                            .push(Route::TimelineList {
                                date: date(),
                                dialog: DialogReference::CreateNote {
                                    user_id,
                                },
                            });
                    },
                    "Notes"
                }
            }
            div { class: "font-bold text-lg", "Outputs" }
            div { class: "mb-2 flex flex-wrap gap-2",
                CreateButton {
                    on_click: move |_| {
                        navigator
                            .push(Route::TimelineList {
                                date: date(),
                                dialog: DialogReference::CreateWeeUrge {
                                    user_id,
                                },
                            });
                    },
                    "Wee Urge"
                }
                CreateButton {
                    on_click: move |_| {
                        navigator
                            .push(Route::TimelineList {
                                date: date(),
                                dialog: DialogReference::CreateWee {
                                    user_id,
                                },
                            });
                    },
                    "Wee"
                }
                CreateButton {
                    on_click: move |_| {
                        navigator
                            .push(Route::TimelineList {
                                date: date(),
                                dialog: DialogReference::CreatePoo {
                                    user_id,
                                },
                            });
                    },
                    "Poo"
                }
                CreateButton {
                    on_click: move |_| {
                        navigator
                            .push(Route::TimelineList {
                                date: date(),
                                dialog: DialogReference::CreateHealthMetric {
                                    user_id,
                                },
                            });
                    },
                    "Health Metric"
                }
                CreateButton {
                    on_click: move |_| {
                        navigator
                            .push(Route::TimelineList {
                                date: date(),
                                dialog: DialogReference::CreateSymptom {
                                    user_id,
                                },
                            });
                    },
                    "Symptom"
                }
                CreateButton {
                    on_click: move |_| {
                        navigator
                            .push(Route::TimelineList {
                                date: date(),
                                dialog: DialogReference::CreateReflux {
                                    user_id,
                                },
                            });
                    },
                    "Reflux"
                }
            }

            div { class: "font-bold text-lg", {display_date(date())} }
            div { class: "mb-2 flex flex-wrap gap-2",
                NavButton {
                    on_click: move |_| {
                        let new_date = date().pred_opt();
                        if let Some(new_date) = new_date {
                            navigator
                                .push(Route::TimelineList {
                                    date: new_date,
                                    dialog: DialogReference::Idle,
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
                                dialog: DialogReference::Idle,
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
                                    dialog: DialogReference::Idle,
                                });
                        }
                    },
                    ">"
                }
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
                    on_change: move || { timeline.restart() },
                    replace_dialog: move |dialog| {
                        navigator
                            .replace(Route::TimelineList {
                                date: date(),
                                dialog,
                            });
                    },
                    show_consumption_update_basic: move |consumption: Consumption| {
                        navigator
                            .push(Route::TimelineList {
                                date: date(),
                                dialog: DialogReference::UpdateBasic {
                                    consumption_id: consumption.id,
                                },
                            });
                    },
                    show_consumption_update_ingredients: move |consumption: Consumption| {
                        navigator
                            .push(Route::TimelineList {
                                date: date(),
                                dialog: DialogReference::UpdateIngredients {
                                    consumption_id: consumption.id,
                                },
                            });
                    },
                    show_consumption_ingredient_update_basic: move |(consumption, consumable): (Consumption, Consumable)| {
                        navigator
                            .push(Route::TimelineList {
                                date: date(),
                                dialog: DialogReference::IngredientUpdateBasic {
                                    parent_id: consumption.id,
                                    consumable_id: consumable.id,
                                },
                            });
                    },
                    show_consumption_ingredient_update_ingredients: move |(consumption, consumable): (Consumption, Consumable)| {
                        navigator
                            .push(Route::TimelineList {
                                date: date(),
                                dialog: DialogReference::IngredientUpdateIngredients {
                                    parent_id: consumption.id,
                                    consumable_id: consumable.id,
                                },
                            });
                    },
                    on_close: move || {
                        navigator
                            .push(Route::TimelineList {
                                date: date(),
                                dialog: DialogReference::Idle,
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
    }
}
