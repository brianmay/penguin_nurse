use chrono::{DateTime, FixedOffset, Local, Utc};
use dioxus::prelude::*;

use crate::{
    components::{
        events::{Markdown, event_date_time_short},
        times::time_delta_to_string,
    },
    forms::{
        Dialog, EditError, FieldValue, FormSaveCancelButton, InputDateTime, InputTextArea, Saving,
        ValidationError, validate_comments, validate_fixed_offset_date_time,
    },
    functions::notes::{create_note, delete_note, update_note},
    models::{ChangeNote, MaybeSet, NewNote, Note, UserId},
};
use classes::classes;

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Create { user_id: UserId },
    Update { note: Note },
}

#[derive(Debug, Clone)]
struct Validate {
    time: Memo<Result<DateTime<FixedOffset>, ValidationError>>,
    comments: Memo<Result<Option<String>, ValidationError>>,
}

async fn do_save(op: &Operation, validate: &Validate) -> Result<Note, EditError> {
    let time = validate.time.read().clone()?;
    let comments = validate.comments.read().clone()?;

    match op {
        Operation::Create { user_id } => {
            let updates = NewNote {
                user_id: *user_id,
                time,
                comments,
            };
            create_note(updates).await.map_err(EditError::Server)
        }
        Operation::Update { note } => {
            let changes = ChangeNote {
                user_id: MaybeSet::NoChange,
                time: MaybeSet::Set(time),
                comments: MaybeSet::Set(comments),
            };
            update_note(note.id, changes)
                .await
                .map_err(EditError::Server)
        }
    }
}

#[component]
pub fn NoteUpdate(op: Operation, on_cancel: Callback, on_save: Callback<Note>) -> Element {
    let time = use_signal(|| match &op {
        Operation::Create { .. } => Utc::now().with_timezone(&Local).fixed_offset().as_string(),
        Operation::Update { note } => note.time.as_string(),
    });

    let comments = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { note } => note.comments.as_string(),
    });

    let validate = Validate {
        time: use_memo(move || validate_fixed_offset_date_time(&time())),
        comments: use_memo(move || validate_comments(&comments())),
    };

    let mut saving = use_signal(|| Saving::No);

    // disable form while waiting for response
    let disabled = use_memo(move || saving.read().is_saving());
    let disabled_save = use_memo(move || {
        validate.time.read().is_err() || validate.comments.read().is_err() || disabled()
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
                Operation::Create { .. } => "Create Note".to_string(),
                Operation::Update { note } => format!("Edit Note {}", note.name()),
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
pub fn NoteDelete(note: Note, on_cancel: Callback, on_delete: Callback<Note>) -> Element {
    let mut saving = use_signal(|| Saving::No);

    let disabled = use_memo(move || saving.read().is_saving());

    let note_clone = note.clone();
    let on_save = use_callback(move |()| {
        let note = note_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            match delete_note(note.id).await {
                Ok(_) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_delete(note.clone());
                }
                Err(err) => saving.set(Saving::Finished(Err(EditError::Server(err)))),
            }
        });
    });

    rsx! {
        h3 { class: "text-lg font-bold",
            "Delete note "
            {note.name()}
        }
        p { class: "py-4", "Press ESC key or click the button below to close" }
        NoteSummary { note: note.clone() }
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

const NOTE_SVG: Asset = asset!("/assets/note.svg");

#[component]
pub fn note_icon() -> Element {
    let alt = note_title();
    let icon = NOTE_SVG;
    rsx! {
        img { alt, src: icon }
    }
}

#[component]
pub fn note_title() -> &'static str {
    "Note"
}

#[component]
pub fn note_duration(duration: chrono::TimeDelta) -> Element {
    let text = time_delta_to_string(duration);

    rsx! {
        span { {text} }
    }
}

#[component]
pub fn note_calories(calories: Option<i32>) -> Element {
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
    Delete(Note),
    #[allow(dead_code)]
    Idle,
}

#[component]
pub fn NoteDialog(
    dialog: ActiveDialog,
    on_close: Callback<()>,
    on_change: Callback<Note>,
    on_delete: Callback<Note>,
) -> Element {
    match dialog {
        ActiveDialog::Change(op) => {
            rsx! {
                Dialog {
                    NoteUpdate { op, on_cancel: on_close, on_save: on_change }
                }
            }
        }
        ActiveDialog::Delete(note) => {
            rsx! {
                Dialog {
                    NoteDelete { note, on_cancel: on_close, on_delete }
                }
            }
        }
        ActiveDialog::Idle => {
            rsx! {}
        }
    }
}

#[component]
pub fn NoteSummary(note: Note) -> Element {
    rsx! {
        div { {note_title()} }
        div {
            event_date_time_short { time: note.time }
        }
        if let Some(comments) = &note.comments {
            Markdown { content: comments.to_string() }
        }
    }
}

#[component]
pub fn NoteDetails(note: Note) -> Element {
    rsx! {
        if let Some(comments) = &note.comments {
            Markdown { content: comments.to_string() }
        }
    }
}
