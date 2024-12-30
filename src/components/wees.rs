use chrono::{DateTime, Local, TimeDelta, Utc};
use dioxus::prelude::*;
use palette::Hsv;
use std::sync::Arc;

use crate::{
    forms::{
        validate_colour, validate_comments, validate_duration, validate_mls, validate_time,
        validate_urgency, CancelButton, DeleteButton, EditError, InputColour, InputString, Saving,
        SubmitButton, ValidationError,
    },
    functions::wees::{create_wee, delete_wee, update_wee},
    models::{NewWee, UpdateWee, Wee},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Create { user_id: i64 },
    Update { wee: Arc<Wee> },
}

async fn do_save(
    op: &Operation,
    validate_time: Result<DateTime<Utc>, ValidationError>,
    validate_duration: Result<TimeDelta, ValidationError>,
    validate_urgency: Result<i32, ValidationError>,
    validate_mls: Result<i32, ValidationError>,
    validate_colour: Result<Hsv, ValidationError>,
    validate_comments: Result<Option<String>, ValidationError>,
) -> Result<Wee, EditError> {
    let time = validate_time?;
    let duration = validate_duration?;
    let urgency = validate_urgency?;
    let mls = validate_mls?;
    let colour = validate_colour?;
    let comments = validate_comments?;

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
pub fn ChangeWee(
    op: Operation,
    on_cancel: Callback,
    on_save: Callback<Wee>,
    on_delete: Callback<Arc<Wee>>,
) -> Element {
    // let user: Signal<Arc<Option<User>>> = use_context();

    let time = use_signal(|| match &op {
        Operation::Create { .. } => Utc::now().with_timezone(&Local).to_rfc3339(),
        Operation::Update { wee } => {
            let local = wee.time.with_timezone(&Local);
            local.to_rfc3339()
        }
    });
    let duration = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { wee } => wee.duration.num_seconds().to_string(),
    });
    let urgency = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { wee } => wee.urgency.to_string(),
    });
    let mls = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { wee } => wee.mls.to_string(),
    });
    let colour = use_signal(|| match &op {
        Operation::Create { .. } => (String::new(), String::new(), String::new()),
        Operation::Update { wee } => (
            wee.colour.hue.into_inner().to_string(),
            wee.colour.saturation.to_string(),
            wee.colour.value.to_string(),
        ),
    });
    let comments = use_signal(|| match &op {
        Operation::Create { .. } => "".to_string(),
        Operation::Update { wee } => wee.comments.clone().unwrap_or_default(),
    });

    let validate_time = use_memo(move || validate_time(&time()));
    let validate_duration = use_memo(move || validate_duration(&duration()));
    let validate_urgency = use_memo(move || validate_urgency(&urgency()));
    let validate_mls = use_memo(move || validate_mls(&mls()));
    let validate_colour = use_memo(move || validate_colour(colour()));
    let validate_comments = use_memo(move || validate_comments(&comments()));

    let mut saving = use_signal(|| Saving::No);

    // disable form while waiting for response
    let disabled = use_memo(move || saving.read().is_saving());
    let disabled_save = use_memo(move || {
        validate_time().is_err()
            || validate_duration().is_err()
            || validate_urgency().is_err()
            || validate_mls().is_err()
            || validate_colour().is_err()
            || validate_comments().is_err()
            || disabled()
    });

    let op_clone = op.clone();
    let on_save = use_callback(move |()| {
        let op = op_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            let result = do_save(
                &op,
                validate_time(),
                validate_duration(),
                validate_urgency(),
                validate_mls(),
                validate_colour(),
                validate_comments(),
            )
            .await;

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

        dialog { class: "modal modal-open", id: "my_modal_1",
            div { class: "modal-box",
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
                    InputString {
                        id: "time",
                        label: "Time",
                        value: time,
                        validate: validate_time,
                        disabled,
                    }
                    InputString {
                        id: "duration",
                        label: "Duration",
                        value: duration,
                        validate: validate_duration,
                        disabled,
                    }
                    InputString {
                        id: "urgency",
                        label: "Urgency",
                        value: urgency,
                        validate: validate_urgency,
                        disabled,
                    }
                    InputString {
                        id: "mls",
                        label: "Mls",
                        value: mls,
                        validate: validate_mls,
                        disabled,
                    }
                    InputColour {
                        id: "colour",
                        label: "Colour",
                        value: colour,
                        validate: validate_colour,
                        colours: vec![
                            ("light".to_string(), Hsv::new(60.0, 0.2, 1.0)),
                            ("normal".to_string(), Hsv::new(60.0, 1.0, 1.0)),
                            ("dark".to_string(), Hsv::new(44.0, 1.0, 1.0)),
                            ("red".to_string(), Hsv::new(0.0, 1.0, 1.0)),
                        ],
                        disabled,
                    }
                    InputString {
                        id: "comments",
                        label: "Comments",
                        value: comments,
                        validate: validate_comments,
                        disabled,
                    }

                    SubmitButton {
                        disabled: disabled_save,
                        on_save: move |_| on_save(()),
                        title: match &op {
                            Operation::Create { .. } => "Create",
                            Operation::Update { .. } => "Save",
                        },
                    }
                    CancelButton { on_cancel: move |_| on_cancel(()), title: "Close" }
                    if let Operation::Update { wee } = &op {
                        {
                            let wee: Arc<Wee> = wee.clone();
                            rsx! {
                                DeleteButton { on_delete: move |()| on_delete(wee.clone()), title: "Delete" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn DeleteWee(wee: Arc<Wee>, on_cancel: Callback, on_delete: Callback<Arc<Wee>>) -> Element {
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
        dialog { class: "modal modal-open", id: "my_modal_1",
            div { class: "modal-box",
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
                    CancelButton { on_cancel: move |_| on_cancel(()), title: "Close" }
                    SubmitButton {
                        disabled,
                        on_save: move |_| on_save(()),
                        title: "Delete",
                    }
                }
            }
        }
    }
}
