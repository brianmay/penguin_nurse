use std::{num::ParseIntError, str::FromStr};

use dioxus::prelude::*;
use dioxus_router::ToQueryArgument;
use tap::Pipe;
use thiserror::Error;

use crate::{
    components::{consumptions::ConsumptionDialog, poos::PooDialog, wees::WeeDialog},
    models::{
        Consumable, ConsumableId, Consumption, ConsumptionId, Exercise, ExerciseId, HealthMetric,
        HealthMetricId, Note, NoteId, Poo, PooId, Reflux, RefluxId, Symptom, SymptomId, UserId,
        Wee, WeeId, WeeUrge, WeeUrgeId,
    },
};

use super::{
    consumptions, exercises, health_metrics, notes, poos, refluxs, symptoms, wee_urges, wees,
};

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveDialog {
    Wee(wees::ActiveDialog),
    WeeUrge(wee_urges::ActiveDialog),
    Poo(poos::ActiveDialog),
    Consumption(consumptions::ActiveDialog),
    Exercise(exercises::ActiveDialog),
    HealthMetric(health_metrics::ActiveDialog),
    Symptom(symptoms::ActiveDialog),
    Reflux(refluxs::ActiveDialog),
    Note(notes::ActiveDialog),
    Idle,
}

#[derive(Error, Debug)]
pub enum DialogReferenceError {
    #[error("Invalid integer")]
    ParseIntError(#[from] ParseIntError),

    #[error("Invalid reference")]
    ReferenceError,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum DialogReference {
    CreateWee {
        user_id: UserId,
    },
    UpdateWee {
        wee_id: WeeId,
    },
    DeleteWee {
        wee_id: WeeId,
    },
    CreateWeeUrge {
        user_id: UserId,
    },
    UpdateWeeUrge {
        wee_urge_id: WeeUrgeId,
    },
    DeleteWeeUrge {
        wee_urge_id: WeeUrgeId,
    },
    CreatePoo {
        user_id: UserId,
    },
    UpdatePoo {
        poo_id: PooId,
    },
    DeletePoo {
        poo_id: PooId,
    },
    CreateConsumption {
        user_id: UserId,
    },
    UpdateBasic {
        consumption_id: ConsumptionId,
    },
    UpdateIngredients {
        consumption_id: ConsumptionId,
    },
    IngredientUpdateBasic {
        parent_id: ConsumptionId,
        consumable_id: ConsumableId,
    },
    IngredientUpdateIngredients {
        parent_id: ConsumptionId,
        consumable_id: ConsumableId,
    },
    DeleteConsumption {
        consumption_id: ConsumptionId,
    },
    CreateExercise {
        user_id: UserId,
    },
    UpdateExercise {
        exercise_id: ExerciseId,
    },
    DeleteExercise {
        exercise_id: ExerciseId,
    },
    CreateHealthMetric {
        user_id: UserId,
    },
    UpdateHealthMetric {
        health_metric_id: HealthMetricId,
    },
    DeleteHealthMetric {
        health_metric_id: HealthMetricId,
    },
    CreateSymptom {
        user_id: UserId,
    },
    UpdateSymptom {
        symptom_id: SymptomId,
    },
    DeleteSymptom {
        symptom_id: SymptomId,
    },
    CreateReflux {
        user_id: UserId,
    },
    UpdateReflux {
        reflux_id: RefluxId,
    },
    DeleteReflux {
        reflux_id: RefluxId,
    },
    CreateNote {
        user_id: UserId,
    },
    UpdateNote {
        note_id: NoteId,
    },
    DeleteNote {
        note_id: NoteId,
    },
    #[default]
    Idle,
}

impl ToQueryArgument for DialogReference {
    fn display_query_argument(
        &self,
        query_name: &str,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}={}", query_name, self.to_string())
    }
}

impl FromStr for DialogReference {
    type Err = DialogReferenceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split("-").collect::<Vec<_>>();
        match split[..] {
            ["wee", "create", id] => {
                let user_id = UserId::new(id.parse()?);
                Self::CreateWee { user_id }
            }
            ["wee", "update", id] => {
                let wee_id = WeeId::new(id.parse()?);
                Self::UpdateWee { wee_id }
            }
            ["wee", "delete", id] => {
                let wee_id = WeeId::new(id.parse()?);
                Self::DeleteWee { wee_id }
            }
            ["wee_urge", "create", id] => {
                let user_id = UserId::new(id.parse()?);
                Self::CreateWeeUrge { user_id }
            }
            ["wee_urge", "update", id] => {
                let wee_urge_id = WeeUrgeId::new(id.parse()?);
                Self::UpdateWeeUrge { wee_urge_id }
            }
            ["wee_urge", "delete", id] => {
                let wee_urge_id = WeeUrgeId::new(id.parse()?);
                Self::DeleteWeeUrge { wee_urge_id }
            }
            ["poo", "create", id] => {
                let user_id = UserId::new(id.parse()?);
                Self::CreatePoo { user_id }
            }
            ["poo", "update", poo_id] => {
                let poo_id = PooId::new(poo_id.parse()?);
                Self::UpdatePoo { poo_id }
            }
            ["poo", "delete", id] => {
                let poo_id = PooId::new(id.parse()?);
                Self::DeletePoo { poo_id }
            }
            ["consumption", "create", id] => {
                let user_id = UserId::new(id.parse()?);
                Self::CreateConsumption { user_id }
            }
            ["consumption", "update", id] => {
                let consumption_id = ConsumptionId::new(id.parse()?);
                Self::UpdateBasic { consumption_id }
            }
            ["consumption_ingredients", "update", id] => {
                let consumption_id = ConsumptionId::new(id.parse()?);
                Self::UpdateIngredients { consumption_id }
            }
            [
                "consumption_ingredients",
                "nested_ingredient",
                parent_id,
                id,
            ] => {
                let parent_id = ConsumptionId::new(parent_id.parse()?);
                let consumable_id = ConsumableId::new(id.parse()?);
                Self::IngredientUpdateBasic {
                    parent_id,
                    consumable_id,
                }
            }
            [
                "consumption_ingredients",
                "nested_ingredients",
                parent_id,
                id,
            ] => {
                let parent_id = ConsumptionId::new(parent_id.parse()?);
                let consumable_id = ConsumableId::new(id.parse()?);
                Self::IngredientUpdateIngredients {
                    parent_id,
                    consumable_id,
                }
            }
            ["consumption", "delete", id] => {
                let consumption_id = ConsumptionId::new(id.parse()?);
                Self::DeleteConsumption { consumption_id }
            }
            ["exercise", "create", id] => {
                let user_id = UserId::new(id.parse()?);
                Self::CreateExercise { user_id }
            }
            ["exercise", "update", id] => {
                let exercise_id = ExerciseId::new(id.parse()?);
                Self::UpdateExercise { exercise_id }
            }
            ["exercise", "delete", id] => {
                let exercise_id = ExerciseId::new(id.parse()?);
                Self::DeleteExercise { exercise_id }
            }
            ["health_metric", "create", id] => {
                let user_id = UserId::new(id.parse()?);
                Self::CreateHealthMetric { user_id }
            }
            ["health_metric", "update", id] => {
                let health_metric_id = HealthMetricId::new(id.parse()?);
                Self::UpdateHealthMetric { health_metric_id }
            }
            ["health_metric", "delete", id] => {
                let health_metric_id = HealthMetricId::new(id.parse()?);
                Self::DeleteHealthMetric { health_metric_id }
            }
            ["symptom", "create", id] => {
                let user_id = UserId::new(id.parse()?);
                Self::CreateSymptom { user_id }
            }
            ["symptom", "update", id] => {
                let symptom_id = SymptomId::new(id.parse()?);
                Self::UpdateSymptom { symptom_id }
            }
            ["symptom", "delete", id] => {
                let symptom_id = SymptomId::new(id.parse()?);
                Self::DeleteSymptom { symptom_id }
            }
            ["reflux", "create", id] => {
                let user_id = UserId::new(id.parse()?);
                Self::CreateReflux { user_id }
            }
            ["reflux", "update", id] => {
                let reflux_id = RefluxId::new(id.parse()?);
                Self::UpdateReflux { reflux_id }
            }
            ["reflux", "delete", id] => {
                let reflux_id = RefluxId::new(id.parse()?);
                Self::DeleteReflux { reflux_id }
            }
            ["note", "create", id] => {
                let user_id = UserId::new(id.parse()?);
                Self::CreateNote { user_id }
            }
            ["note", "update", id] => {
                let note_id = id.parse()?;
                Self::UpdateNote { note_id }
            }
            ["note", "delete", id] => {
                let note_id = id.parse()?;
                Self::DeleteNote { note_id }
            }
            [""] | [] => Self::Idle,
            _ => return Err(DialogReferenceError::ReferenceError),
        }
        .pipe(Ok)
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for DialogReference {
    fn to_string(&self) -> String {
        match self {
            DialogReference::CreateWee { user_id } => format!("wee-create-{user_id}"),
            DialogReference::UpdateWee { wee_id } => format!("wee-update-{wee_id}"),
            DialogReference::DeleteWee { wee_id } => format!("wee-delete-{wee_id}"),
            DialogReference::CreateWeeUrge { user_id } => {
                format!("wee_urge-create-{user_id}")
            }
            DialogReference::UpdateWeeUrge { wee_urge_id } => {
                format!("wee_urge-update-{wee_urge_id}")
            }
            DialogReference::DeleteWeeUrge { wee_urge_id } => {
                format!("wee_urge-delete-{wee_urge_id}")
            }
            DialogReference::CreatePoo { user_id } => format!("poo-create-{user_id}"),
            DialogReference::UpdatePoo { poo_id } => format!("poo-update-{poo_id}"),
            DialogReference::DeletePoo { poo_id } => format!("poo-delete-{poo_id}"),
            DialogReference::CreateConsumption { user_id } => {
                format!("consumption-create-{user_id}")
            }
            DialogReference::UpdateBasic { consumption_id } => {
                format!("consumption-update-{consumption_id}")
            }
            DialogReference::UpdateIngredients { consumption_id } => {
                format!("consumption_ingredients-update-{consumption_id}")
            }
            DialogReference::IngredientUpdateBasic {
                parent_id,
                consumable_id,
            } => {
                format!("consumption_ingredients-nested_ingredient-{parent_id}-{consumable_id}")
            }
            DialogReference::IngredientUpdateIngredients {
                parent_id,
                consumable_id,
            } => {
                format!("consumption_ingredients-nested_ingredients-{parent_id}-{consumable_id}")
            }
            DialogReference::DeleteConsumption { consumption_id } => {
                format!("consumption-delete-{consumption_id}")
            }
            DialogReference::CreateExercise { user_id } => format!("exercise-create-{user_id}"),
            DialogReference::UpdateExercise { exercise_id } => {
                format!("exercise-update-{exercise_id}")
            }
            DialogReference::DeleteExercise { exercise_id } => {
                format!("exercise-delete-{exercise_id}")
            }
            DialogReference::CreateHealthMetric { user_id } => {
                format!("health_metric-create-{user_id}")
            }
            DialogReference::UpdateHealthMetric { health_metric_id } => {
                format!("health_metric-update-{health_metric_id}")
            }
            DialogReference::DeleteHealthMetric { health_metric_id } => {
                format!("health_metric-delete-{health_metric_id}")
            }
            DialogReference::CreateSymptom { user_id } => format!("symptom-create-{user_id}"),
            DialogReference::UpdateSymptom { symptom_id } => {
                format!("symptom-update-{symptom_id}")
            }
            DialogReference::DeleteSymptom { symptom_id } => {
                format!("symptom-delete-{symptom_id}")
            }
            DialogReference::CreateReflux { user_id } => format!("reflux-create-{user_id}"),
            DialogReference::UpdateReflux { reflux_id } => {
                format!("reflux-update-{reflux_id}")
            }
            DialogReference::DeleteReflux { reflux_id } => {
                format!("reflux-delete-{reflux_id}")
            }
            DialogReference::CreateNote { user_id } => format!("note-create-{user_id}"),
            DialogReference::UpdateNote { note_id } => format!("note-update-{note_id}"),
            DialogReference::DeleteNote { note_id } => format!("note-delete-{note_id}"),
            DialogReference::Idle => String::new(),
        }
    }
}

#[component]
pub fn TimelineDialog(
    dialog: ReadSignal<ActiveDialog>,
    on_change: Callback<()>,
    on_close: Callback<()>,
    replace_dialog: Callback<DialogReference>,
    show_consumption_update_basic: Callback<Consumption>,
    show_consumption_update_ingredients: Callback<Consumption>,
    show_consumption_ingredient_update_basic: Callback<(Consumption, Consumable)>,
    show_consumption_ingredient_update_ingredients: Callback<(Consumption, Consumable)>,
) -> Element {
    match dialog() {
        ActiveDialog::Wee(wee_dialog) => {
            rsx! {
                WeeDialog {
                    dialog: wee_dialog,
                    on_close,
                    on_change: move |wee: Wee| {
                        replace_dialog(DialogReference::UpdateWee {
                            wee_id: wee.id,
                        });
                        on_change(());
                        on_close(());
                    },
                    on_delete: move |_wee| {
                        on_change(());
                        on_close(());
                    },
                }
            }
        }
        ActiveDialog::WeeUrge(wee_urge_dialog) => {
            rsx! {
                wee_urges::WeeUrgeDialog {
                    dialog: wee_urge_dialog,
                    on_close,
                    on_change: move |wee_urge: WeeUrge| {
                        replace_dialog(DialogReference::UpdateWeeUrge {
                            wee_urge_id: wee_urge.id,
                        });
                        on_change(());
                        on_close(());
                    },
                    on_delete: move |_wee_urge| {
                        on_change(());
                        on_close(());
                    },
                }
            }
        }
        ActiveDialog::Poo(poo_dialog) => {
            rsx! {
                PooDialog {
                    dialog: poo_dialog,
                    on_close,
                    on_change: move |poo: Poo| {
                        replace_dialog(DialogReference::UpdatePoo {
                            poo_id: poo.id,
                        });
                        on_change(());
                        on_close(());
                    },
                    on_delete: move |_poo| {
                        on_change(());
                        on_close(());
                    },
                }
            }
        }
        ActiveDialog::Consumption(consumption_dialog) => {
            rsx! {
                ConsumptionDialog {
                    dialog: consumption_dialog,
                    show_update_basic: show_consumption_update_basic,
                    show_update_ingredients: show_consumption_update_ingredients,
                    show_ingredient_update_basic: show_consumption_ingredient_update_basic,
                    show_ingredient_update_ingredients: show_consumption_ingredient_update_ingredients,
                    on_change: move |consumption: Consumption| {
                        replace_dialog(DialogReference::UpdateBasic {
                            consumption_id: consumption.id,
                        });
                        on_change(());
                    },
                    on_change_ingredients: move |_consumption| {
                        on_change(());
                    },
                    on_delete: move |_consumption| {
                        on_change(());
                    },
                    on_close,
                }
            }
        }
        ActiveDialog::Exercise(exercise_dialog) => {
            rsx! {
                exercises::ExerciseDialog {
                    dialog: exercise_dialog,
                    on_close,
                    on_change: move |exercise: Exercise| {
                        replace_dialog(DialogReference::UpdateExercise {
                            exercise_id: exercise.id,
                        });
                        on_change(());
                        on_close(());
                    },
                    on_delete: move |_exercise| {
                        on_change(());
                        on_close(());
                    },
                }
            }
        }
        ActiveDialog::HealthMetric(health_metric_dialog) => {
            rsx! {
                health_metrics::HealthMetricDialog {
                    dialog: health_metric_dialog,
                    on_close,
                    on_change: move |health_metric: HealthMetric| {
                        replace_dialog(DialogReference::UpdateHealthMetric {
                            health_metric_id: health_metric.id,
                        });
                        on_change(());
                        on_close(());
                    },
                    on_delete: move |_health_metric| {
                        on_change(());
                        on_close(());
                    },
                }
            }
        }
        ActiveDialog::Symptom(symptom_dialog) => {
            rsx! {
                symptoms::SymptomDialog {
                    dialog: symptom_dialog,
                    on_close,
                    on_change: move |symptom: Symptom| {
                        replace_dialog(DialogReference::UpdateSymptom {
                            symptom_id: symptom.id,
                        });
                        on_change(());
                        on_close(());
                    },
                    on_delete: move |_symptom| {
                        on_change(());
                        on_close(());
                    },
                }
            }
        }
        ActiveDialog::Reflux(reflux_dialog) => {
            rsx! {
                refluxs::RefluxDialog {
                    dialog: reflux_dialog,
                    on_close,
                    on_change: move |reflux: Reflux| {
                        replace_dialog(DialogReference::UpdateReflux {
                            reflux_id: reflux.id,
                        });
                        on_change(());
                        on_close(());
                    },
                    on_delete: move |_reflux| {
                        on_change(());
                        on_close(());
                    },
                }
            }
        }
        ActiveDialog::Note(note_dialog) => {
            rsx! {
                notes::NoteDialog {
                    dialog: note_dialog,
                    on_close,
                    on_change: move |note: Note| {
                        replace_dialog(DialogReference::UpdateNote {
                            note_id: note.id,
                        });
                        on_change(());
                        on_close(());
                    },
                    on_delete: move |_note| {
                        on_change(());
                        on_close(());
                    },
                }
            }
        }
        ActiveDialog::Idle => {
            rsx! {}
        }
    }
}
