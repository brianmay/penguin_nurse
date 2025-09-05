use chrono::{DateTime, FixedOffset, Local, TimeDelta, Utc};
use dioxus::prelude::*;

use crate::{
    components::{events::Markdown, times::time_delta_to_string},
    forms::{
        Dialog, EditError, FieldValue, FormSaveCancelButton, InputDateTime, InputDuration,
        InputNumber, InputTextArea, Saving, ValidationError, validate_comments, validate_duration,
        validate_fixed_offset_date_time, validate_location, validate_symptom_intensity,
    },
    functions::refluxs::{create_reflux, delete_reflux, update_reflux},
    models::{ChangeReflux, MaybeSet, NewReflux, Reflux, UserId},
};
use classes::classes;

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Create { user_id: UserId },
    Update { reflux: Reflux },
}

#[derive(Debug, Clone)]
struct Validate {
    time: Memo<Result<DateTime<FixedOffset>, ValidationError>>,
    duration: Memo<Result<TimeDelta, ValidationError>>,
    location: Memo<Result<Option<String>, ValidationError>>,
    severity: Memo<Result<i32, ValidationError>>,
    comments: Memo<Result<Option<String>, ValidationError>>,
}

async fn do_save(op: &Operation, validate: &Validate) -> Result<Reflux, EditError> {
    let time = validate.time.read().clone()?;
    let duration = validate.duration.read().clone()?;
    let location = validate.location.read().clone()?;
    let severity = validate.severity.read().clone()?;
    let comments = validate.comments.read().clone()?;

    match op {
        Operation::Create { user_id } => {
            let updates = NewReflux {
                user_id: *user_id,
                time,
                duration,
                location,
                severity,
                comments,
            };
            create_reflux(updates).await.map_err(EditError::Server)
        }
        Operation::Update { reflux } => {
            let changes = ChangeReflux {
                user_id: MaybeSet::NoChange,
                time: MaybeSet::Set(time),
                duration: MaybeSet::Set(duration),
                location: MaybeSet::Set(location),
                severity: MaybeSet::Set(severity),
                comments: MaybeSet::Set(comments),
            };
            update_reflux(reflux.id, changes)
                .await
                .map_err(EditError::Server)
        }
    }
}

#[component]
pub fn RefluxUpdate(op: Operation, on_cancel: Callback, on_save: Callback<Reflux>) -> Element {
    let time = use_signal(|| match &op {
        Operation::Create { .. } => Utc::now().with_timezone(&Local).fixed_offset().as_string(),
        Operation::Update { reflux } => reflux.time.as_string(),
    });

    let duration = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { reflux } => reflux.duration.as_string(),
    });

    let location = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { reflux } => reflux.location.as_string(),
    });

    let severity = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { reflux } => reflux.severity.to_string(),
    });

    let comments = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { reflux } => reflux.comments.as_string(),
    });

    let validate = Validate {
        time: use_memo(move || validate_fixed_offset_date_time(&time())),
        duration: use_memo(move || validate_duration(&duration())),
        location: use_memo(move || validate_location(&location())),
        severity: use_memo(move || validate_symptom_intensity(&severity())),
        comments: use_memo(move || validate_comments(&comments())),
    };

    let mut saving = use_signal(|| Saving::No);

    // disable form while waiting for response
    let disabled = use_memo(move || saving.read().is_saving());
    let disabled_save = use_memo(move || {
        validate.time.read().is_err()
            || validate.duration.read().is_err()
            || validate.location.read().is_err()
            || validate.severity.read().is_err()
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
                Operation::Create { .. } => "Create Reflux".to_string(),
                Operation::Update { reflux } => format!("Edit Reflux {}", reflux.name()),
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
                id: "severity",
                label: "Severity (0-10)",
                value: severity,
                validate: validate.severity,
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
pub fn RefluxDelete(reflux: Reflux, on_cancel: Callback, on_delete: Callback<Reflux>) -> Element {
    let mut saving = use_signal(|| Saving::No);

    let disabled = use_memo(move || saving.read().is_saving());

    let reflux_clone = reflux.clone();
    let on_save = use_callback(move |()| {
        let reflux = reflux_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            match delete_reflux(reflux.id).await {
                Ok(_) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_delete(reflux.clone());
                }
                Err(err) => saving.set(Saving::Finished(Err(EditError::Server(err)))),
            }
        });
    });

    rsx! {
        h3 { class: "text-lg font-bold",
            "Delete reflux "
            {reflux.name()}
        }
        p { class: "py-4", "Press ESC key or click the button below to close" }
        RefluxSummary { reflux: reflux.clone() }
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

const REFLUX_SVG: Asset = asset!("/assets/reflux.svg");

#[component]
pub fn reflux_icon() -> Element {
    let alt = reflux_title();
    let icon = REFLUX_SVG;
    rsx! {
        img { class: "w-10 dark:invert inline-block", alt, src: icon }
    }
}

#[component]
pub fn reflux_title() -> &'static str {
    "Reflux"
}

#[component]
pub fn reflux_duration(duration: chrono::TimeDelta) -> Element {
    let text = time_delta_to_string(duration);

    rsx! {
        span { {text} }
    }
}

#[component]
pub fn reflux_calories(calories: Option<i32>) -> Element {
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

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveDialog {
    Change(Operation),
    Delete(Reflux),
    #[allow(dead_code)]
    Idle,
}

#[component]
pub fn RefluxDialog(
    dialog: ActiveDialog,
    on_close: Callback<()>,
    on_change: Callback<Reflux>,
    on_delete: Callback<Reflux>,
) -> Element {
    match dialog {
        ActiveDialog::Change(op) => {
            rsx! {
                Dialog {
                    RefluxUpdate { op, on_cancel: on_close, on_save: on_change }
                }
            }
        }
        ActiveDialog::Delete(reflux) => {
            rsx! {
                Dialog {
                    RefluxDelete { reflux, on_cancel: on_close, on_delete }
                }
            }
        }
        ActiveDialog::Idle => {
            rsx! {}
        }
    }
}

#[component]
pub fn RefluxSummary(reflux: Reflux) -> Element {
    rsx! {
        div {
            div { reflux_icon {} }
            div {
                reflux_duration { duration: reflux.duration }
            }
            div {
                span { class: "font-bold", "Severity: " }
                span { {reflux.severity.to_string()} }
                if let Some(location) = &reflux.location {
                    br {}
                    span { class: "font-bold", "Location: " }
                    span { {location.to_string()} }
                }
                if let Some(comments) = &reflux.comments {
                    Markdown { content: comments.to_string() }
                }
            }
        }
    }
}
