use chrono::{DateTime, Local, TimeDelta, Utc};
use dioxus::prelude::*;
use palette::Hsv;
use std::sync::Arc;

use crate::{
    forms::{
        validate_bristol, validate_colour, validate_comments, validate_duration,
        validate_poo_quantity, validate_time, validate_urgency, CancelButton, DeleteButton,
        EditError, InputColour, InputSelect, InputString, Saving, SubmitButton, ValidationError,
    },
    functions::poos::{create_poo, delete_poo, update_poo},
    models::{Bristol, NewPoo, Poo, UpdatePoo},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Create { user_id: i64 },
    Update { poo: Arc<Poo> },
}

async fn do_save(
    op: &Operation,
    validate_time: Result<DateTime<Utc>, ValidationError>,
    validate_duration: Result<TimeDelta, ValidationError>,
    validate_urgency: Result<i32, ValidationError>,
    validate_quantity: Result<i32, ValidationError>,
    validate_bristol: Result<Bristol, ValidationError>,
    validate_colour: Result<Hsv, ValidationError>,
    validate_comments: Result<Option<String>, ValidationError>,
) -> Result<Poo, EditError> {
    let time = validate_time?;
    let duration = validate_duration?;
    let urgency = validate_urgency?;
    let quantity = validate_quantity?;
    let bristol = validate_bristol?;
    let colour = validate_colour?;
    let comments = validate_comments?;

    match op {
        Operation::Create { user_id } => {
            let updates = NewPoo {
                user_id: *user_id,
                time,
                duration,
                urgency,
                quantity,
                bristol,
                colour,
                comments: comments.into(),
            };
            create_poo(updates).await.map_err(EditError::Server)
        }
        Operation::Update { poo } => {
            let updates = UpdatePoo {
                user_id: None,
                time: Some(time),
                duration: Some(duration),
                urgency: Some(urgency),
                quantity: Some(quantity),
                bristol: Some(bristol),
                colour: Some(colour),
                comments: Some(comments.into()),
            };
            use tracing::error;
            error!("updates: {:?}", updates);
            update_poo(poo.id, updates).await.map_err(EditError::Server)
        }
    }
}

#[component]
pub fn ChangePoo(
    op: Operation,
    on_cancel: Callback,
    on_save: Callback<Poo>,
    on_delete: Callback<Arc<Poo>>,
) -> Element {
    // let user: Signal<Arc<Option<User>>> = use_context();

    let time = use_signal(|| match &op {
        Operation::Create { .. } => Utc::now().with_timezone(&Local).to_rfc3339(),
        Operation::Update { poo } => {
            let local = poo.time.with_timezone(&Local);
            local.to_rfc3339()
        }
    });
    let duration = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { poo } => poo.duration.num_seconds().to_string(),
    });
    let urgency = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { poo } => poo.urgency.to_string(),
    });
    let quantity = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { poo } => poo.quantity.to_string(),
    });
    let bristol = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { poo } => poo.bristol.as_value().to_string(),
    });
    let colour = use_signal(|| match &op {
        Operation::Create { .. } => (String::new(), String::new(), String::new()),
        Operation::Update { poo } => (
            poo.colour.hue.into_inner().to_string(),
            poo.colour.saturation.to_string(),
            poo.colour.value.to_string(),
        ),
    });
    let comments = use_signal(|| match &op {
        Operation::Create { .. } => "".to_string(),
        Operation::Update { poo } => poo.comments.clone().option().unwrap_or_default(),
    });

    let validate_time = use_memo(move || validate_time(&time()));
    let validate_duration = use_memo(move || validate_duration(&duration()));
    let validate_urgency = use_memo(move || validate_urgency(&urgency()));
    let validate_quantity = use_memo(move || validate_poo_quantity(&quantity()));
    let validate_bristol = use_memo(move || validate_bristol(&bristol()));
    let validate_colour = use_memo(move || validate_colour(colour()));
    let validate_comments = use_memo(move || validate_comments(&comments()));

    let mut saving = use_signal(|| Saving::No);

    // disable form while waiting for response
    let disabled = use_memo(move || saving.read().is_saving());
    let disabled_save = use_memo(move || {
        validate_time().is_err()
            || validate_duration().is_err()
            || validate_urgency().is_err()
            || validate_quantity().is_err()
            || validate_bristol().is_err()
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
                validate_quantity(),
                validate_bristol(),
                validate_colour(),
                validate_comments(),
            )
            .await;

            match result {
                Ok(poo) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_save(poo);
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
                        Operation::Create { .. } => "Create Poo".to_string(),
                        Operation::Update { poo } => format!("Edit Poo {}", poo.id),
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
                        id: "quantity",
                        label: "Quantity",
                        value: quantity,
                        validate: validate_quantity,
                        disabled,
                    }
                    InputSelect {
                        id: "bristol",
                        label: "Bristol",
                        value: bristol,
                        validate: validate_bristol,
                        options: Bristol::options(),
                        disabled,
                    }
                    InputColour {
                        id: "colour",
                        label: "Colour",
                        value: colour,
                        validate: validate_colour,
                        colours: vec![
                            ("light".to_string(), Hsv::new(25.0, 1.0, 0.8)),
                            ("normal".to_string(), Hsv::new(25.0, 1.0, 0.5)),
                            ("dark".to_string(), Hsv::new(25.0, 1.0, 0.2)),
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
                    if let Operation::Update { poo } = &op {
                        {
                            let poo: Arc<Poo> = poo.clone();
                            rsx! {
                                DeleteButton { on_delete: move |()| on_delete(poo.clone()), title: "Delete" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn DeletePoo(poo: Arc<Poo>, on_cancel: Callback, on_delete: Callback<Arc<Poo>>) -> Element {
    let mut saving = use_signal(|| Saving::No);

    let disabled = use_memo(move || saving.read().is_saving());

    let poo_clone = poo.clone();
    let on_save = use_callback(move |()| {
        let poo_clone = poo_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            match delete_poo(poo_clone.id).await {
                Ok(_) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_delete(poo_clone.clone());
                }
                Err(err) => saving.set(Saving::Finished(Err(EditError::Server(err)))),
            }
        });
    });

    rsx! {
        dialog { class: "modal modal-open", id: "my_modal_1",
            div { class: "modal-box",
                h3 { class: "text-lg font-bold",
                    "Delete poo "
                    {poo.id.to_string()}
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
