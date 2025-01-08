use chrono::Local;
use dioxus::prelude::*;

use crate::{
    forms::{
        validate_barcode, validate_brand, validate_comments, validate_consumable_unit,
        validate_maybe_date_time, validate_name, CancelButton, Dialog, EditError, FieldValue,
        InputBoolean, InputConsumable, InputMaybeDateTime, InputSelect, InputString, InputTextArea,
        Saving, SubmitButton, ValidationError,
    },
    functions::consumables::{
        create_consumable, create_nested_consumable, delete_consumable, delete_nested_consumable,
        get_child_consumables, update_consumable,
    },
    models::{
        Consumable, ConsumableUnit, Maybe, MaybeDateTime, MaybeString, NestedConsumableId,
        NewConsumable, NewNestedConsumable, UpdateConsumable,
    },
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Operation {
    Create {},
    Update { consumable: Consumable },
}

#[derive(Debug, Clone)]
struct Validate {
    name: Memo<Result<String, ValidationError>>,
    brand: Memo<Result<MaybeString, ValidationError>>,
    barcode: Memo<Result<MaybeString, ValidationError>>,
    is_organic: Memo<Result<bool, ValidationError>>,
    unit: Memo<Result<ConsumableUnit, ValidationError>>,
    comments: Memo<Result<MaybeString, ValidationError>>,
    created: Memo<Result<MaybeDateTime, ValidationError>>,
    destroyed: Memo<Result<MaybeDateTime, ValidationError>>,
}

async fn do_save(op: &Operation, validate: &Validate) -> Result<Consumable, EditError> {
    let name = validate.name.read().clone()?;
    let brand = validate.brand.read().clone()?;
    let barcode = validate.barcode.read().clone()?;
    let is_organic = validate.is_organic.read().clone()?;
    let unit = validate.unit.read().clone()?;
    let comments = validate.comments.read().clone()?;
    let created: MaybeDateTime = validate.created.read().clone()?;
    let destroyed: MaybeDateTime = validate.destroyed.read().clone()?;

    match op {
        Operation::Create {} => {
            let updates = NewConsumable {
                name,
                brand,
                barcode,
                is_organic,
                unit,
                comments,
                created,
                destroyed,
            };
            create_consumable(updates).await.map_err(EditError::Server)
        }
        Operation::Update { consumable } => {
            let updates = UpdateConsumable {
                name: Some(name),
                brand: Some(brand),
                barcode: Some(barcode),
                is_organic: Some(is_organic),
                unit: Some(unit),
                comments: Some(comments),
                created: Some(created),
                destroyed: Some(destroyed),
            };
            update_consumable(consumable.id, updates)
                .await
                .map_err(EditError::Server)
        }
    }
}

#[component]
pub fn ChangeConsumable(
    op: Operation,
    on_cancel: Callback,
    on_save: Callback<Consumable>,
) -> Element {
    let name = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { consumable } => consumable.name.as_string(),
    });

    let brand = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { consumable } => consumable.brand.as_string(),
    });

    let barcode = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { consumable } => consumable.barcode.as_string(),
    });

    let is_organic = use_signal(|| match &op {
        Operation::Create { .. } => false,
        Operation::Update { consumable } => consumable.is_organic,
    });

    let unit = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { consumable } => consumable.unit.as_string(),
    });

    let comments = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { consumable } => consumable.comments.as_string(),
    });

    let created = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { consumable } => consumable.created.as_string(),
    });

    let destroyed = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { consumable } => consumable.destroyed.as_string(),
    });

    let validate = Validate {
        name: use_memo(move || validate_name(&name())),
        brand: use_memo(move || validate_brand(&brand())),
        barcode: use_memo(move || validate_barcode(&barcode())),
        is_organic: use_memo(move || Ok(is_organic())),
        unit: use_memo(move || validate_consumable_unit(&unit())),
        comments: use_memo(move || validate_comments(&comments())),
        created: use_memo(move || validate_maybe_date_time(&created())),
        destroyed: use_memo(move || validate_maybe_date_time(&destroyed())),
    };

    let mut saving = use_signal(|| Saving::No);

    // disable form while waiting for response
    let disabled = use_memo(move || saving.read().is_saving());
    let disabled_save = use_memo(move || {
        validate.name.read().is_err()
            || validate.brand.read().is_err()
            || validate.barcode.read().is_err()
            || validate.is_organic.read().is_err()
            || validate.unit.read().is_err()
            || validate.comments.read().is_err()
            || validate.created.read().is_err()
            || validate.destroyed.read().is_err()
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
                    Operation::Create { .. } => "Create Consumable".to_string(),
                    Operation::Update { consumable } => {
                        format!("Edit Consumable {}", consumable.name)
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
                InputString {
                    id: "name",
                    label: "Name",
                    value: name,
                    validate: validate.name,
                    disabled,
                }
                InputString {
                    id: "brand",
                    label: "Brand",
                    value: brand,
                    validate: validate.brand,
                    disabled,
                }
                InputString {
                    id: "barcode",
                    label: "Barcode",
                    value: barcode,
                    validate: validate.barcode,
                    disabled,
                }
                InputBoolean {
                    id: "is_organic",
                    label: "Is Organic",
                    value: is_organic,
                    disabled,
                }
                InputSelect {
                    id: "unit",
                    label: "Unit",
                    value: unit,
                    validate: validate.unit,
                    options: ConsumableUnit::options(),
                    disabled,
                }
                InputTextArea {
                    id: "comments",
                    label: "Comments",
                    value: comments,
                    validate: validate.comments,
                    disabled,
                }
                InputMaybeDateTime {
                    id: "created",
                    label: "Created",
                    value: created,
                    validate: validate.created,
                    disabled,
                }
                InputMaybeDateTime {
                    id: "destroyed",
                    label: "Destroyed",
                    value: destroyed,
                    validate: validate.destroyed,
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
pub fn DeleteConsumable(
    consumable: Consumable,
    on_cancel: Callback,
    on_delete: Callback<Consumable>,
) -> Element {
    let mut saving = use_signal(|| Saving::No);

    let disabled = use_memo(move || saving.read().is_saving());

    let consumable_clone = consumable.clone();
    let on_save = use_callback(move |()| {
        let consumable_clone = consumable_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            match delete_consumable(consumable_clone.id).await {
                Ok(_) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_delete(consumable_clone.clone());
                }
                Err(err) => saving.set(Saving::Finished(Err(EditError::Server(err)))),
            }
        });
    });

    rsx! {
        Dialog {
            h3 { class: "text-lg font-bold",
                "Delete consumable "
                {consumable.id.to_string()}
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ActiveDialog {
    Change(Operation),
    Delete(Consumable),
    Details(Consumable),
    Nested(Consumable),
    Idle,
}

#[component]
pub fn ConsumableDialog(
    dialog: Signal<ActiveDialog>,
    on_change: Callback<Consumable>,
    on_delete: Callback<Consumable>,
) -> Element {
    match dialog() {
        ActiveDialog::Idle => rsx! {},
        ActiveDialog::Change(op) => {
            rsx! {
                ChangeConsumable {
                    op,
                    on_cancel: move || dialog.set(ActiveDialog::Idle),
                    on_save: move |consumable| {
                        on_change(consumable);
                        dialog.set(ActiveDialog::Idle);
                    },
                }
            }
        }
        ActiveDialog::Delete(consumable) => {
            rsx! {
                DeleteConsumable {
                    consumable,
                    on_cancel: move || dialog.set(ActiveDialog::Idle),
                    on_delete: move |consumable| {
                        on_delete(consumable);
                        dialog.set(ActiveDialog::Idle);
                    },
                }
            }
        }
        ActiveDialog::Details(consumable) => {
            rsx! {
                ConsumableDetails {
                    consumable,
                    on_close: move || dialog.set(ActiveDialog::Idle),
                }
            }
        }
        ActiveDialog::Nested(consumable) => {
            rsx! {
                ConsumableNested {
                    consumable,
                    on_close: move || dialog.set(ActiveDialog::Idle),
                }
            }
        }
    }
}

#[component]
pub fn ConsumableDetails(consumable: Consumable, on_close: Callback<()>) -> Element {
    rsx! {
        Dialog {
            h3 { class: "text-lg font-bold",
                "Consumable "
                {consumable.name.clone()}
            }

            div { class: "p-4",
                table { class: "table table-striped",
                    tbody {
                        tr {
                            td { "Name" }
                            td { {consumable.name} }
                        }
                        tr {
                            td { "Brand" }
                            td {
                                if let MaybeString::Some(brand) = &consumable.brand {
                                    {brand.clone()}
                                }
                            }
                        }
                        tr {
                            td { "Barcode" }
                            td {
                                if let MaybeString::Some(barcode) = &consumable.barcode {
                                    {barcode.clone()}
                                }
                            }
                        }
                        tr {
                            td { "Is Organic" }
                            td { {consumable.is_organic.to_string()} }
                        }
                        tr {
                            td { "Unit" }
                            td { {consumable.unit.to_string()} }
                        }
                        tr {
                            td { "Comments" }
                            td {
                                if let MaybeString::Some(comments) = &consumable.comments {
                                    {comments.to_string()}
                                }
                            }
                        }
                        tr {
                            td { "Created" }
                            td {
                                if let MaybeDateTime::Some(created) = consumable.created {
                                    {created.with_timezone(&Local).to_string()}
                                } else {
                                    "Not Created"
                                }
                            }
                        }
                        tr {
                            td { "Destroyed" }
                            td {
                                if let MaybeDateTime::Some(destroyed) = consumable.destroyed {
                                    {destroyed.with_timezone(&Local).to_string()}
                                } else {
                                    "Not destroyed"
                                }
                            }
                        }
                        tr {
                            td { "Created At" }
                            td { {consumable.created_at.with_timezone(&Local).to_string()} }
                        }
                        tr {
                            td { "Updated At" }
                            td { {consumable.updated_at.with_timezone(&Local).to_string()} }
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
pub fn ConsumableNested(consumable: Consumable, on_close: Callback<()>) -> Element {
    let mut nested_consumables =
        use_resource(move || async move { get_child_consumables(consumable.id).await });

    let consumable_clone = consumable.clone();
    let mut state = use_signal(|| State::Idle);

    let mut add_value = use_signal(|| None);
    let add_consumable = use_callback(move |child: Consumable| {
        let consumable = consumable_clone.clone();
        spawn(async move {
            state.set(State::Saving);
            let updates = NewNestedConsumable {
                id: NestedConsumableId::new(consumable.id, child.id),
                quantity: Maybe::None,
                liquid_mls: Maybe::None,
                comments: Maybe::None,
            };
            let result = create_nested_consumable(updates).await.map(|_nested| ());
            if result.is_ok() {
                nested_consumables.restart();
            }
            state.set(State::Finished(result));
        });
    });

    let consumable_clone = consumable.clone();
    let remove_consumable = use_callback(move |child: Consumable| {
        let consumable = consumable_clone.clone();
        spawn(async move {
            state.set(State::Saving);
            let id = NestedConsumableId::new(consumable.id, child.id);
            let result = delete_nested_consumable(id).await;
            state.set(State::Finished(result));
            nested_consumables.restart();
        });
    });

    let mut selected_consumable = use_signal(|| None);

    let disabled = use_memo(move || State::Saving == *state.read());

    rsx! {
        Dialog {
            h3 { class: "text-lg font-bold",
                "Consumable Ingredients "
                {consumable.name.clone()}
            }

            match nested_consumables() {
                Some(Ok(nested_consumables)) => {
                    rsx! {
                        div { class: "p-4",
                            table { class: "table table-striped",
                                thead {
                                    tr {
                                        th { "Name" }
                                        th { "Brand" }
                                        th { "Comments" }
                                        th { "Created" }
                                        th { "Destroyed" }
                                    }
                                }
                                tbody {
                                    for (nested , consumable) in nested_consumables {
                                        tr {
                                            onclick: move |_| {
                                                selected_consumable.set(Some(consumable.clone()));
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
                                                if let Maybe::Some(created) = consumable.created {
                                                    {created.with_timezone(&Local).to_string()}
                                                } else {
                                                    "Not Created"
                                                }
                                            }
                                            td {
                                                if let Maybe::Some(destroyed) = consumable.destroyed {
                                                    {destroyed.with_timezone(&Local).to_string()}
                                                } else {
                                                    "Not destroyed"
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
            if let Some(selected) = selected_consumable() {
                div { class: "p-4",
                    p { "Form goes here" }
                    button {
                        class: "btn btn-secondary m-1",
                        onclick: move |_| {
                            remove_consumable(selected.clone());
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
