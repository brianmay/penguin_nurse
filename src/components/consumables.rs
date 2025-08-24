use std::{num::ParseIntError, str::FromStr};

use chrono::Local;
use dioxus::prelude::*;
use tap::Pipe;
use thiserror::Error;

use crate::{
    forms::{
        Barcode, Dialog, EditError, FieldValue, FormCloseButton, FormDeleteButton, FormEditButton,
        FormSaveCancelButton, InputBoolean, InputConsumable, InputMaybeDateTime, InputNumber,
        InputSelect, InputString, InputTextArea, Saving, ValidationError, validate_barcode,
        validate_brand, validate_comments, validate_consumable_millilitres,
        validate_consumable_quantity, validate_consumable_unit, validate_maybe_date_time,
        validate_name,
    },
    functions::consumables::{
        create_consumable, create_nested_consumable, delete_consumable, delete_nested_consumable,
        get_child_consumables, update_consumable, update_nested_consumable,
    },
    models::{
        ChangeConsumable, ChangeNestedConsumable, Consumable, ConsumableId, ConsumableItem,
        ConsumableUnit, Maybe, MaybeDateTime, MaybeF64, MaybeString, NestedConsumable,
        NestedConsumableId, NewConsumable, NewNestedConsumable,
    },
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Operation {
    Create,
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
        Operation::Create => {
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
            let changes = ChangeConsumable {
                name: Some(name),
                brand: Some(brand),
                barcode: Some(barcode),
                is_organic: Some(is_organic),
                unit: Some(unit),
                comments: Some(comments),
                created: Some(created),
                destroyed: Some(destroyed),
            };
            update_consumable(consumable.id, changes)
                .await
                .map_err(EditError::Server)
        }
    }
}

#[component]
pub fn ConsumableUpdate(
    op: Operation,
    on_cancel: Callback,
    on_save: Callback<Consumable>,
) -> Element {
    let name = use_signal(|| match &op {
        Operation::Create => String::new(),
        Operation::Update { consumable } => consumable.name.as_string(),
    });

    let brand = use_signal(|| match &op {
        Operation::Create => String::new(),
        Operation::Update { consumable } => consumable.brand.as_string(),
    });

    let barcode = use_signal(|| match &op {
        Operation::Create => String::new(),
        Operation::Update { consumable } => consumable.barcode.as_string(),
    });

    let is_organic = use_signal(|| match &op {
        Operation::Create => false,
        Operation::Update { consumable } => consumable.is_organic,
    });

    let unit = use_signal(|| match &op {
        Operation::Create => String::new(),
        Operation::Update { consumable } => consumable.unit.as_string(),
    });

    let comments = use_signal(|| match &op {
        Operation::Create => String::new(),
        Operation::Update { consumable } => consumable.comments.as_string(),
    });

    let created = use_signal(|| match &op {
        Operation::Create => String::new(),
        Operation::Update { consumable } => consumable.created.as_string(),
    });

    let destroyed = use_signal(|| match &op {
        Operation::Create => String::new(),
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

        h3 { class: "text-lg font-bold",
            match &op {
                Operation::Create => "Create Consumable".to_string(),
                Operation::Update { consumable } => format!("Edit Consumable {}", consumable.name),
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
            Barcode {
                barcode: barcode,
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

            FormSaveCancelButton {
                disabled: disabled_save,
                on_save: move |()| on_save(()),
                on_cancel: move |()| on_cancel(()),
                title: match &op {
                    Operation::Create => "Create",
                    Operation::Update { .. } => "Save",
                },
                saving
            }
        }
    }
}

#[component]
pub fn ConsumableDelete(
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
        h3 { class: "text-lg font-bold",
            "Delete consumable "
            {consumable.name.clone()}
        }
        p { class: "py-4", "Press ESC key or click the button below to close" }
        ConsumableSummary { consumable: consumable.clone() }
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
                saving
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ActiveDialog {
    Change(Operation),
    Delete(Consumable),
    Ingredients(Consumable),
    NestedIngredient(Consumable, Consumable),
    NestedIngredients(Consumable, Consumable),
    Idle,
}

#[derive(Error, Debug)]
pub enum ListDialogReferenceError {
    #[error("Invalid integer")]
    ParseIntError(#[from] ParseIntError),

    #[error("Invalid reference")]
    ReferenceError,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ListDialogReference {
    Create,
    UpdateBasic {
        consumable_id: ConsumableId,
    },
    UpdateIngredients {
        consumable_id: ConsumableId,
    },
    IngredientUpdateBasic {
        parent_id: ConsumableId,
        consumable_id: ConsumableId,
    },
    IngredientUpdateIngredients {
        parent_id: ConsumableId,
        consumable_id: ConsumableId,
    },
    Delete {
        consumable_id: ConsumableId,
    },
    #[default]
    Idle,
}

impl FromStr for ListDialogReference {
    type Err = ListDialogReferenceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split("-").collect::<Vec<_>>();
        match split[..] {
            ["create"] => Self::Create,
            ["update", id] => {
                let consumable_id = ConsumableId::new(id.parse()?);
                Self::UpdateBasic { consumable_id }
            }
            ["delete", id] => {
                let consumable_id = ConsumableId::new(id.parse()?);
                Self::Delete { consumable_id }
            }
            ["ingredients", id] => {
                let consumable_id = ConsumableId::new(id.parse()?);
                Self::UpdateIngredients { consumable_id }
            }
            ["ingredients_update", parent_id, id] => {
                let parent_id = ConsumableId::new(parent_id.parse()?);
                let consumable_id = ConsumableId::new(id.parse()?);
                Self::IngredientUpdateBasic {
                    parent_id,
                    consumable_id,
                }
            }
            ["ingredients_ingredients", parent_id, id] => {
                let parent_id = ConsumableId::new(parent_id.parse()?);
                let consumable_id = ConsumableId::new(id.parse()?);
                Self::IngredientUpdateIngredients {
                    parent_id,
                    consumable_id,
                }
            }
            [""] | [] => Self::Idle,
            _ => return Err(ListDialogReferenceError::ReferenceError),
        }
        .pipe(Ok)
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for ListDialogReference {
    fn to_string(&self) -> String {
        match self {
            ListDialogReference::Create => "create".to_string(),
            ListDialogReference::UpdateBasic { consumable_id } => format!("update-{consumable_id}"),
            ListDialogReference::UpdateIngredients { consumable_id } => {
                format!("ingredients-{consumable_id}")
            }
            ListDialogReference::IngredientUpdateBasic {
                parent_id,
                consumable_id,
            } => {
                format!("ingredients_update-{parent_id}-{consumable_id}")
            }
            ListDialogReference::IngredientUpdateIngredients {
                parent_id,
                consumable_id,
            } => {
                format!("ingredients_ingredients-{parent_id}-{consumable_id}")
            }
            ListDialogReference::Delete { consumable_id } => format!("delete-{consumable_id}"),
            ListDialogReference::Idle => String::new(),
        }
    }
}

#[component]
pub fn ConsumableDialog(
    dialog: ReadOnlySignal<ActiveDialog>,
    on_change: Callback<Consumable>,
    on_change_ingredients: Callback<Consumable>,
    on_delete: Callback<Consumable>,
    show_update_basic: Callback<Consumable>,
    show_update_ingredients: Callback<Consumable>,
    show_ingredient_update_basic: Callback<(Consumable, Consumable)>,
    show_ingredient_update_ingredients: Callback<(Consumable, Consumable)>,
    on_close: Callback<()>,
) -> Element {
    match dialog() {
        ActiveDialog::Idle => rsx! {},
        ActiveDialog::Change(op) => {
            rsx! {
                Dialog {
                    ConsumableUpdate {
                        op,
                        on_cancel: on_close,
                        on_save: move |consumable: Consumable| {
                            on_change(consumable.clone());
                            show_update_ingredients(consumable)
                        },
                    }
                }
            }
        }
        ActiveDialog::Delete(consumable) => {
            rsx! {
                Dialog {
                    ConsumableDelete {
                        consumable,
                        on_cancel: on_close,
                        on_delete: move |consumable| {
                            on_delete(consumable);
                            on_close(())
                        },
                    }
                }
            }
        }
        ActiveDialog::Ingredients(consumable) => {
            rsx! {
                Dialog {
                    // Closures must be used here or it will sometimes panic.
                    // See https://github.com/DioxusLabs/dioxus/discussions/4534
                    ConsumableUpdateIngredients {
                        consumable,
                        on_close,
                        on_change: move |param| { on_change_ingredients(param); },
                        show_update_basic,
                        show_ingredient_update_basic: move |param| { show_ingredient_update_basic(param); },
                        show_ingredient_update_ingredients: move |param| { show_ingredient_update_ingredients(param); },
                    }
                }
            }
        }
        ActiveDialog::NestedIngredient(parent, consumable) => {
            let parent_clone_1 = parent.clone();
            let parent_clone_2 = parent.clone();
            rsx! {
                Dialog {
                    ConsumableUpdate {
                        op: Operation::Update { consumable: consumable.clone() },
                        on_cancel: move |()| {
                            show_update_ingredients(parent_clone_1.clone())
                        },
                        on_save: move |consumable: Consumable| {
                            on_change(consumable.clone());
                            show_ingredient_update_ingredients((parent_clone_2.clone(), consumable.clone()));
                        },
                    }
                }
            }
        }
        ActiveDialog::NestedIngredients(parent, consumable) => {
            let parent_clone_1 = parent.clone();
            let parent_clone_2 = parent.clone();
            let parent_clone_3 = parent.clone();
            let parent_clone_4 = parent.clone();
            rsx! {
                Dialog {
                    ConsumableUpdateIngredients {
                        consumable,
                        on_close: move |()| {
                            show_update_ingredients(parent.clone())
                        },
                        on_change: move |_consumable: Consumable| {
                            on_change_ingredients(parent_clone_1.clone());
                        },
                        show_update_basic: move |consumable: Consumable| {
                            show_ingredient_update_basic((parent_clone_2.clone(), consumable));
                        },
                        show_ingredient_update_basic: move |(_parent, consumable): (Consumable, Consumable)| {
                            show_ingredient_update_basic((parent_clone_3.clone(), consumable));
                        },
                        show_ingredient_update_ingredients: move |(_parent, consumable): (Consumable, Consumable)| {
                            show_ingredient_update_ingredients((parent_clone_4.clone(), consumable.clone()));
                        },
                    }
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
pub fn ConsumableUpdateIngredients(
    consumable: ReadOnlySignal<Consumable>,
    on_close: Callback<()>,
    on_change: Callback<Consumable>,
    show_update_basic: Callback<Consumable>,
    show_ingredient_update_basic: Callback<(Consumable, Consumable)>,
    show_ingredient_update_ingredients: Callback<(Consumable, Consumable)>,
) -> Element {
    let mut selected_consumable = use_signal(|| None);

    let mut nested_consumables =
        use_resource(move || async move { get_child_consumables(consumable().id).await });

    use_effect(move || {
        let _trigger = consumable();
        selected_consumable.set(None);
    });

    let consumable = consumable();

    let consumable_clone = consumable.clone();
    let consumable_clone_2 = consumable.clone();
    let consumable_clone_3 = consumable.clone();
    let consumable_clone_4 = consumable.clone();
    let consumable_clone_5 = consumable.clone();
    let consumable_clone_6 = consumable.clone();
    let consumable_clone_7 = consumable.clone();
    let mut state = use_signal(|| State::Idle);

    let mut add_value = use_signal(|| None);
    let add_consumable = use_callback(move |child: Consumable| {
        let consumable = consumable_clone.clone();
        if let Some(Ok(nested_consumables)) = nested_consumables() {
            if let Some(existing) = nested_consumables
                .iter()
                .find(|item| item.consumable.id == child.id)
            {
                selected_consumable.set(Some(existing.clone()));
                return;
            }
        }

        let consumable_clone = consumable_clone_4.clone();
        spawn(async move {
            state.set(State::Saving);
            let updates = NewNestedConsumable {
                id: NestedConsumableId::new(consumable.id, child.id),
                quantity: Maybe::None,
                liquid_mls: Maybe::None,
                comments: Maybe::None,
            };
            let result = create_nested_consumable(updates).await;
            if let Ok(nested) = result.clone() {
                selected_consumable.set(Some(ConsumableItem::new(nested, child.clone())));
                nested_consumables.restart();
            }
            let result = result.map(|_nested| ());
            state.set(State::Finished(result));
            on_change(consumable_clone.clone());
        });
    });

    let remove_consumable = use_callback(move |child: NestedConsumable| {
        let consumable = consumable_clone_5.clone();
        spawn(async move {
            state.set(State::Saving);
            let result = delete_nested_consumable(child.id).await;
            state.set(State::Finished(result));
            nested_consumables.restart();
            on_change(consumable);
        });
    });

    let disabled = use_memo(move || State::Saving == *state.read());

    let is_selected = |item: &ConsumableItem| {
        if let Some(selected) = selected_consumable() {
            selected.consumable.id == item.consumable.id
        } else {
            false
        }
    };

    rsx! {
        h3 { class: "text-lg font-bold",
            "Consumable Ingredients "
            {consumable.name.clone()}
        }

        match nested_consumables() {
            Some(Ok(nested_consumables)) => {
                rsx! {
                    ConsumableSummary { consumable: consumable.clone() }
                    div { class: "p-4",
                        ul {
                            for item in nested_consumables {
                                li {
                                    class: "p-4 mb-1 bg-gray-700 border-2 rounded-lg",
                                    class: if is_selected(&item) { "border-gray-50 text-gray-50" } else { "border-gray-500" },
                                    onclick: move |_| {
                                        selected_consumable.set(Some(item.clone()));
                                    },
                                    ConsumableItemSummary { item: item.clone() }
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
        if let Some(sel) = selected_consumable() {{
            let consumable_clone_1 = sel.consumable.clone();
            let consumable_clone_2 = sel.consumable.clone();
            rsx!{
                div { class: "card bg-gray-800 shadow-xl",
                    div { class: "card-body",
                        h2 {
                            "Selected: "
                            {sel.consumable.name.clone()}
                        }
                        ConsumableNestedForm {
                            nested: sel.nested.clone(),
                            consumable: sel.consumable.clone(),
                            on_cancel: move |_| {
                                selected_consumable.set(None);
                            },
                            on_save: move |_nested| {
                                selected_consumable.set(None);
                                nested_consumables.restart();
                                on_change(consumable_clone_3.clone());
                            },
                        }
                        FormEditButton {
                            title: "Edit",
                            on_edit: move || {
                                show_ingredient_update_basic((consumable_clone_6.clone(), consumable_clone_1.clone()));
                            },
                        }
                        FormEditButton {
                            title: "Ingredients",
                            on_edit: move || {
                                show_ingredient_update_ingredients((consumable_clone_7.clone(), consumable_clone_2.clone()));
                            },
                        }
                        FormDeleteButton {
                            title: "Remove",
                            on_delete: move |_| {
                                selected_consumable.set(None);
                                remove_consumable(sel.nested.clone());
                            },
                        }
                    }
                }
            }
        }} else {
            div { class: "p-4",
                InputConsumable {
                    id: "consumable",
                    label: "Add",
                    value: add_value,
                    on_create: move |value| {
                        add_consumable(value);
                        add_value.set(None);
                    },
                    on_change: move |value| {
                        if let Some(value) = value {
                            add_consumable(value);
                            add_value.set(None);
                        }
                    },
                    disabled,
                }
                FormEditButton {
                    title: "Edit Consumable",
                    on_edit: move |_| show_update_basic(consumable_clone_2.clone()),
                }
                FormCloseButton { on_close, title: "Close" }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct ValidateNested {
    quantity: Memo<Result<MaybeF64, ValidationError>>,
    liquid_mls: Memo<Result<MaybeF64, ValidationError>>,
    comments: Memo<Result<MaybeString, ValidationError>>,
}

async fn do_save_nested(
    nested: NestedConsumable,
    validate: &ValidateNested,
) -> Result<NestedConsumable, EditError> {
    let quantity = validate.quantity.read().clone()?;
    let liquid_mls = validate.liquid_mls.read().clone()?;
    let comments = validate.comments.read().clone()?;

    let updates = ChangeNestedConsumable {
        quantity: Some(quantity),
        liquid_mls: Some(liquid_mls),
        comments: Some(comments),
    };
    update_nested_consumable(nested.id, updates)
        .await
        .map_err(EditError::Server)
}

#[component]
fn ConsumableNestedForm(
    nested: ReadOnlySignal<NestedConsumable>,
    consumable: ReadOnlySignal<Consumable>,
    on_cancel: Callback<()>,
    on_save: Callback<NestedConsumable>,
) -> Element {
    let mut quantity = use_signal(|| nested.read().quantity.as_string());
    let mut liquid_mls = use_signal(|| nested.read().liquid_mls.as_string());
    let mut comments = use_signal(|| nested.read().comments.as_string());

    use_effect(move || {
        let nested = nested.read();
        quantity.set(nested.quantity.as_string());
        liquid_mls.set(nested.liquid_mls.as_string());
        comments.set(nested.comments.as_string());
    });

    let validate = ValidateNested {
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

    let validate_clone = validate.clone();
    let on_save = use_callback(move |()| {
        let validate = validate_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            let result = do_save_nested(nested(), &validate).await;
            match result {
                Ok(nested) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_save(nested);
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
                label: format!("Quantity ({})", consumable.read().unit.to_string()),
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

            FormSaveCancelButton {
                disabled: disabled_save,
                on_save: move |()| on_save(()),
                on_cancel: move |_| on_cancel(()),
                title: "Save",
                saving
            }
        }
    }
}

const ORGANIC_SVG: Asset = asset!("/assets/organic.svg");

#[component]
pub fn ConsumableSummary(consumable: Consumable) -> Element {
    rsx! {
        div {
            div {
                if consumable.is_organic {
                    img {
                        class: "w-5  invert inline-block",
                        alt: "organic",
                        src: ORGANIC_SVG,
                    }
                }
                {consumable.name}
            }
            div {
                if let Maybe::Some(brand) = &consumable.brand {
                    div { {brand.clone()} }
                }
            }
            div {
                span { class: "sm:hidden", "Unit: " }
                {consumable.unit.to_string()}
            }
            div {
                if let Maybe::Some(comments) = &consumable.comments {
                    div { {comments.to_string()} }
                }
            }
            div {
                if let Maybe::Some(created) = &consumable.created {
                    span { class: "sm:hidden", "Created: " }
                    {created.with_timezone(&Local).to_string()}
                }
            }
            div {
                if let Maybe::Some(destroyed) = &consumable.destroyed {
                    span { class: "sm:hidden", "Destroyed: " }
                    {destroyed.with_timezone(&Local).to_string()}
                }
            }
        }

    }
}

#[component]
pub fn ConsumableItemSummary(item: ConsumableItem) -> Element {
    rsx! {
        span {
            if item.consumable.is_organic {
                img {
                    class: "w-5  invert inline-block",
                    alt: "organic",
                    src: ORGANIC_SVG,
                }
            }
            if let Maybe::Some(quantity) = item.nested.quantity {
                span {
                    {quantity.to_string()}
                    {item.consumable.unit.postfix()}
                    " "
                }
            }
            {item.consumable.name.clone()}
            if let Maybe::Some(brand) = &item.consumable.brand {
                ", "
                {brand.clone()}
            }
            if let Maybe::Some(dt) = &item.consumable.created {
                {dt.with_timezone(&Local).format(" %Y-%m-%d").to_string()}
            }
            if let Maybe::Some(comments) = &item.nested.comments {
                " ("
                {comments.to_string()}
                ")"
            }
            if let Maybe::Some(liquid_mls) = item.nested.liquid_mls {
                span {
                    " Liquid: "
                    {liquid_mls.to_string()}
                    "ml"
                }
            }
        }
    }
}

#[component]
pub fn ConsumableItemList(list: Vec<ConsumableItem>, show_links: Option<bool>) -> Element {
    rsx! {
        if !list.is_empty() {
            ul { class: "list-disc ml-4",
                for item in &list {
                    li {
                        ConsumableItemSummary {
                            key: item.id,
                            item: item.clone(),
                        }
                    }
                }
            }
        }
    }
}
