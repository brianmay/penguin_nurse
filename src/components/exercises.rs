use chrono::{DateTime, FixedOffset, Local, TimeDelta, Utc};
use dioxus::prelude::*;

use crate::{
    components::{
        events::{Markdown, event_date_time_short},
        times::time_delta_to_string,
    },
    forms::{
        Dialog, EditError, FieldValue, FormSaveCancelButton, InputDateTime, InputDuration,
        InputExerciseCalories, InputExerciseRpe, InputExerciseType, InputNumber, InputTextArea,
        Saving, ValidationError, validate_comments, validate_distance, validate_duration,
        validate_exercise_calories, validate_exercise_rpe, validate_exercise_type,
        validate_fixed_offset_date_time, validate_location,
    },
    functions::exercises::{create_exercise, delete_exercise, update_exercise},
    models::{ChangeExercise, Exercise, ExerciseType, MaybeSet, NewExercise, UserId},
};
use classes::classes;

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Create { user_id: UserId },
    Update { exercise: Exercise },
}

#[derive(Debug, Clone)]
struct Validate {
    time: Memo<Result<DateTime<FixedOffset>, ValidationError>>,
    duration: Memo<Result<TimeDelta, ValidationError>>,
    location: Memo<Result<Option<String>, ValidationError>>,
    distance: Memo<Result<Option<bigdecimal::BigDecimal>, ValidationError>>,
    calories: Memo<Result<Option<i32>, ValidationError>>,
    rpe: Memo<Result<Option<i32>, ValidationError>>,
    exercise_type: Memo<Result<ExerciseType, ValidationError>>,
    comments: Memo<Result<Option<String>, ValidationError>>,
}

async fn do_save(op: &Operation, validate: &Validate) -> Result<Exercise, EditError> {
    let time = validate.time.read().clone()?;
    let duration = validate.duration.read().clone()?;
    let exercise_type = validate.exercise_type.read().clone()?;
    let location = validate.location.read().clone()?;
    let distance = validate.distance.read().clone()?;
    let calories = validate.calories.read().clone()?;
    let rpe = validate.rpe.read().clone()?;
    let comments = validate.comments.read().clone()?;

    match op {
        Operation::Create { user_id } => {
            let updates = NewExercise {
                user_id: *user_id,
                time,
                duration,
                location,
                distance,
                calories,
                rpe,
                comments,
                exercise_type,
            };
            create_exercise(updates).await.map_err(EditError::Server)
        }
        Operation::Update { exercise } => {
            let changes = ChangeExercise {
                user_id: MaybeSet::NoChange,
                time: MaybeSet::Set(time),
                duration: MaybeSet::Set(duration),
                exercise_type: MaybeSet::Set(exercise_type),
                location: MaybeSet::Set(location),
                distance: MaybeSet::Set(distance),
                calories: MaybeSet::Set(calories),
                rpe: MaybeSet::Set(rpe),
                comments: MaybeSet::Set(comments),
            };
            update_exercise(exercise.id, changes)
                .await
                .map_err(EditError::Server)
        }
    }
}

#[component]
pub fn ExerciseUpdate(op: Operation, on_cancel: Callback, on_save: Callback<Exercise>) -> Element {
    let time = use_signal(|| match &op {
        Operation::Create { .. } => Utc::now().with_timezone(&Local).fixed_offset().as_string(),
        Operation::Update { exercise } => exercise.time.as_string(),
    });

    let duration = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { exercise } => exercise.duration.as_string(),
    });

    let exercise_type = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { exercise } => exercise.exercise_type.as_string(),
    });

    let location = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { exercise } => exercise.location.as_string(),
    });

    let distance = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { exercise } => exercise.distance.as_string(),
    });

    let calories: Signal<String> = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { exercise } => exercise.calories.as_string(),
    });

    let rpe = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { exercise } => exercise.rpe.as_string(),
    });

    let comments = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { exercise } => exercise.comments.as_string(),
    });

    let validate = Validate {
        time: use_memo(move || validate_fixed_offset_date_time(&time())),
        duration: use_memo(move || validate_duration(&duration())),
        location: use_memo(move || validate_location(&location())),
        distance: use_memo(move || validate_distance(&distance())),
        calories: use_memo(move || validate_exercise_calories(&calories())),
        rpe: use_memo(move || validate_exercise_rpe(&rpe())),
        exercise_type: use_memo(move || validate_exercise_type(&exercise_type())),
        comments: use_memo(move || validate_comments(&comments())),
    };

    let mut saving = use_signal(|| Saving::No);

    // disable form while waiting for response
    let disabled = use_memo(move || saving.read().is_saving());
    let disabled_save = use_memo(move || {
        validate.time.read().is_err()
            || validate.duration.read().is_err()
            || validate.location.read().is_err()
            || validate.distance.read().is_err()
            || validate.calories.read().is_err()
            || validate.rpe.read().is_err()
            || validate.exercise_type.read().is_err()
            || validate.comments.read().is_err()
            || disabled()
    });

    let op_clone = op.clone();
    let validate_clone = validate.clone();
    let on_save = use_callback(move |()| {
        let op = op_clone.clone();
        let validate = validate_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            let result = do_save(&op, &validate).await;

            match result {
                Ok(consumable) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_save(consumable);
                }
                Err(err) => saving.set(Saving::Finished(Err(err))),
            }
        });
    });

    rsx! {
        h3 { class: "text-lg font-bold",
            match &op {
                Operation::Create { .. } => "Create Exercise".to_string(),
                Operation::Update { exercise } => format!("Edit Exercise {}", exercise.name()),
            }
        }
        p { class: "py-4", "Press ESC key or click the button below to close" }
        form {
            novalidate: true,
            action: "javascript:void(0)",
            method: "dialog",
            onkeyup: move |event| {
                if event.key() == Key::Escape {
                    on_cancel(());
                }
            },
            InputDateTime {
                id: "time",
                label: "Time",
                value: time,
                validate: validate.time,
                disabled,
            }
            InputDuration {
                id: "duration",
                label: "Duration",
                value: duration,
                start_time: validate.time,
                validate: validate.duration,
                disabled,
            }
            InputTextArea {
                id: "location",
                label: "Location",
                value: location,
                validate: validate.location,
                disabled,
            }
            InputNumber {
                id: "distance",
                label: "Distance (km)",
                value: distance,
                validate: validate.distance,
                disabled,
            }
            InputExerciseCalories {
                id: "calories",
                label: "Calories (0-10000)",
                value: calories,
                validate: validate.calories,
                disabled,
            }
            InputExerciseRpe {
                id: "rpe",
                label: "RPE (1-10)",
                value: rpe,
                validate: validate.rpe,
                disabled,
            }
            InputExerciseType {
                id: "exercise_type",
                label: "Type",
                value: exercise_type,
                validate: validate.exercise_type,
                disabled,
            }
            InputTextArea {
                id: "comments",
                label: "Comments",
                value: comments,
                validate: validate.comments,
                disabled,
            }
            FormSaveCancelButton {
                disabled: disabled_save,
                on_save: move |()| on_save(()),
                on_cancel: move |_| on_cancel(()),
                title: match &op {
                    Operation::Create { .. } => "Create",
                    Operation::Update { .. } => "Save",
                },
                saving,
            }
        }
    }
}

#[component]
pub fn ExerciseDelete(
    exercise: Exercise,
    on_cancel: Callback,
    on_delete: Callback<Exercise>,
) -> Element {
    let mut saving = use_signal(|| Saving::No);

    let disabled = use_memo(move || saving.read().is_saving());

    let exercise_clone = exercise.clone();
    let on_save = use_callback(move |()| {
        let exercise = exercise_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            match delete_exercise(exercise.id).await {
                Ok(_) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_delete(exercise.clone());
                }
                Err(err) => saving.set(Saving::Finished(Err(EditError::Server(err)))),
            }
        });
    });

    rsx! {
        h3 { class: "text-lg font-bold",
            "Delete exercise "
            {exercise.name()}
        }
        p { class: "py-4", "Press ESC key or click the button below to close" }
        ExerciseSummary { exercise: exercise.clone() }
        form {
            novalidate: true,
            action: "javascript:void(0)",
            method: "dialog",
            onkeyup: move |event| {
                if event.key() == Key::Escape {
                    on_cancel(());
                }
            },
            FormSaveCancelButton {
                disabled,
                on_save: move |()| on_save(()),
                on_cancel: move |_| on_cancel(()),
                title: "Delete",
                saving,
            }
        }
    }
}

const WALKING_SVG: Asset = asset!("/assets/exercise/walking.svg");
const RUNNING_SVG: Asset = asset!("/assets/exercise/running.svg");
const CYCLING_SVG: Asset = asset!("/assets/exercise/cycling.svg");
const INDOOR_CYCLING_SVG: Asset = asset!("/assets/exercise/indoor_cycling.svg");
const JUMPING_SVG: Asset = asset!("/assets/exercise/jumping.svg");
const SKIPPING_SVG: Asset = asset!("/assets/exercise/skipping.svg");
const FLYING_SVG: Asset = asset!("/assets/exercise/flying.svg");
const OTHER_SVG: Asset = asset!("/assets/exercise/other.svg");

#[component]
pub fn exercise_icon(exercise_type: ExerciseType) -> Element {
    let icon = match exercise_type {
        ExerciseType::Walking => WALKING_SVG,
        ExerciseType::Running => RUNNING_SVG,
        ExerciseType::Cycling => CYCLING_SVG,
        ExerciseType::IndoorCycling => INDOOR_CYCLING_SVG,
        ExerciseType::Jumping => JUMPING_SVG,
        ExerciseType::Skipping => SKIPPING_SVG,
        ExerciseType::Flying => FLYING_SVG,
        ExerciseType::Other => OTHER_SVG,
    };
    let alt = exercise_title(exercise_type);
    rsx! {
        img { alt, src: icon }
    }
}

pub const EXERCISE_TYPES: [ExerciseType; 8] = [
    ExerciseType::Walking,
    ExerciseType::Running,
    ExerciseType::Cycling,
    ExerciseType::IndoorCycling,
    ExerciseType::Jumping,
    ExerciseType::Skipping,
    ExerciseType::Flying,
    ExerciseType::Other,
];

pub fn exercise_id(exercise_type: ExerciseType) -> &'static str {
    match exercise_type {
        ExerciseType::Walking => "walking",
        ExerciseType::Running => "running",
        ExerciseType::Cycling => "cycling",
        ExerciseType::IndoorCycling => "indoor_cycling",
        ExerciseType::Jumping => "jumping",
        ExerciseType::Skipping => "skipping",
        ExerciseType::Flying => "flying",
        ExerciseType::Other => "other",
    }
}

pub fn exercise_title(exercise_type: ExerciseType) -> &'static str {
    match exercise_type {
        ExerciseType::Walking => "Walking",
        ExerciseType::Running => "Running",
        ExerciseType::Cycling => "Cycling",
        ExerciseType::IndoorCycling => "Indoor Cycling",
        ExerciseType::Jumping => "Jumping",
        ExerciseType::Skipping => "Skipping",
        ExerciseType::Flying => "Flying",
        ExerciseType::Other => "Other",
    }
}

#[component]
pub fn exercise_duration(duration: chrono::TimeDelta) -> Element {
    let text = time_delta_to_string(duration);

    rsx! {
        if duration.num_seconds() < 2 {
            span { class: "text-error", {text} }
        } else if duration.num_minutes() < 60 {
            span { class: "text-success", {text} }
        } else {
            span { class: "text-error", {text} }
        }
    }
}

#[component]
pub fn exercise_calories(calories: Option<i32>) -> Element {
    let text = if let Some(c) = calories {
        format!("{} kcal", c)
    } else {
        "N/A".to_string()
    };
    let classes = if let Some(c) = calories {
        if c == 0 {
            classes!["text-error"]
        } else if c <= 300 {
            classes!["text-success"]
        } else if c <= 1000 {
            classes!["text-warning"]
        } else {
            classes!["text-red-800"]
        }
    } else {
        classes!["text-success"]
    };
    rsx! {
        span { class: classes, {text} }
    }
}

#[component]
pub fn exercise_rpe(rpe: Option<i32>) -> Element {
    let text = match rpe {
        Some(1) => "1 (Very Light)".to_string(),
        Some(2) => "2 (Light Activity)".to_string(),
        Some(3) => "3 (Light Activity)".to_string(),
        Some(4) => "4 (Moderate Activity)".to_string(),
        Some(5) => "5 (Moderate Activity)".to_string(),
        Some(6) => "6 (Moderate Activity)".to_string(),
        Some(7) => "7 (Vigorous Activity)".to_string(),
        Some(8) => "8 (Vigorous Activity)".to_string(),
        Some(9) => "9 (Very Hard Activity)".to_string(),
        Some(10) => "10 (Max Effort Activity)".to_string(),
        Some(i) => format!("{} (Unknown)", i),
        None => "N/A".to_string(),
    };

    let classes = match rpe {
        Some(1) => classes!["text-blue-800"],
        Some(2..=3) => classes!["text-blue-400"],
        Some(4..=6) => classes!["text-green-400"],
        Some(7..=8) => classes!["text-yellow-400"],
        Some(9) => classes!["text-orange-400"],
        Some(10) => classes!["text-red-800"],
        Some(value) if (7..=10).contains(&value) => classes!["text-error"],
        Some(_) => classes!["text-error"],
        None => classes!["text-success"],
    };

    let description = match rpe {
        Some(1) => Some("Hardly any exertion, but more then sleeping, watching TV, etc."),
        Some(2..=3) => {
            Some("Feels like you can maintain for hours, easy to breathe and carry a conversation.")
        }
        Some(4..=6) => Some(
            "Breathing heavily, but can still hold a conversation. Still somewhat comfortable, but becoming more challenging.",
        ),
        Some(7..=8) => Some("Borderline uncomfortable, short of breath, can speak a sentence."),
        Some(9) => {
            Some("Very difficult to maintain. Can barely breathe and speak only a few words.")
        }
        Some(10) => Some(
            "Feels almost impossible to keep going. Completely out of breath, unable to talk. Cannot maintain for more than a few seconds.",
        ),
        Some(value) if (7..=10).contains(&value) => Some("Very Hard to Max Effort Activity"),
        Some(_) | None => None,
    };

    rsx! {
        span { class: classes,
            {text}
            if let Some(description) = description {
                br {}
                span { class: "text-sm", {description} }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveDialog {
    Change(Operation),
    Delete(Exercise),
    #[allow(dead_code)]
    Idle,
}

#[component]
pub fn ExerciseDialog(
    dialog: ActiveDialog,
    on_close: Callback<()>,
    on_change: Callback<Exercise>,
    on_delete: Callback<Exercise>,
) -> Element {
    match dialog {
        ActiveDialog::Change(op) => {
            rsx! {
                Dialog {
                    ExerciseUpdate { op, on_cancel: on_close, on_save: on_change }
                }
            }
        }
        ActiveDialog::Delete(exercise) => {
            rsx! {
                Dialog {
                    ExerciseDelete { exercise, on_cancel: on_close, on_delete }
                }
            }
        }
        ActiveDialog::Idle => {
            rsx! {}
        }
    }
}

#[component]
pub fn ExerciseSummary(exercise: Exercise) -> Element {
    rsx! {
        div { {exercise_title(exercise.exercise_type)} }
        div {
            event_date_time_short { time: exercise.time }
        }
        div {
            exercise_duration { duration: exercise.duration }
        }
        div {
            {exercise.exercise_type.to_string()}
            if let Some(comments) = &exercise.comments {
                Markdown { content: comments.to_string() }
            }
        }
    }
}

#[component]
pub fn ExerciseDetails(exercise: Exercise) -> Element {
    rsx! {
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
