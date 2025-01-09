use chrono::{DateTime, Local, TimeDelta, Utc};
use dioxus::prelude::*;

use crate::{
    forms::{
        validate_comments, validate_consumable_millilitres, validate_consumable_quantity,
        validate_date_time, validate_duration, CancelButton, Dialog, EditError, FieldValue,
        InputConsumable, InputDateTime, InputDuration, InputNumber, InputTextArea, Saving,
        SubmitButton, ValidationError,
    },
    functions::consumptions::{
        create_consumption, create_consumption_consumable, delete_consumption,
        delete_consumption_consumable, get_child_consumables, update_consumption,
        update_consumption_consumable,
    },
    models::{
        Consumable, Consumption, ConsumptionConsumable, ConsumptionConsumableId, Maybe, MaybeF64,
        MaybeString, NewConsumption, NewConsumptionConsumable, UpdateConsumption,
        UpdateConsumptionConsumable, UserId,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Create { user_id: UserId },
    Update { consumption: Consumption },
}

#[derive(Debug, Clone)]
struct Validate {
    time: Memo<Result<DateTime<Utc>, ValidationError>>,
    duration: Memo<Result<TimeDelta, ValidationError>>,
    liquid_mls: Memo<Result<MaybeF64, ValidationError>>,
    comments: Memo<Result<MaybeString, ValidationError>>,
}

async fn do_save(op: &Operation, validate: &Validate) -> Result<Consumption, EditError> {
    let time = validate.time.read().clone()?;
    let duration = validate.duration.read().clone()?;
    let liquid_mls = validate.liquid_mls.read().clone()?;
    let comments = validate.comments.read().clone()?;

    match op {
        Operation::Create { user_id } => {
            let updates = NewConsumption {
                user_id: *user_id,
                time,
                duration,
                liquid_mls,
                comments,
            };
            create_consumption(updates).await.map_err(EditError::Server)
        }
        Operation::Update { consumption } => {
            let updates = UpdateConsumption {
                user_id: None,
                time: Some(time),
                duration: Some(duration),
                liquid_mls: Some(liquid_mls),
                comments: Some(comments),
            };
            update_consumption(consumption.id, updates)
                .await
                .map_err(EditError::Server)
        }
    }
}

#[component]
pub fn ChangeConsumption(
    op: Operation,
    on_cancel: Callback,
    on_save: Callback<Consumption>,
) -> Element {
    let time = use_signal(|| match &op {
        Operation::Create { .. } => Utc::now().with_timezone(&Local).to_rfc3339(),
        Operation::Update { consumption } => consumption.time.as_string(),
    });

    let duration = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { consumption } => consumption.duration.as_string(),
    });

    let liquid_mls = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { consumption } => consumption.liquid_mls.as_string(),
    });

    let comments = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { consumption } => consumption.comments.as_string(),
    });

    let validate = Validate {
        time: use_memo(move || validate_date_time(&time())),
        duration: use_memo(move || validate_duration(&duration())),
        liquid_mls: use_memo(move || validate_consumable_millilitres(&liquid_mls())),
        comments: use_memo(move || validate_comments(&comments())),
    };

    let mut saving = use_signal(|| Saving::No);

    // disable form while waiting for response
    let disabled = use_memo(move || saving.read().is_saving());
    let disabled_save = use_memo(move || {
        validate.time.read().is_err()
            || validate.duration.read().is_err()
            || validate.liquid_mls.read().is_err()
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

        Dialog {
            h3 { class: "text-lg font-bold",
                match &op {
                    Operation::Create { .. } => "Create Consumption".to_string(),
                    Operation::Update { consumption } => {
                        format!("Edit consumption {}", consumption.id)
                    }
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
                    validate: validate.duration,
                    disabled,
                }
                InputNumber {
                    id: "liquid_mls",
                    label: "Liquid Millilitres",
                    value: liquid_mls,
                    validate: validate.liquid_mls,
                    disabled,
                }
                InputTextArea {
                    id: "comments",
                    label: "Comments",
                    value: comments,
                    validate: validate.comments,
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
            }
        }
    }
}

#[component]
pub fn DeleteConsumption(
    consumption: Consumption,
    on_cancel: Callback,
    on_delete: Callback<Consumption>,
) -> Element {
    let mut saving = use_signal(|| Saving::No);

    let disabled = use_memo(move || saving.read().is_saving());

    let consumption_clone = consumption.clone();
    let on_save = use_callback(move |()| {
        let consumption = consumption_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            match delete_consumption(consumption.id).await {
                Ok(_) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_delete(consumption.clone());
                }
                Err(err) => saving.set(Saving::Finished(Err(EditError::Server(err)))),
            }
        });
    });

    rsx! {
        Dialog {
            h3 { class: "text-lg font-bold",
                "Delete consumption "
                {consumption.id.to_string()}
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

const CONSUMPTION_SVG: Asset = asset!("/assets/consumption.svg");

#[component]
pub fn consumption_icon() -> Element {
    rsx! {
        img {
            class: "w-10 invert inline-block",
            alt: "Consumption",
            src: CONSUMPTION_SVG,
        }
    }
}

pub fn div_rem(a: i64, b: i64) -> (i64, i64) {
    (a / b, a % b)
}

#[component]
pub fn consumption_duration(duration: chrono::TimeDelta) -> Element {
    let seconds = duration.num_seconds();
    let (minutes, seconds) = div_rem(seconds, 60);
    let (hours, minutes) = div_rem(minutes, 60);
    let (days, hours) = div_rem(hours, 24);

    let text = if duration.num_seconds() < 60 {
        format!("{seconds} seconds")
    } else if duration.num_minutes() < 60 {
        format!("{minutes} minutes + {seconds} seconds")
    } else if duration.num_hours() < 24 {
        format!("{hours} hours + {minutes} minutes")
    } else {
        format!("{days} days + {hours} hours")
    };

    rsx! {
        if duration.num_minutes() < 5 {
            span { class: "text-error", {text} }
        } else if duration.num_minutes() < 20 {
            span { class: "text-success", {text} }
        } else if duration.num_minutes() < 30 {
            span { class: "text-warning", {text} }
        } else {
            span { class: "text-error", {text} }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveDialog {
    Change(Operation),
    Delete(Consumption),
    Details(Consumption),
    Consumption(Consumption),
}

#[component]
pub fn ConsumptionDialog(
    dialog: ActiveDialog,
    on_close: Callback,
    on_change: Callback<Consumption>,
    on_delete: Callback<Consumption>,
) -> Element {
    match dialog {
        ActiveDialog::Change(op) => {
            rsx! {
                ChangeConsumption {
                    op,
                    on_cancel: on_close,
                    on_save: move |consumption| {
                        on_change(consumption);
                    },
                }
            }
        }
        ActiveDialog::Delete(consumption) => {
            rsx! {
                DeleteConsumption {
                    consumption,
                    on_cancel: on_close,
                    on_delete: move |consumption| {
                        on_delete(consumption);
                    },
                }
            }
        }
        ActiveDialog::Details(consumption) => {
            rsx! {
                ConsumptionDetails { consumption, on_close }
            }
        }
        ActiveDialog::Consumption(consumption) => {
            rsx! {
                ConsumableConsumption { consumption, on_close }
            }
        }
    }
}

#[component]
pub fn ConsumptionDetails(consumption: Consumption, on_close: Callback<()>) -> Element {
    rsx! {
        Dialog {
            h3 { class: "text-lg font-bold",
                "Consumable "
                {consumption.id.to_string()}
            }

            div { class: "p-4",
                table { class: "table table-striped",
                    tbody {
                        tr {
                            td { "Event" }
                            td { consumption_icon {} }
                        }
                        tr {
                            td { "ID" }
                            td { {consumption.id.to_string()} }
                        }
                        tr {
                            td { "Time" }
                            td { {consumption.time.with_timezone(&Local).to_string()} }
                        }
                        tr {
                            td { "Duration" }
                            td { {consumption.duration.to_string()} }
                        }
                        tr {
                            td { "Liquid Millilitres" }
                            td { {consumption.liquid_mls.as_string()} }
                        }
                        tr {
                            td { "Created At" }
                            td { {consumption.created_at.with_timezone(&Local).to_string()} }
                        }
                        tr {
                            td { "Updated At" }
                            td { {consumption.updated_at.with_timezone(&Local).to_string()} }
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
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum State {
    Idle,
    Saving,
    Finished(Result<(), ServerFnError>),
}

#[component]
pub fn ConsumableConsumption(consumption: Consumption, on_close: Callback<()>) -> Element {
    let mut selected_consumable = use_signal(|| None);

    let mut consumption_consumables =
        use_resource(move || async move { get_child_consumables(consumption.id).await });

    let consumable_clone = consumption.clone();
    let mut state = use_signal(|| State::Idle);

    let mut add_value = use_signal(|| None);
    let add_consumable = use_callback(move |child: Consumable| {
        let consumable = consumable_clone.clone();
        spawn(async move {
            state.set(State::Saving);
            let updates = NewConsumptionConsumable {
                id: ConsumptionConsumableId::new(consumable.id, child.id),
                quantity: Maybe::None,
                liquid_mls: Maybe::None,
                comments: Maybe::None,
            };
            let result = create_consumption_consumable(updates).await;
            if let Ok(nested) = result.clone() {
                selected_consumable.set(Some((nested, child.clone())));
                consumption_consumables.restart();
            }
            let result = result.map(|_nested| ());
            state.set(State::Finished(result));
        });
    });

    let remove_consumable = use_callback(move |child: ConsumptionConsumable| {
        spawn(async move {
            state.set(State::Saving);
            let result = delete_consumption_consumable(child.id).await;
            state.set(State::Finished(result));
            consumption_consumables.restart();
        });
    });

    let disabled = use_memo(move || State::Saving == *state.read());

    rsx! {
        Dialog {
            h3 { class: "text-lg font-bold",
                "Consumable Ingredients "
                {consumption.id.to_string()}
            }

            match consumption_consumables() {
                Some(Ok(consumption_consumables)) => {
                    rsx! {
                        div { class: "p-4",
                            table { class: "table table-striped",
                                thead {
                                    tr {
                                        th { "Name" }
                                        th { "Brand" }
                                        th { "Comments" }
                                        th { "Quantity" }
                                    }
                                }
                                tbody {
                                    for (nested , consumable) in consumption_consumables {
                                        tr {
                                            onclick: move |_| {
                                                selected_consumable.set(Some((nested.clone(), consumable.clone())));
                                            },
                                            td { {consumable.name.clone()} }
                                            td {
                                                if let Maybe::Some(brand) = &consumable.brand {
                                                    {brand.clone()}
                                                }
                                            }
                                            td {
                                                if let Maybe::Some(comments) = &nested.comments {
                                                    {comments.to_string()}
                                                }
                                            }
                                            td {
                                                if let Maybe::Some(quantity) = nested.quantity {
                                                    {quantity.to_string()}
                                                    {consumable.unit.to_string()}
                                                }
                                                if let Maybe::Some(liquid_mls) = nested.liquid_mls {
                                                    {liquid_mls.to_string()}
                                                    "ml"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Some(Err(err)) => {
                    rsx! {
                        div { class: "p-4",
                            "Error: "
                            {err.to_string()}
                        }
                    }
                }
                None => {
                    rsx! {
                        div { class: "p-4", "Loading..." }
                    }
                }
            }

            match state() {
                State::Saving => {
                    rsx! {
                        div { class: "alert alert-info", "Saving..." }
                    }
                }
                State::Finished(Ok(())) => {
                    rsx! {
                        div { class: "alert alert-success", "Saved!" }
                    }
                }
                State::Finished(Err(err)) => {
                    rsx! {
                        div { class: "alert alert-error",
                            "Error: "
                            {err.to_string()}
                        }
                    }
                }
                State::Idle => {
                    rsx! {}
                }
            }
            if let Some((sel_consumption, sel_consumable)) = selected_consumable() {
                div { class: "p-4",
                    p {
                        "Selected: "
                        {sel_consumable.name.clone()}
                    }
                    ConsumableConsumptionForm {
                        consumption: sel_consumption.clone(),
                        consumable: sel_consumable.clone(),
                        on_cancel: move |_| {
                            selected_consumable.set(None);
                        },
                        on_save: move |_consumption| {
                            selected_consumable.set(None);
                            consumption_consumables.restart();
                        },
                    }
                    button {
                        class: "btn btn-secondary m-1",
                        onclick: move |_| {
                            selected_consumable.set(None);
                            remove_consumable(sel_consumption.clone());
                        },
                        "Delete"
                    }
                }
            } else {
                div { class: "p-4",
                    InputConsumable {
                        id: "consumable",
                        label: "Add Consumable",
                        value: add_value,
                        on_change: move |value| {
                            if let Some(value) = value {
                                add_consumable(value);
                                add_value.set(None);
                            }
                        },
                        disabled,
                    }
                }
            }


            div { class: "p-4",
                button {
                    class: "btn btn-primary m-1",
                    onclick: move |_| {
                        on_close(());
                    },
                    "Close"
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct ValidateConsumption {
    quantity: Memo<Result<MaybeF64, ValidationError>>,
    liquid_mls: Memo<Result<MaybeF64, ValidationError>>,
    comments: Memo<Result<MaybeString, ValidationError>>,
}

async fn do_save_consumption(
    consumption: ConsumptionConsumable,
    validate: &ValidateConsumption,
) -> Result<ConsumptionConsumable, EditError> {
    let quantity = validate.quantity.read().clone()?;
    let liquid_mls = validate.liquid_mls.read().clone()?;
    let comments = validate.comments.read().clone()?;

    let updates = UpdateConsumptionConsumable {
        quantity: Some(quantity),
        liquid_mls: Some(liquid_mls),
        comments: Some(comments),
    };
    update_consumption_consumable(consumption.id, updates)
        .await
        .map_err(EditError::Server)
}

#[component]
fn ConsumableConsumptionForm(
    consumption: ConsumptionConsumable,
    consumable: Consumable,
    on_cancel: Callback<()>,
    on_save: Callback<ConsumptionConsumable>,
) -> Element {
    let quantity = use_signal(|| consumption.quantity.as_string());
    let liquid_mls = use_signal(|| consumption.liquid_mls.as_string());
    let comments = use_signal(|| consumption.comments.as_string());

    let validate = ValidateConsumption {
        quantity: use_memo(move || validate_consumable_quantity(&quantity())),
        liquid_mls: use_memo(move || validate_consumable_millilitres(&liquid_mls())),
        comments: use_memo(move || validate_comments(&comments())),
    };

    let mut saving = use_signal(|| Saving::No);

    let disabled = use_memo(move || saving.read().is_saving());
    let disabled_save = use_memo(move || {
        validate.quantity.read().is_err()
            || validate.liquid_mls.read().is_err()
            || validate.comments.read().is_err()
            || disabled()
    });

    let consumption_clone = consumption.clone();
    let validate_clone = validate.clone();
    let on_save = use_callback(move |()| {
        let consumption = consumption_clone.clone();
        let validate = validate_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            let result = do_save_consumption(consumption, &validate).await;
            match result {
                Ok(consumption) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_save(consumption);
                }
                Err(err) => saving.set(Saving::Finished(Err(err))),
            }
        });
    });

    rsx! {
        form {
            novalidate: true,
            action: "javascript:void(0)",
            method: "dialog",
            onkeyup: move |event| {
                if event.key() == Key::Escape {
                    on_cancel(());
                }
            },
            InputNumber {
                id: "quantity",
                label: format!("Quantity ({})", consumable.unit.to_string()),
                value: quantity,
                validate: validate.quantity,
                disabled,
            }
            InputNumber {
                id: "liquid_mls",
                label: "Liquid Millilitres",
                value: liquid_mls,
                validate: validate.liquid_mls,
                disabled,
            }
            InputTextArea {
                id: "comments",
                label: "Comments",
                value: comments,
                validate: validate.comments,
                disabled,
            }

            SubmitButton {
                disabled: disabled_save,
                on_save: move |_| on_save(()),
                title: "Save",
            }
            CancelButton { on_cancel: move |_| on_cancel(()), title: "Cancel" }
        }
    }
}
