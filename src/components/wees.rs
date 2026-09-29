use chrono::Duration;
use chrono::{DateTime, FixedOffset, Local, Utc};
use classes::classes;
use dioxus::prelude::*;
use palette::Hsv;

use crate::{
    components::{
        events::{EventDateTimeShort, Markdown, UrgencyLabel, event_colour},
        symptoms::{SymptomDisplay, SymptomIntensity},
        times::time_delta_to_string,
    },
    forms::{
        Colour, Dialog, EditError, FieldValue, FormSaveCancelButton, InputBoolean, InputColour,
        InputDateTime, InputDuration, InputNumber, InputTextArea, InputUrgency, Saving,
        ValidationError, validate_colour_maybe, validate_comments, validate_fixed_offset_date_time,
        validate_optional_chrono_duration, validate_optional_leakage, validate_optional_mls,
        validate_urgency,
    },
    functions::wees::{create_wee, delete_wee, update_wee},
    models::{ChangeWee, MaybeSet, NewWee, Urgency, UserId, Wee},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Create { user_id: UserId },
    Update { wee: Wee },
}

#[derive(Debug, Clone)]
struct Validate {
    time: Memo<Result<DateTime<FixedOffset>, ValidationError>>,
    duration: Memo<Result<Option<Duration>, ValidationError>>,
    urgency: Memo<Result<Urgency, ValidationError>>,
    leakage: Memo<Result<Option<i32>, ValidationError>>,
    mls: Memo<Result<Option<i32>, ValidationError>>,
    colour: Memo<Result<Option<Hsv>, ValidationError>>,
    comments: Memo<Result<Option<String>, ValidationError>>,
}

async fn do_save(op: &Operation, validate: &Validate, complete: bool) -> Result<Wee, EditError> {
    let time = validate.time.read().clone()?;
    let duration = validate.duration.read().clone()?;
    let urgency = validate.urgency.read().clone()?;
    let leakage = validate.leakage.read().clone()?;
    let mls = validate.mls.read().clone()?;
    let colour = validate.colour.read().clone()?;
    let comments = validate.comments.read().clone()?;

    match op {
        Operation::Create { user_id } => {
            let updates = NewWee {
                user_id: *user_id,
                time,
                duration,
                urgency,
                leakage,
                mls,
                colour,
                comments,
                complete,
            };
            create_wee(updates).await.map_err(EditError::Server)
        }
        Operation::Update { wee } => {
            let changes = ChangeWee {
                user_id: MaybeSet::NoChange,
                time: MaybeSet::Set(time),
                duration: MaybeSet::Set(duration),
                urgency: MaybeSet::Set(urgency),
                leakage: MaybeSet::Set(leakage),
                mls: MaybeSet::Set(mls),
                colour: MaybeSet::Set(colour),
                comments: MaybeSet::Set(comments),
                complete: MaybeSet::Set(complete),
            };
            update_wee(wee.id, changes).await.map_err(EditError::Server)
        }
    }
}

#[component]
pub fn WeeUpdate(op: Operation, on_cancel: Callback, on_save: Callback<Wee>) -> Element {
    let time = use_signal(|| match &op {
        Operation::Create { .. } => Utc::now().with_timezone(&Local).fixed_offset().as_raw(),
        Operation::Update { wee } => wee.time.as_raw(),
    });
    let duration = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { wee } => wee.duration.as_raw(),
    });
    let complete = use_signal(|| match &op {
        Operation::Create { .. } => false,
        Operation::Update { wee } => wee.complete,
    });
    let urgency = use_signal(|| match &op {
        Operation::Create { .. } => None,
        Operation::Update { wee } => Some(wee.urgency),
    });
    let leakage = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { wee } => wee.leakage.map(|l| l.to_string()).unwrap_or_default(),
    });
    let mls = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { wee } => wee.mls.map(|m| m.to_string()).unwrap_or_default(),
    });
    let colour = use_signal(|| match &op {
        Operation::Create { .. } => (String::new(), String::new(), String::new()),
        Operation::Update { wee } => {
            if let Some(colour) = wee.colour {
                (
                    colour.hue.into_inner().to_string(),
                    colour.saturation.to_string(),
                    colour.value.to_string(),
                )
            } else {
                (String::new(), String::new(), String::new())
            }
        }
    });
    let comments = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { wee } => wee.comments.as_raw(),
    });

    let validate = {
        let mls_validate = use_memo(move || validate_optional_mls(*complete.read(), &mls()));
        Validate {
            time: use_memo(move || validate_fixed_offset_date_time(&time())),
            duration: use_memo(move || {
                validate_optional_chrono_duration(*complete.read(), &duration())
            }),
            urgency: use_memo(move || validate_urgency(urgency())),
            leakage: use_memo(move || validate_optional_leakage(*complete.read(), &leakage())),
            mls: mls_validate,
            colour: use_memo(move || validate_colour_maybe(&mls_validate.read(), colour())),
            comments: use_memo(move || validate_comments(&comments())),
        }
    };

    let mut saving = use_signal(|| Saving::No);

    // disable form while waiting for response
    let disabled = use_memo(move || saving.read().is_saving());
    let disabled_save = use_memo(move || {
        validate.time.read().is_err()
            || validate.duration.read().is_err()
            || validate.urgency.read().is_err()
            || validate.leakage.read().is_err()
            || validate.mls.read().is_err()
            || validate.colour.read().is_err()
            || validate.comments.read().is_err()
            || disabled()
    });

    let op_clone = op.clone();
    let validate_clone = validate.clone();
    let complete_clone = *complete.read();
    let on_save = use_callback(move |()| {
        let op = op_clone.clone();
        let validate = validate_clone.clone();
        let complete = complete_clone;
        spawn(async move {
            saving.set(Saving::Yes);

            let result = do_save(&op, &validate, complete).await;

            match result {
                Ok(wee) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_save(wee);
                }
                Err(err) => saving.set(Saving::Finished(Err(err))),
            }
        });
    });

    rsx! {
        h3 { class: "text-lg font-bold",
            match &op {
                Operation::Create { .. } => "Create Wee".to_string(),
                Operation::Update { wee } => format!("Edit Wee {}", wee.id),
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
            InputUrgency {
                id: "urgency",
                label: "Urgency",
                value: urgency,
                validate: validate.urgency,
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
            InputBoolean {
                id: "complete",
                label: "Complete",
                value: complete,
                disabled,
            }
            InputNumber {
                id: "leakage",
                label: "Leakage (0-10)".to_string(),
                value: leakage,
                validate: validate.leakage,
                disabled,
            }
            InputNumber {
                id: "mls",
                label: "Quantity (ml)".to_string(),
                value: mls,
                validate: validate.mls,
                disabled,
            }
            InputColour {
                id: "colour",
                label: "Colour",
                value: colour,
                validate: validate.colour,
                colours: vec![
                    ("extra light".to_string(), Hsv::new(44.0, 1.0, 0.8)),
                    ("light".to_string(), Hsv::new(42.0, 1.0, 0.8)),
                    ("normal".to_string(), Hsv::new(40.0, 1.0, 0.8)),
                    ("dark".to_string(), Hsv::new(38.0, 1.0, 0.8)),
                    ("extra dark".to_string(), Hsv::new(36.0, 1.0, 0.8)),
                ],
                disabled,
            }
            Colour { colour }
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
                on_cancel: move |()| on_cancel(()),
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
pub fn WeeDelete(wee: Wee, on_cancel: Callback, on_delete: Callback<Wee>) -> Element {
    let mut saving = use_signal(|| Saving::No);

    let disabled = use_memo(move || saving.read().is_saving());

    let wee_clone = wee.clone();
    let on_save = use_callback(move |()| {
        let wee_clone = wee_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            match delete_wee(wee_clone.id).await {
                Ok(_) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_delete(wee_clone.clone());
                }
                Err(err) => saving.set(Saving::Finished(Err(EditError::Server(err)))),
            }
        });
    });

    rsx! {
        h3 { class: "text-lg font-bold",
            "Delete wee "
            {wee.id.to_string()}
        }
        p { class: "py-4", "Press ESC key or click the button below to close" }
        WeeSummary { wee: wee.clone() }
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

const WEE_SVG: Asset = asset!("/assets/wee.svg");

#[component]
pub fn WeeIcon() -> Element {
    let alt = wee_title();
    rsx! {
        img { alt, src: WEE_SVG }
    }
}

pub fn wee_title() -> &'static str {
    "Wee"
}

#[component]
pub fn WeeDuration(duration: Option<chrono::Duration>) -> Element {
    match duration {
        Some(d) => {
            let text = time_delta_to_string(d);
            let classes = if d.num_seconds() == 0 {
                classes!["text-error"]
            } else if d.num_seconds() < 60 {
                classes!["text-success"]
            } else if d.num_minutes() < 3 {
                classes!["text-warning"]
            } else {
                classes!["text-error"]
            };
            rsx! {
                span { class: classes, {text} }
            }
        }
        None => rsx! {
            span { class: "text-gray-400", "No duration" }
        },
    }
}

#[component]
pub fn WeeMls(mls: Option<i32>) -> Element {
    match mls {
        Some(m) => {
            let classes = if m == 0 {
                classes!["text-error"]
            } else if m < 100 {
                classes!["text-warning"]
            } else if m < 500 {
                classes!["text-success"]
            } else {
                classes!["text-error"]
            };
            rsx! {
                span { class: classes, {m.to_string() + " ml"} }
            }
        }
        None => rsx! {
            span { class: "text-gray-400", "No quantity" }
        },
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ActiveDialog {
    Change(Operation),
    Delete(Wee),
    Idle,
}

#[component]
pub fn WeeDialog(
    dialog: ActiveDialog,
    on_close: Callback,
    on_change: Callback<Wee>,
    on_delete: Callback<Wee>,
) -> Element {
    match dialog.clone() {
        ActiveDialog::Change(op) => {
            rsx! {
                Dialog {
                    WeeUpdate { op, on_cancel: on_close, on_save: on_change }
                }
            }
        }
        ActiveDialog::Delete(wee) => {
            rsx! {
                Dialog {
                    WeeDelete { wee, on_cancel: on_close, on_delete }
                }
            }
        }
        ActiveDialog::Idle => {
            rsx! {}
        }
    }
}

#[component]
pub fn WeeSummary(wee: Wee) -> Element {
    rsx! {
        div { {wee_title()} }
        div {
            EventDateTimeShort { time: wee.time }
        }
        WeeMls { mls: wee.mls }
        WeeDuration { duration: wee.duration }
        UrgencyLabel { urgency: wee.urgency }
        if let Some(leakage) = wee.leakage {
            SymptomIntensity { intensity: leakage }
        }
        event_colour { colour: wee.colour }
        if let Some(comments) = &wee.comments {
            Markdown { content: comments.to_string() }
        }
    }
}
#[component]
pub fn WeeDetails(wee: Wee) -> Element {
    rsx! {
        event_colour { colour: wee.colour }
        div { class: "inline-block align-top",
            div {
                WeeMls { mls: wee.mls }
            }
            div {
                UrgencyLabel { urgency: wee.urgency }
            }
            if let Some(leakage) = wee.leakage {
                SymptomDisplay {
                    name: "Leakage".to_string(),
                    intensity: leakage,
                    extra: None,
                }
            }
        }
        if let Some(comments) = &wee.comments {
            Markdown { content: comments.to_string() }
        }
    }
}
