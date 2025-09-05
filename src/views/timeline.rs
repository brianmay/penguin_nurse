use std::ops::Deref;

use chrono::{NaiveDate, Utc};
use dioxus::prelude::*;
use tap::Pipe;

use crate::{
    Route,
    components::{
        buttons::{ChangeButton, CreateButton, DeleteButton, NavButton},
        consumptions::{
            self, ConsumptionItemList, consumption_duration, consumption_icon, consumption_title,
        },
        events::{Markdown, event_colour, event_time, event_urgency},
        exercises::{exercise_calories, exercise_icon, exercise_rpe, exercise_title},
        health_metrics::{health_metric_icon, health_metric_title},
        notes::{note_icon, note_title},
        poos::{self, poo_bristol, poo_duration, poo_icon, poo_quantity, poo_title},
        refluxs::{reflux_duration, reflux_icon, reflux_title},
        symptoms::{SymptomDisplay, symptom_icon, symptom_title},
        timeline::{ActiveDialog, DialogReference, TimelineDialog},
        wee_urges::{self, wee_urge_icon, wee_urge_title},
        wees::{self, wee_duration, wee_icon, wee_mls, wee_title},
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
pub fn Icon(title: &'static str, icon: Element) -> Element {
    rsx! {
        div { class: "text-sm", {icon} }
        div { class: "text-sm", {title} }
    }
}

#[component]
fn EntryRow(
    entry: ReadOnlySignal<Entry>,
    date: ReadOnlySignal<NaiveDate>,
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
                event_time { time: entry.time }
            }
            match &entry.data {
                EntryData::Wee(wee) => {
                    rsx! {
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            Icon { title: wee_title(), icon: wee_icon() }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            wee_duration { duration: wee.duration }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            event_colour { colour: wee.colour }
                            div { class: "inline-block align-top",
                                div {
                                    wee_mls { mls: wee.mls }
                                }
                                div {
                                    event_urgency { urgency: wee.urgency }
                                }
                                if let Some(comments) = &wee.comments {
                                    Markdown { content: comments.to_string() }
                                }
                            }
                        }
                    }
                }
                EntryData::WeeUrge(wee_urge) => {
                    rsx! {
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            Icon { title: wee_urge_title(), icon: wee_urge_icon() }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2" }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            event_urgency { urgency: wee_urge.urgency }
                            if let Some(comments) = &wee_urge.comments {
                                Markdown { content: comments.to_string() }
                            }
                        }
                    }
                }
                EntryData::Poo(poo) => {
                    rsx! {
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            Icon { title: poo_title(), icon: poo_icon() }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            poo_duration { duration: poo.duration }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            event_colour { colour: poo.colour }
                            div { class: "inline-block align-top",
                                div {
                                    poo_bristol { bristol: poo.bristol }
                                }
                                div {
                                    poo_quantity { quantity: poo.quantity }
                                }
                                div {
                                    event_urgency { urgency: poo.urgency }
                                }
                                if let Some(comments) = &poo.comments {
                                    Markdown { content: comments.to_string() }
                                }
                            }
                        }
                    }
                }
                EntryData::Consumption(consumption) => {
                    rsx! {
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            Icon {
                                title: consumption_title(consumption.consumption.consumption_type),
                                icon: rsx! {
                                    consumption_icon { consumption_type: consumption.consumption.consumption_type }
                                },
                            }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            consumption_duration { duration: consumption.consumption.duration }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            {consumption.consumption.consumption_type.to_string()}
                            if let Some(liquid_mls) = &consumption.consumption.liquid_mls {
                                div {
                                    "Liquid: "
                                    {liquid_mls.to_string()}
                                    "ml"
                                }
                            }
                            if !consumption.items.is_empty() {
                                ConsumptionItemList { list: consumption.items.clone() }
                            }
                            if let Some(comments) = &consumption.consumption.comments {
                                div { "Comments:" }
                                Markdown { content: comments.to_string() }
                            }
                        }
                    }
                }
                EntryData::Exercise(exercise) => {
                    rsx! {
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            Icon {
                                title: exercise_title(exercise.exercise_type),
                                icon: rsx! {
                                    exercise_icon { exercise_type: exercise.exercise_type }
                                },
                            }
                        
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            wee_duration { duration: exercise.duration }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            {exercise.exercise_type.to_string()}
                            if let Some(location) = &exercise.location {
                                div {
                                    "Location: "
                                    {location.to_string()}
                                }
                            }
                            if let Some(distance) = &exercise.distance {
                                div {
                                    "Distance: "
                                    {distance.to_string()}
                                    "km"
                                }
                            }
                            if let Some(calories) = &exercise.calories {
                                div {
                                    "Calories: "
                                    exercise_calories { calories: Some(*calories) }
                                }
                            }
                            if let Some(rpe) = &exercise.rpe {
                                div {
                                    "RPE: "
                                    exercise_rpe { rpe: Some(*rpe) }
                                }
                            }
                            if let Some(comments) = &exercise.comments {
                                Markdown { content: comments.to_string() }
                            }
                        }
                    }
                }
                EntryData::HealthMetric(health_metric) => {
                    rsx! {
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            Icon { title: health_metric_title(), icon: health_metric_icon() }
                        
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2" }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            div { class: "inline-block align-top",
                                if let Some(pulse) = &health_metric.pulse {
                                    div {
                                        "Pulse: "
                                        {pulse.to_string()}
                                    }
                                }
                                if let Some(blood_glucose) = &health_metric.blood_glucose {
                                    div {
                                        "Blood Glucose: "
                                        {blood_glucose.to_string()}
                                    }
                                }
                                if let (Some(systolic_bp), Some(diastolic_bp)) = (
                                    &health_metric.systolic_bp,
                                    &health_metric.diastolic_bp,
                                )
                                {
                                    div {
                                        "Blood Pressure: "
                                        {systolic_bp.to_string()}
                                        "/"
                                        {diastolic_bp.to_string()}
                                    }
                                }
                                if let Some(weight) = &health_metric.weight {
                                    div {
                                        "Weight: "
                                        {weight.to_string()}
                                        "kg"
                                    }
                                }
                                if let Some(height) = &health_metric.height {
                                    div {
                                        "Height: "
                                        {height.to_string()}
                                        "cm"
                                    }
                                }
                                if let Some(comments) = &health_metric.comments {
                                    Markdown { content: comments.to_string() }
                                }
                            }
                        }
                    }
                }
                EntryData::Symptom(symptom) => {
                    rsx! {
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            Icon { title: symptom_title(), icon: symptom_icon() }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2" }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            div { class: "inline-block align-top",
                                SymptomDisplay { name: "Appetite Loss", intensity: symptom.appetite_loss }
                                SymptomDisplay { name: "Fever", intensity: symptom.fever }
                                SymptomDisplay { name: "Cough", intensity: symptom.cough }
                                SymptomDisplay { name: "Sore Throat", intensity: symptom.sore_throat }
                                SymptomDisplay { name: "Runny Nose", intensity: symptom.runny_nose }
                                SymptomDisplay { name: "Sneezing", intensity: symptom.sneezing }
                                SymptomDisplay { name: "Heart Burn", intensity: symptom.heart_burn }
                                SymptomDisplay {
                                    name: "Abdominal Pain",
                                    intensity: symptom.abdominal_pain,
                                    extra: if let Some(location) = &symptom.abdominal_pain_location { rsx! {
                                        div { class: "inline-block ml-2 align-top",
                                            "Location: "
                                            {location.to_string()}
                                        }
                                    }.pipe(Some) } else { None },
                                }
                                SymptomDisplay { name: "Diarrhea", intensity: symptom.diarrhea }
                                SymptomDisplay { name: "Constipation", intensity: symptom.constipation }
                                SymptomDisplay { name: "Lower Back Pain", intensity: symptom.lower_back_pain }
                                SymptomDisplay { name: "Upper Back Pain", intensity: symptom.upper_back_pain }
                                SymptomDisplay { name: "Neck Pain", intensity: symptom.neck_pain }
                                SymptomDisplay { name: "Joint Pain", intensity: symptom.joint_pain }
                                SymptomDisplay { name: "Headache", intensity: symptom.headache }
                                SymptomDisplay { name: "Nausea", intensity: symptom.nausea }
                                SymptomDisplay { name: "Dizziness", intensity: symptom.dizziness }
                                SymptomDisplay { name: "Stomach Pain", intensity: symptom.stomach_ache }
                                SymptomDisplay { name: "Chest Pain", intensity: symptom.chest_pain }
                                SymptomDisplay {
                                    name: "Shortness of Breath",
                                    intensity: symptom.shortness_of_breath,
                                }
                                SymptomDisplay { name: "Fatigue", intensity: symptom.fatigue }
                                SymptomDisplay { name: "Anxiety", intensity: symptom.anxiety }
                                SymptomDisplay { name: "Depression", intensity: symptom.depression }
                                SymptomDisplay { name: "Insomnia", intensity: symptom.insomnia }
                                if let Some(comments) = &symptom.comments {
                                    Markdown { content: comments.to_string() }
                                }
                            }
                        }
                    }
                }
                EntryData::Reflux(reflux) => {
                    rsx! {
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            Icon { title: reflux_title(), icon: reflux_icon() }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            reflux_duration { duration: reflux.duration }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            div { class: "inline-block align-top",
                                SymptomDisplay { name: "Severity", intensity: reflux.severity }
                                if let Some(location) = &reflux.location {
                                    div {
                                        "Location: "
                                        {location.to_string()}
                                    }
                                }
                                if let Some(comments) = &reflux.comments {
                                    Markdown { content: comments.to_string() }
                                }
                            }
                        }
                    }
                }
                EntryData::Note(note) => {
                    rsx! {
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            Icon { title: note_title(), icon: note_icon() }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2" }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            if let Some(comments) = &note.comments {
                                Markdown { content: comments.to_string() }
                            }
                        }
                    }
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
                let wee =
                    get_wee_by_id(wee_id)
                        .await?
                        .ok_or(ServerFnError::<String>::ServerError(
                            "Cannot find wee".to_string(),
                        ))?;
                ActiveDialog::Wee(wees::ActiveDialog::Change(wees::Operation::Update { wee }))
                    .pipe(Ok)
            }
            DialogReference::DeleteWee { wee_id } => {
                let wee =
                    get_wee_by_id(wee_id)
                        .await?
                        .ok_or(ServerFnError::<String>::ServerError(
                            "Cannot find wee".to_string(),
                        ))?;
                ActiveDialog::Wee(wees::ActiveDialog::Delete(wee)).pipe(Ok)
            }
            DialogReference::CreateWeeUrge { user_id } => ActiveDialog::WeeUrge(
                wee_urges::ActiveDialog::Change(wee_urges::Operation::Create { user_id }),
            )
            .pipe(Ok),
            DialogReference::UpdateWeeUrge { wee_urge_id } => {
                let wee_urge =
                    get_wee_urge_by_id(wee_urge_id).await?.ok_or(
                        ServerFnError::<String>::ServerError("Cannot find wee urgency".to_string()),
                    )?;
                ActiveDialog::WeeUrge(wee_urges::ActiveDialog::Change(
                    wee_urges::Operation::Update { wee_urge },
                ))
                .pipe(Ok)
            }
            DialogReference::DeleteWeeUrge { wee_urge_id } => {
                let wee_urge =
                    get_wee_urge_by_id(wee_urge_id).await?.ok_or(
                        ServerFnError::<String>::ServerError("Cannot find wee urgency".to_string()),
                    )?;
                ActiveDialog::WeeUrge(wee_urges::ActiveDialog::Delete(wee_urge)).pipe(Ok)
            }
            DialogReference::CreatePoo { user_id } => {
                ActiveDialog::Poo(poos::ActiveDialog::Change(poos::Operation::Create {
                    user_id,
                }))
                .pipe(Ok)
            }
            DialogReference::UpdatePoo { poo_id } => {
                let poo =
                    get_poo_by_id(poo_id)
                        .await?
                        .ok_or(ServerFnError::<String>::ServerError(
                            "Cannot find poo".to_string(),
                        ))?;
                ActiveDialog::Poo(poos::ActiveDialog::Change(poos::Operation::Update { poo }))
                    .pipe(Ok)
            }
            DialogReference::DeletePoo { poo_id } => {
                let poo =
                    get_poo_by_id(poo_id)
                        .await?
                        .ok_or(ServerFnError::<String>::ServerError(
                            "Cannot find poo".to_string(),
                        ))?;
                ActiveDialog::Poo(poos::ActiveDialog::Delete(poo)).pipe(Ok)
            }
            DialogReference::CreateConsumption { user_id } => {
                ActiveDialog::Consumption(consumptions::ActiveDialog::UpdateBasic(
                    consumptions::Operation::Create { user_id },
                ))
                .pipe(Ok)
            }
            DialogReference::UpdateBasic { consumption_id } => {
                let consumption = get_consumption_by_id(consumption_id).await?.ok_or(
                    ServerFnError::<String>::ServerError("Cannot find consumption".to_string()),
                )?;
                ActiveDialog::Consumption(consumptions::ActiveDialog::UpdateBasic(
                    consumptions::Operation::Update { consumption },
                ))
                .pipe(Ok)
            }
            DialogReference::UpdateIngredients { consumption_id } => {
                let consumption = get_consumption_by_id(consumption_id).await?.ok_or(
                    ServerFnError::<String>::ServerError("Cannot find consumption".to_string()),
                )?;
                ActiveDialog::Consumption(consumptions::ActiveDialog::UpdateIngredients(
                    consumption,
                ))
                .pipe(Ok)
            }
            DialogReference::IngredientUpdateBasic {
                parent_id,
                consumable_id,
            } => {
                let parent =
                    get_consumption_by_id(parent_id).await?.ok_or(
                        ServerFnError::<String>::ServerError("Cannot find consumption".to_string()),
                    )?;
                let consumable = get_consumable_by_id(consumable_id).await?.ok_or(
                    ServerFnError::<String>::ServerError("Cannot find consumption".to_string()),
                )?;
                ActiveDialog::Consumption(consumptions::ActiveDialog::NestedIngredient(
                    parent, consumable,
                ))
                .pipe(Ok)
            }
            DialogReference::IngredientUpdateIngredients {
                parent_id,
                consumable_id,
            } => {
                let parent =
                    get_consumption_by_id(parent_id).await?.ok_or(
                        ServerFnError::<String>::ServerError("Cannot find consumption".to_string()),
                    )?;
                let consumable = get_consumable_by_id(consumable_id).await?.ok_or(
                    ServerFnError::<String>::ServerError("Cannot find consumption".to_string()),
                )?;
                ActiveDialog::Consumption(consumptions::ActiveDialog::NestedIngredients(
                    parent, consumable,
                ))
                .pipe(Ok)
            }
            DialogReference::DeleteConsumption { consumption_id } => {
                let consumption = get_consumption_by_id(consumption_id).await?.ok_or(
                    ServerFnError::<String>::ServerError("Cannot find consumption".to_string()),
                )?;
                ActiveDialog::Consumption(consumptions::ActiveDialog::Delete(consumption)).pipe(Ok)
            }
            DialogReference::CreateExercise { user_id } => {
                ActiveDialog::Exercise(crate::components::exercises::ActiveDialog::Change(
                    crate::components::exercises::Operation::Create { user_id },
                ))
                .pipe(Ok)
            }
            DialogReference::UpdateExercise { exercise_id } => {
                let exercise =
                    get_exercise_by_id(exercise_id).await?.ok_or(
                        ServerFnError::<String>::ServerError("Cannot find exercise".to_string()),
                    )?;
                ActiveDialog::Exercise(crate::components::exercises::ActiveDialog::Change(
                    crate::components::exercises::Operation::Update { exercise },
                ))
                .pipe(Ok)
            }
            DialogReference::DeleteExercise { exercise_id } => {
                let exercise =
                    get_exercise_by_id(exercise_id).await?.ok_or(
                        ServerFnError::<String>::ServerError("Cannot find exercise".to_string()),
                    )?;
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
                let health_metric =
                    get_health_metric_by_id(health_metric_id)
                        .await?
                        .ok_or(ServerFnError::<String>::ServerError(
                            "Cannot find health metric".to_string(),
                        ))?;
                ActiveDialog::HealthMetric(crate::components::health_metrics::ActiveDialog::Change(
                    crate::components::health_metrics::Operation::Update { health_metric },
                ))
                .pipe(Ok)
            }
            DialogReference::DeleteHealthMetric { health_metric_id } => {
                let health_metric =
                    get_health_metric_by_id(health_metric_id)
                        .await?
                        .ok_or(ServerFnError::<String>::ServerError(
                            "Cannot find health metric".to_string(),
                        ))?;
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
                let symptom = get_symptom_by_id(symptom_id).await?.ok_or(
                    ServerFnError::<String>::ServerError("Cannot find symptom".to_string()),
                )?;
                ActiveDialog::Symptom(crate::components::symptoms::ActiveDialog::Change(
                    crate::components::symptoms::Operation::Update { symptom },
                ))
                .pipe(Ok)
            }
            DialogReference::DeleteSymptom { symptom_id } => {
                let symptom = get_symptom_by_id(symptom_id).await?.ok_or(
                    ServerFnError::<String>::ServerError("Cannot find symptom".to_string()),
                )?;
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
                let reflux = get_reflux_by_id(reflux_id).await?.ok_or(
                    ServerFnError::<String>::ServerError("Cannot find reflux".to_string()),
                )?;
                ActiveDialog::Reflux(crate::components::refluxs::ActiveDialog::Change(
                    crate::components::refluxs::Operation::Update { reflux },
                ))
                .pipe(Ok)
            }
            DialogReference::DeleteReflux { reflux_id } => {
                let reflux = get_reflux_by_id(reflux_id).await?.ok_or(
                    ServerFnError::<String>::ServerError("Cannot find reflux".to_string()),
                )?;
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
                let note =
                    get_note_by_id(note_id)
                        .await?
                        .ok_or(ServerFnError::<String>::ServerError(
                            "Cannot find note".to_string(),
                        ))?;
                ActiveDialog::Note(crate::components::notes::ActiveDialog::Change(
                    crate::components::notes::Operation::Update { note },
                ))
                .pipe(Ok)
            }
            DialogReference::DeleteNote { note_id } => {
                let note =
                    get_note_by_id(note_id)
                        .await?
                        .ok_or(ServerFnError::<String>::ServerError(
                            "Cannot find note".to_string(),
                        ))?;
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
