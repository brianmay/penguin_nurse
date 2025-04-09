use chrono::{DateTime, FixedOffset, Local, TimeDelta, Utc};
use classes::classes;
use dioxus::prelude::*;
use palette::Hsv;

use crate::{
    components::{
        events::{event_colour, event_date_time, event_urgency},
        times::time_delta_to_string,
    },
    forms::{
        Dialog, EditError, FieldValue, FormCancelButton, FormSubmitButton, InputColour,
        InputDateTime, InputDuration, InputNumber, InputTextArea, Saving, ValidationError,
        validate_colour, validate_comments, validate_duration, validate_fixed_offset_date_time,
        validate_millilitres, validate_urgency,
    },
    functions::wees::{create_wee, delete_wee, update_wee},
    models::{MaybeString, NewWee, UpdateWee, UserId, Wee},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Create { user_id: UserId },
    Update { wee: Wee },
}

#[derive(Debug, Clone)]
struct Validate {
    time: Memo<Result<DateTime<FixedOffset>, ValidationError>>,
    duration: Memo<Result<TimeDelta, ValidationError>>,
    urgency: Memo<Result<i32, ValidationError>>,
    mls: Memo<Result<i32, ValidationError>>,
    colour: Memo<Result<Hsv, ValidationError>>,
    comments: Memo<Result<MaybeString, ValidationError>>,
}

async fn do_save(op: &Operation, validate: &Validate) -> Result<Wee, EditError> {
    let time = validate.time.read().clone()?;
    let duration = validate.duration.read().clone()?;
    let urgency = validate.urgency.read().clone()?;
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
                mls,
                colour,
                comments,
            };
            create_wee(updates).await.map_err(EditError::Server)
        }
        Operation::Update { wee } => {
            let updates = UpdateWee {
                user_id: None,
                time: Some(time),
                duration: Some(duration),
                urgency: Some(urgency),
                mls: Some(mls),
                colour: Some(colour),
                comments: Some(comments),
            };
            update_wee(wee.id, updates).await.map_err(EditError::Server)
        }
    }
}

#[component]
pub fn ChangeWee(op: Operation, on_cancel: Callback, on_save: Callback<Wee>) -> Element {
    let time = use_signal(|| match &op {
        Operation::Create { .. } => Utc::now().with_timezone(&Local).fixed_offset().as_string(),
        Operation::Update { wee } => wee.time.as_string(),
    });
    let duration = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { wee } => wee.duration.as_string(),
    });
    let urgency = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { wee } => wee.urgency.as_string(),
    });
    let mls = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { wee } => wee.mls.as_string(),
    });
    let colour = use_signal(|| match &op {
        Operation::Create { .. } => (String::new(), String::new(), String::new()),
        Operation::Update { wee } => (
            wee.colour.hue.as_string(),
            wee.colour.saturation.as_string(),
            wee.colour.value.as_string(),
        ),
    });
    let comments = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { wee } => wee.comments.as_string(),
    });

    let validate = Validate {
        time: use_memo(move || validate_fixed_offset_date_time(&time())),
        duration: use_memo(move || validate_duration(&duration())),
        urgency: use_memo(move || validate_urgency(&urgency())),
        mls: use_memo(move || validate_millilitres(&mls())),
        colour: use_memo(move || validate_colour(colour())),
        comments: use_memo(move || validate_comments(&comments())),
    };

    let mut saving = use_signal(|| Saving::No);

    // disable form while waiting for response
    let disabled = use_memo(move || saving.read().is_saving());
    let disabled_save = use_memo(move || {
        validate.time.read().is_err()
            || validate.duration.read().is_err()
            || validate.urgency.read().is_err()
            || validate.mls.read().is_err()
            || validate.colour.read().is_err()
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
        match &*saving.read() {
            Saving::Yes => {
                rsx! {
                    div { class: "alert alert-info", "Saving..." }
                }
            }
            Saving::Finished(Ok(())) => {
                rsx! {
                    div { class: "alert alert-success", "Saved!" }
                }
            }
            Saving::Finished(Err(err)) => {
                rsx! {
                    div { class: "alert alert-error",
                        "Error: "
                        {err.to_string()}
                    }
                }
            }
            _ => {
                rsx! {}
            }
        }
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
            InputNumber {
                id: "urgency",
                label: "Urgency",
                value: urgency,
                validate: validate.urgency,
                disabled,
            }
            InputNumber {
                id: "mls",
                label: "Quantity",
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
                    ("light".to_string(), Hsv::new(45.0, 1.0, 0.8)),
                    ("normal".to_string(), Hsv::new(40.0, 1.0, 0.8)),
                    ("dark".to_string(), Hsv::new(35.0, 1.0, 0.8)),
                ],
                disabled,
            }
            InputTextArea {
                id: "comments",
                label: "Comments",
                value: comments,
                validate: validate.comments,
                disabled,
            }

            FormSubmitButton {
                disabled: disabled_save,
                on_save: move |_| on_save(()),
                title: match &op {
                    Operation::Create { .. } => "Create",
                    Operation::Update { .. } => "Save",
                },
            }
            FormCancelButton { on_cancel: move |_| on_cancel(()), title: "Close" }
        }
    }
}

#[component]
pub fn DeleteWee(wee: Wee, on_cancel: Callback, on_delete: Callback<Wee>) -> Element {
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
        match &*saving.read() {
            Saving::Yes => {
                rsx! {
                    div { class: "alert alert-info", "Deleting..." }
                }
            }
            Saving::Finished(Ok(())) => {
                rsx! {
                    div { class: "alert alert-success", "Deleted!" }
                }
            }
            Saving::Finished(Err(err)) => {
                rsx! {
                    div { class: "alert alert-error",
                        "Error: "
                        {err.to_string()}
                    }
                }
            }
            _ => {
                rsx! {}
            }
        }
        form {
            novalidate: true,
            action: "javascript:void(0)",
            method: "dialog",
            onkeyup: move |event| {
                if event.key() == Key::Escape {
                    on_cancel(());
                }
            },
            FormCancelButton { on_cancel: move |_| on_cancel(()), title: "Close" }
            FormSubmitButton {
                disabled,
                on_save: move |_| on_save(()),
                title: "Delete",
            }
        }
    }
}

const WEE_SVG: Asset = asset!("/assets/wee.svg");

#[component]
pub fn wee_icon() -> Element {
    rsx! {
        img { class: "w-10 invert inline-block", alt: "Wee", src: WEE_SVG }
    }
}

#[component]
pub fn wee_duration(duration: chrono::TimeDelta) -> Element {
    let text = time_delta_to_string(duration);

    let classes = if duration.num_seconds() == 0 {
        classes!["text-error"]
    } else if duration.num_seconds() < 60 {
        classes!["text-success"]
    } else if duration.num_minutes() < 3 {
        classes!["text-warning"]
    } else {
        classes!["text-error"]
    };

    rsx! {
        span { class: classes, {text} }
    }
}

#[component]
pub fn wee_mls(mls: i32) -> Element {
    let classes = if mls == 0 {
        classes!["text-error"]
    } else if mls < 100 {
        classes!["text-warning"]
    } else if mls < 500 {
        classes!["text-success"]
    } else {
        classes!["text-error"]
    };

    rsx! {
        span { class: classes, {mls.to_string() + " ml"} }
    }
}

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
                    ChangeWee { op, on_cancel: on_close, on_save: on_change }
                }
            }
        }
        ActiveDialog::Delete(wee) => {
            rsx! {
                Dialog {
                    DeleteWee { wee, on_cancel: on_close, on_delete }
                }
            }
        }
        ActiveDialog::Idle => {
            rsx! {}
        }
    }
}

#[component]
pub fn WeeDetail(wee: Wee, on_close: Callback<()>) -> Element {
    rsx! {
        h3 { class: "text-lg font-bold",
            "Wee "
            {wee.id.to_string()}
        }
        div { class: "p-4",
            table { class: "table table-striped",
                tbody {
                    tr {
                        td { "Event" }
                        td { wee_icon {} }
                    }
                    tr {
                        td { "ID" }
                        td { {wee.id.as_inner().to_string()} }
                    }
                    tr {
                        td { "Time" }
                        td {
                            event_date_time { time: wee.time }
                        }
                    }
                    tr {
                        td { "Duration" }
                        td {
                            wee_duration { duration: wee.duration }
                        }
                    }
                    tr {
                        td { "Colour" }
                        td {
                            event_colour { colour: wee.colour }
                        }
                    }
                    tr {
                        td { "Urgency" }
                        td {
                            event_urgency { urgency: wee.urgency }
                        }
                    }
                    tr {
                        td { "Duration" }
                        td {
                            wee_duration { duration: wee.duration }
                        }
                    }
                    tr {
                        td { "Quantity" }
                        td {
                            wee_mls { mls: wee.mls }
                        }
                    }
                    tr {
                        td { "Created" }
                        td { {wee.created_at.with_timezone(&Local).to_string()} }
                    }
                    tr {
                        td { "Updated" }
                        td { {wee.updated_at.with_timezone(&Local).to_string()} }
                    }
                }
            }
        }

        div { class: "p-4",
            button {
                class: "btn btn-secondary m-1",
                onclick: move |_| {
                    on_close(());
                },
                "Close"
            }
        }
    }
}
