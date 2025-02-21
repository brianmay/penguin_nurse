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
        validate_bristol, validate_colour, validate_comments, validate_duration,
        validate_fixed_offset_date_time, validate_poo_quantity, validate_urgency, Dialog,
        EditError, FieldValue, FormCancelButton, FormSubmitButton, InputColour, InputDateTime,
        InputDuration, InputNumber, InputSelect, InputTextArea, Saving, ValidationError,
    },
    functions::poos::{create_poo, delete_poo, update_poo},
    models::{Bristol, MaybeString, NewPoo, Poo, UpdatePoo, UserId},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Create { user_id: UserId },
    Update { poo: Poo },
}

#[derive(Debug, Clone)]
struct Validate {
    time: Memo<Result<DateTime<FixedOffset>, ValidationError>>,
    duration: Memo<Result<TimeDelta, ValidationError>>,
    urgency: Memo<Result<i32, ValidationError>>,
    quantity: Memo<Result<i32, ValidationError>>,
    bristol: Memo<Result<Bristol, ValidationError>>,
    colour: Memo<Result<Hsv, ValidationError>>,
    comments: Memo<Result<MaybeString, ValidationError>>,
}

async fn do_save(op: &Operation, validate: &Validate) -> Result<Poo, EditError> {
    let time = validate.time.read().clone()?;
    let duration = validate.duration.read().clone()?;
    let urgency = validate.urgency.read().clone()?;
    let quantity = validate.quantity.read().clone()?;
    let bristol = validate.bristol.read().clone()?;
    let colour = validate.colour.read().clone()?;
    let comments = validate.comments.read().clone()?;

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
                comments,
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
                comments: Some(comments),
            };
            update_poo(poo.id, updates).await.map_err(EditError::Server)
        }
    }
}

#[component]
pub fn ChangePoo(op: Operation, on_cancel: Callback, on_save: Callback<Poo>) -> Element {
    let time = use_signal(|| match &op {
        Operation::Create { .. } => Utc::now().with_timezone(&Local).fixed_offset().as_string(),
        Operation::Update { poo } => poo.time.as_string(),
    });
    let duration = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { poo } => poo.duration.as_string(),
    });
    let urgency = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { poo } => poo.urgency.as_string(),
    });
    let quantity = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { poo } => poo.quantity.as_string(),
    });
    let bristol = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { poo } => poo.bristol.as_string(),
    });
    let colour = use_signal(|| match &op {
        Operation::Create { .. } => (String::new(), String::new(), String::new()),
        Operation::Update { poo } => (
            poo.colour.hue.as_string(),
            poo.colour.saturation.as_string(),
            poo.colour.value.as_string(),
        ),
    });
    let comments = use_signal(|| match &op {
        Operation::Create { .. } => "".to_string(),
        Operation::Update { poo } => poo.comments.clone().option().unwrap_or_default(),
    });

    let validate = Validate {
        time: use_memo(move || validate_fixed_offset_date_time(&time())),
        duration: use_memo(move || validate_duration(&duration())),
        urgency: use_memo(move || validate_urgency(&urgency())),
        quantity: use_memo(move || validate_poo_quantity(&quantity())),
        bristol: use_memo(move || validate_bristol(&bristol())),
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
            || validate.quantity.read().is_err()
            || validate.bristol.read().is_err()
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
                Ok(poo) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_save(poo);
                }
                Err(err) => saving.set(Saving::Finished(Err(err))),
            }
        });
    });

    rsx! {
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
                id: "quantity",
                label: "Quantity",
                value: quantity,
                validate: validate.quantity,
                disabled,
            }
            InputSelect {
                id: "bristol",
                label: "Bristol",
                value: bristol,
                validate: validate.bristol,
                options: Bristol::options(),
                disabled,
            }
            InputColour {
                id: "colour",
                label: "Colour",
                value: colour,
                validate: validate.colour,
                colours: vec![
                    ("light".to_string(), Hsv::new(25.0, 1.0, 0.8)),
                    ("normal".to_string(), Hsv::new(25.0, 1.0, 0.5)),
                    ("dark".to_string(), Hsv::new(25.0, 1.0, 0.2)),
                    ("red".to_string(), Hsv::new(0.0, 1.0, 1.0)),
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
pub fn DeletePoo(poo: Poo, on_cancel: Callback, on_delete: Callback<Poo>) -> Element {
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
            FormCancelButton { on_cancel: move |_| on_cancel(()), title: "Close" }
            FormSubmitButton {
                disabled,
                on_save: move |_| on_save(()),
                title: "Delete",
            }
        }
    }
}

const POO_SVG: Asset = asset!("/assets/poo.svg");

#[component]
pub fn poo_icon() -> Element {
    rsx! {
        img { class: "w-10 invert inline-block", alt: "Poo", src: POO_SVG }
    }
}

#[component]
pub fn poo_duration(duration: chrono::TimeDelta) -> Element {
    let text = time_delta_to_string(duration);

    let classes = if duration.num_seconds() == 0 {
        classes!["text-error"]
    } else if duration.num_minutes() < 3 {
        classes!["text-success"]
    } else if duration.num_minutes() < 10 {
        classes!["text-warning"]
    } else {
        classes!["text-error"]
    };

    rsx! {
        span { class: classes, {text} }
    }
}

#[component]
pub fn poo_bristol(bristol: Bristol) -> Element {
    let bristol_string = bristol.as_str();

    let classes = match bristol {
        Bristol::B0 => classes!["text-error"],
        Bristol::B1 => classes!["text-error"],
        Bristol::B2 => classes!["text-success"],
        Bristol::B3 => classes!["text-success"],
        Bristol::B4 => classes!["text-success"],
        Bristol::B5 => classes!["text-warning"],
        Bristol::B6 => classes!["text-warning"],
        Bristol::B7 => classes!["text-error"],
    };

    rsx! {
        span { class: classes, {bristol_string} }
    }
}

#[component]
pub fn poo_quantity(quantity: i32) -> Element {
    let classes = if quantity == 0 {
        classes!["text-error"]
    } else if quantity < 2 {
        classes!["text-warning"]
    } else {
        classes!["text-success"]
    };

    rsx! {
        span { class: classes, {quantity.to_string() + " out of 10"} }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveDialog {
    Change(Operation),
    Delete(Poo),
    Idle,
}

#[component]
pub fn PooDialog(
    dialog: ActiveDialog,
    on_close: Callback,
    on_change: Callback<Poo>,
    on_delete: Callback<Poo>,
) -> Element {
    match dialog.clone() {
        ActiveDialog::Change(op) => {
            rsx! {
                Dialog {
                    ChangePoo { op, on_cancel: on_close, on_save: on_change }
                }
            }
        }
        ActiveDialog::Delete(poo) => {
            rsx! {
                Dialog {
                    DeletePoo { poo, on_cancel: on_close, on_delete }
                }
            }
        }
        ActiveDialog::Idle => {
            rsx! {}
        }
    }
}

#[component]
pub fn PooDetail(poo: Poo, on_close: Callback<()>) -> Element {
    rsx! {
        h3 { class: "text-lg font-bold",
            "Poo "
            {poo.id.to_string()}
        }
        div { class: "p-4",
            table { class: "table table-striped",
                tbody {
                    tr {
                        td { "Event" }
                        td { poo_icon {} }
                    }
                    tr {
                        td { "ID" }
                        td { {poo.id.as_inner().to_string()} }
                    }
                    tr {
                        td { "Time" }
                        td {
                            event_date_time { time: poo.time }
                        }
                    }
                    tr {
                        td { "Duration" }
                        td {
                            poo_duration { duration: poo.duration }
                        }
                    }
                    tr {
                        td { "Colour" }
                        td {
                            event_colour { colour: poo.colour }
                        }
                    }
                    tr {
                        td { "Urgency" }
                        td {
                            event_urgency { urgency: poo.urgency }
                        }
                    }
                    tr {
                        td { "Duration" }
                        td {
                            poo_duration { duration: poo.duration }
                        }
                    }
                    tr {
                        td { "Quantity" }
                        td {
                            poo_quantity { quantity: poo.quantity }
                        }
                    }
                    tr {
                        td { "Bristol" }
                        td {
                            poo_bristol { bristol: poo.bristol }
                        }
                    }
                    tr {
                        td { "Created" }
                        td { {poo.created_at.with_timezone(&Local).to_string()} }
                    }
                    tr {
                        td { "Updated" }
                        td { {poo.updated_at.with_timezone(&Local).to_string()} }
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
