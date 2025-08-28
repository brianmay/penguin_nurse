use chrono::{DateTime, FixedOffset, Local, Utc};
use dioxus::prelude::*;

use crate::{
    forms::{
        Dialog, EditError, FieldValue, FormSaveCancelButton, InputDateTime, InputNumber,
        InputTextArea, Saving, ValidationError, validate_comments, validate_fixed_offset_date_time,
        validate_urgency,
    },
    functions::wee_urges::{create_wee_urge, delete_wee_urge, update_wee_urge},
    models::{ChangeWeeUrge, MaybeString, NewWeeUrge, UserId, WeeUrge},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Create { user_id: UserId },
    Update { wee_urge: WeeUrge },
}

#[derive(Debug, Clone)]
struct Validate {
    time: Memo<Result<DateTime<FixedOffset>, ValidationError>>,
    urgency: Memo<Result<i32, ValidationError>>,
    comments: Memo<Result<MaybeString, ValidationError>>,
}

async fn do_save(op: &Operation, validate: &Validate) -> Result<WeeUrge, EditError> {
    let time = validate.time.read().clone()?;
    let urgency = validate.urgency.read().clone()?;
    let comments = validate.comments.read().clone()?;

    match op {
        Operation::Create { user_id } => {
            let updates = NewWeeUrge {
                user_id: *user_id,
                time,
                urgency,
                comments,
            };
            create_wee_urge(updates).await.map_err(EditError::Server)
        }
        Operation::Update { wee_urge } => {
            let changes = ChangeWeeUrge {
                user_id: None,
                time: Some(time),
                urgency: Some(urgency),
                comments: Some(comments),
            };
            update_wee_urge(wee_urge.id, changes)
                .await
                .map_err(EditError::Server)
        }
    }
}

#[component]
pub fn WeeUrgeUpdate(op: Operation, on_cancel: Callback, on_save: Callback<WeeUrge>) -> Element {
    let time = use_signal(|| match &op {
        Operation::Create { .. } => Utc::now().with_timezone(&Local).fixed_offset().as_string(),
        Operation::Update { wee_urge } => wee_urge.time.as_string(),
    });
    let urgency = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { wee_urge } => wee_urge.urgency.as_string(),
    });
    let comments = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { wee_urge } => wee_urge.comments.as_string(),
    });

    let validate = Validate {
        time: use_memo(move || validate_fixed_offset_date_time(&time())),
        urgency: use_memo(move || validate_urgency(&urgency())),
        comments: use_memo(move || validate_comments(&comments())),
    };

    let mut saving = use_signal(|| Saving::No);

    // disable form while waiting for response
    let disabled = use_memo(move || saving.read().is_saving());
    let disabled_save = use_memo(move || {
        validate.time.read().is_err()
            || validate.urgency.read().is_err()
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
                Ok(wee_urge) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_save(wee_urge);
                }
                Err(err) => saving.set(Saving::Finished(Err(err))),
            }
        });
    });

    rsx! {
        h3 { class: "text-lg font-bold",
            match &op {
                Operation::Create { .. } => "Create Wee Urge".to_string(),
                Operation::Update { wee_urge } => format!("Edit Wee Urge {}", wee_urge.id),
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
            InputNumber {
                id: "urgency",
                label: "Urgency",
                value: urgency,
                validate: validate.urgency,
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
pub fn WeeUrgeDelete(
    wee_urge: WeeUrge,
    on_cancel: Callback,
    on_delete: Callback<WeeUrge>,
) -> Element {
    let mut saving = use_signal(|| Saving::No);

    let disabled = use_memo(move || saving.read().is_saving());

    let wee_urge_clone = wee_urge.clone();
    let on_save = use_callback(move |()| {
        let wee_urge_clone = wee_urge_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            match delete_wee_urge(wee_urge_clone.id).await {
                Ok(_) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_delete(wee_urge_clone.clone());
                }
                Err(err) => saving.set(Saving::Finished(Err(EditError::Server(err)))),
            }
        });
    });

    rsx! {
        h3 { class: "text-lg font-bold",
            "Delete wee_urge "
            {wee_urge.id.to_string()}
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

const WEE_URGENCY_SVG: Asset = asset!("/assets/wee_urge.svg");

#[component]
pub fn wee_urge_icon() -> Element {
    let alt = wee_urge_title();
    rsx! {
        img {
            class: "w-10 dark:invert inline-block",
            alt,
            src: WEE_URGENCY_SVG,
        }
    }
}

pub fn wee_urge_title() -> &'static str {
    "Wee Urgency"
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ActiveDialog {
    Change(Operation),
    Delete(WeeUrge),
    Idle,
}

#[component]
pub fn WeeUrgeDialog(
    dialog: ActiveDialog,
    on_close: Callback,
    on_change: Callback<WeeUrge>,
    on_delete: Callback<WeeUrge>,
) -> Element {
    match dialog.clone() {
        ActiveDialog::Change(op) => {
            rsx! {
                Dialog {
                    WeeUrgeUpdate { op, on_cancel: on_close, on_save: on_change }
                }
            }
        }
        ActiveDialog::Delete(wee_urge) => {
            rsx! {
                Dialog {
                    WeeUrgeDelete { wee_urge, on_cancel: on_close, on_delete }
                }
            }
        }
        ActiveDialog::Idle => {
            rsx! {}
        }
    }
}
