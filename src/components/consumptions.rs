use chrono::{DateTime, FixedOffset, Local, TimeDelta, Utc};
use dioxus::prelude::*;

use crate::{
    components::{
        consumables::{self, ConsumableUpdate, ConsumableUpgradeIngredients},
        times::time_delta_to_string,
    },
    forms::{
        Dialog, EditError, FieldValue, FormCancelButton, FormCloseButton, FormDeleteButton,
        FormEditButton, FormSubmitButton, InputConsumable, InputDateTime, InputDuration,
        InputNumber, InputSelect, InputTextArea, Saving, ValidationError, validate_comments,
        validate_consumable_millilitres, validate_consumable_quantity, validate_consumption_type,
        validate_duration, validate_fixed_offset_date_time,
    },
    functions::consumptions::{
        create_consumption, create_consumption_consumable, delete_consumption,
        delete_consumption_consumable, get_child_consumables, update_consumption,
        update_consumption_consumable,
    },
    models::{
        ChangeConsumption, ChangeConsumptionConsumable, Consumable, Consumption,
        ConsumptionConsumable, ConsumptionConsumableId, ConsumptionItem, ConsumptionType, Maybe,
        MaybeF64, MaybeString, NewConsumption, NewConsumptionConsumable, UserId,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Create { user_id: UserId },
    Update { consumption: Consumption },
}

#[derive(Debug, Clone)]
struct Validate {
    time: Memo<Result<DateTime<FixedOffset>, ValidationError>>,
    duration: Memo<Result<TimeDelta, ValidationError>>,
    consumption_type: Memo<Result<ConsumptionType, ValidationError>>,
    liquid_mls: Memo<Result<MaybeF64, ValidationError>>,
    comments: Memo<Result<MaybeString, ValidationError>>,
}

async fn do_save(op: &Operation, validate: &Validate) -> Result<Consumption, EditError> {
    let time = validate.time.read().clone()?;
    let duration = validate.duration.read().clone()?;
    let consumption_type = validate.consumption_type.read().clone()?;
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
                consumption_type,
            };
            create_consumption(updates).await.map_err(EditError::Server)
        }
        Operation::Update { consumption } => {
            let changes = ChangeConsumption {
                user_id: None,
                time: Some(time),
                duration: Some(duration),
                consumption_type: Some(consumption_type),
                liquid_mls: Some(liquid_mls),
                comments: Some(comments),
            };
            update_consumption(consumption.id, changes)
                .await
                .map_err(EditError::Server)
        }
    }
}

#[component]
pub fn ConsumptionUpdate(
    op: Operation,
    on_cancel: Callback,
    on_save: Callback<Consumption>,
) -> Element {
    let time = use_signal(|| match &op {
        Operation::Create { .. } => Utc::now().with_timezone(&Local).fixed_offset().as_string(),
        Operation::Update { consumption } => consumption.time.as_string(),
    });

    let duration = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { consumption } => consumption.duration.as_string(),
    });

    let consumption_type = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { consumption } => consumption.consumption_type.as_string(),
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
        time: use_memo(move || validate_fixed_offset_date_time(&time())),
        duration: use_memo(move || validate_duration(&duration())),
        consumption_type: use_memo(move || validate_consumption_type(&consumption_type())),
        liquid_mls: use_memo(move || validate_consumable_millilitres(&liquid_mls())),
        comments: use_memo(move || validate_comments(&comments())),
    };

    let mut saving = use_signal(|| Saving::No);

    // disable form while waiting for response
    let disabled = use_memo(move || saving.read().is_saving());
    let disabled_save = use_memo(move || {
        validate.time.read().is_err()
            || validate.duration.read().is_err()
            || validate.consumption_type.read().is_err()
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
        h3 { class: "text-lg font-bold",
            match &op {
                Operation::Create { .. } => "Create Consumption".to_string(),
                Operation::Update { consumption } => {
                    format!("Edit Consumption {}", consumption.name())
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
                start_time: validate.time,
                validate: validate.duration,
                disabled,
            }
            InputSelect {
                id: "consumption_type",
                label: "Type",
                value: consumption_type,
                validate: validate.consumption_type,
                options: ConsumptionType::options(),
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
pub fn ConsumptionDelete(
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
            FormCancelButton { on_cancel: move |_| on_cancel(()), title: "Close" }
            FormSubmitButton {
                disabled,
                on_save: move |_| on_save(()),
                title: "Delete",
            }
        }
    }
}

const DIGEST_SVG: Asset = asset!("/assets/consumption/digest.svg");
const NOSE_SVG: Asset = asset!("/assets/consumption/nose.svg");
const MOUTH_SVG: Asset = asset!("/assets/consumption/mouth.svg");
const SPIT_SVG: Asset = asset!("/assets/consumption/spit.svg");
const INJECT_SVG: Asset = asset!("/assets/consumption/inject.svg");
const SKIN_SVG: Asset = asset!("/assets/consumption/skin.svg");

#[component]
pub fn consumption_icon(consumption_type: ConsumptionType) -> Element {
    let icon = match consumption_type {
        ConsumptionType::Digest => DIGEST_SVG,
        ConsumptionType::InhaleNose => NOSE_SVG,
        ConsumptionType::InhaleMouth => MOUTH_SVG,
        ConsumptionType::SpitOut => SPIT_SVG,
        ConsumptionType::Inject => INJECT_SVG,
        ConsumptionType::ApplySkin => SKIN_SVG,
    };
    rsx! {
        img {
            class: "w-10 invert inline-block",
            alt: format!("{}", consumption_type),
            src: icon,
        }
    }
}

#[component]
pub fn consumption_duration(duration: chrono::TimeDelta) -> Element {
    let text = time_delta_to_string(duration);

    rsx! {
        if duration.num_seconds() < 2 {
            span { class: "text-error", {text} }
        } else if duration.num_minutes() < 60 {
            span { class: "text-success", {text} }
        } else {
            span { class: "text-error", {text} }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveDialog {
    UpdateBasic(Operation),
    Delete(Consumption),
    UpdateIngredients(Consumption),
    NestedIngredient(Consumption, Consumable),
    NestedIngredients(Consumption, Consumable),
    #[allow(dead_code)]
    Idle,
}

#[component]
pub fn ConsumptionDialog(
    dialog: ActiveDialog,
    on_change: Callback<Consumption>,
    on_change_ingredients: Callback<Consumption>,
    on_delete: Callback<Consumption>,
    show_update_basic: Callback<Consumption>,
    show_update_ingredients: Callback<Consumption>,
    show_ingredient_update_basic: Callback<(Consumption, Consumable)>,
    show_ingredient_update_ingredients: Callback<(Consumption, Consumable)>,
    on_close: Callback<()>,
) -> Element {
    match dialog {
        ActiveDialog::UpdateBasic(op) => {
            rsx! {
                Dialog {
                    ConsumptionUpdate {
                        op,
                        on_cancel: on_close,
                        on_save: move |consumption: Consumption| {
                            on_change(consumption.clone());
                            show_update_ingredients(consumption);
                        },
                    }
                }
            }
        }
        ActiveDialog::Delete(consumption) => {
            rsx! {
                Dialog {
                    ConsumptionDelete {
                        consumption,
                        on_cancel: on_close,
                        on_delete: move |consumption| {
                            on_delete(consumption);
                            on_close(())
                        },
                    }
                }
            }
        }
        ActiveDialog::UpdateIngredients(consumption) => {
            rsx! {
                Dialog {
                    ConsumptionUpdateIngredients {
                        consumption,
                        on_close,
                        on_change: move |consumption: Consumption| {
                            on_change_ingredients(consumption);
                        },
                        show_update_basic,
                        show_ingredient_update_basic,
                        show_ingredient_update_ingredients,
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
                        op: consumables::Operation::Update {
                            consumable: consumable.clone()
                        },

                        on_cancel: move |()| {
                            show_update_ingredients(parent_clone_1.clone())
                        },
                        on_save: move |_consumable: Consumable| {
                            on_change_ingredients(parent.clone());
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
                    ConsumableUpgradeIngredients {
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
        ActiveDialog::Idle => {
            rsx! {}
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
pub fn ConsumptionUpdateIngredients(
    consumption: ReadOnlySignal<Consumption>,
    on_close: Callback<()>,
    on_change: Callback<Consumption>,
    show_update_basic: Callback<Consumption>,
    show_ingredient_update_basic: Callback<(Consumption, Consumable)>,
    show_ingredient_update_ingredients: Callback<(Consumption, Consumable)>,
) -> Element {
    let mut selected_consumable = use_signal(|| None);
    let mut consumption_consumables =
        use_resource(move || async move { get_child_consumables(consumption().id).await });

    use_effect(move || {
        let _trigger = consumption();
        selected_consumable.set(None);
    });

    let consumption = consumption();

    let consumption_clone = consumption.clone();
    let consumption_clone_3 = consumption.clone();
    let consumption_clone_4 = consumption.clone();
    let consumption_clone_5 = consumption.clone();
    let consumption_clone_6 = consumption.clone();

    let mut state = use_signal(|| State::Idle);

    let mut add_value = use_signal(|| None);
    let add_consumable = use_callback(move |(child, new): (Consumable, bool)| {
        let consumption = consumption_clone.clone();
        if let Some(Ok(list)) = consumption_consumables.read().as_ref() {
            if let Some(existing) = list.iter().find(|cc| cc.consumable.id == child.id) {
                selected_consumable.set(Some(existing.clone()));
                return;
            }
        }

        let consumption_clone = consumption_clone_3.clone();
        spawn(async move {
            state.set(State::Saving);
            let updates = NewConsumptionConsumable {
                id: ConsumptionConsumableId::new(consumption.id, child.id),
                quantity: Maybe::None,
                liquid_mls: Maybe::None,
                comments: Maybe::None,
            };
            let result = create_consumption_consumable(updates).await;
            if let Ok(nested) = result.clone() {
                selected_consumable.set(Some(ConsumptionItem::new(nested, child.clone())));
                consumption_consumables.restart();
            }
            let result = result.map(|_nested| ());
            state.set(State::Finished(result));
            on_change(consumption_clone.clone());
            if new {
                show_ingredient_update_basic((consumption, child))
            }
        });
    });

    let remove_consumable = use_callback(move |child: ConsumptionConsumable| {
        let consumption_clone = consumption_clone_4.clone();
        spawn(async move {
            state.set(State::Saving);
            let result = delete_consumption_consumable(child.id).await;
            state.set(State::Finished(result));
            consumption_consumables.restart();
            on_change(consumption_clone.clone());
        });
    });

    let disabled = use_memo(move || State::Saving == *state.read());

    let is_selected = |item: &ConsumptionItem| {
        if let Some(selected) = selected_consumable() {
            selected.consumable.id == item.consumable.id
        } else {
            false
        }
    };

    rsx! {
        h3 { class: "text-lg font-bold",
            "Consumption Ingredients "
            {consumption.name()}
        }

        match consumption_consumables() {
            Some(Ok(consumption_consumables)) => {
                rsx! {
                    div { class: "p-4",
                        ul {
                            for item in consumption_consumables {

                                li {
                                    class: "p-4 mb-1 bg-gray-700 border-2 rounded-lg",
                                    class: if is_selected(&item) { "border-gray-50 text-gray-50" } else { "border-gray-500" },
                                    onclick: move |_| {
                                        selected_consumable.set(Some(item.clone()));
                                    },
                                    ConsumptionItemSummary { key: item.id, item: item.clone() }
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
                        h2 { class: "card-title",
                            "Selected: "
                            {sel.consumable.name.clone()}
                        }
                        ConsumableConsumptionForm {
                            consumption: sel.nested.clone(),
                            consumable: sel.consumable.clone(),
                            on_cancel: move |_| {
                                selected_consumable.set(None);
                            },
                            on_save: move |_consumption| {
                                selected_consumable.set(None);
                                consumption_consumables.restart();
                                on_change(consumption.clone());
                            },
                        }
                        FormEditButton {
                            title: "Edit",
                            on_edit: move || {
                                show_ingredient_update_basic((consumption_clone_5.clone(), consumable_clone_1.clone()));
                            },
                        }
                        FormEditButton {
                            title: "Ingredients",
                            on_edit: move || {
                                show_ingredient_update_ingredients((consumption_clone_6.clone(), consumable_clone_2.clone()));
                            },
                        }
                        FormDeleteButton {
                            title: "Delete",
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
                    label: "Add Consumable",
                    value: add_value,
                    on_create: move |value| {
                        add_consumable((value, true));
                        add_value.set(None);
                    },
                    on_change: move |value| {
                        if let Some(value) = value {
                            add_consumable((value, false));
                            add_value.set(None);
                        }
                    },
                    disabled,
                }
                FormEditButton {
                    title: "Edit",
                    on_edit: move || {
                        show_update_basic(consumption.clone());
                    },
                }
                FormCloseButton {
                    on_close: move || {
                        on_close(());
                    },
                    title: "Close",
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

    let updates = ChangeConsumptionConsumable {
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
    consumption: ReadOnlySignal<ConsumptionConsumable>,
    consumable: ReadOnlySignal<Consumable>,
    on_cancel: Callback<()>,
    on_save: Callback<ConsumptionConsumable>,
) -> Element {
    let mut quantity = use_signal(|| consumption.read().quantity.as_string());
    let mut liquid_mls = use_signal(|| consumption.read().liquid_mls.as_string());
    let mut comments = use_signal(|| consumption.read().comments.as_string());

    use_effect(move || {
        let nested = consumption.read();
        quantity.set(nested.quantity.as_string());
        liquid_mls.set(nested.liquid_mls.as_string());
        comments.set(nested.comments.as_string());
    });

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

    let validate_clone = validate.clone();
    let on_save = use_callback(move |()| {
        let validate = validate_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            let result = do_save_consumption(consumption(), &validate).await;
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

            FormSubmitButton {
                disabled: disabled_save,
                on_save: move |_| on_save(()),
                title: "Save",
            }
            FormCancelButton { on_cancel: move |_| on_cancel(()), title: "Cancel" }
        }
    }
}

#[component]
pub fn ConsumptionItemSummary(item: ConsumptionItem) -> Element {
    rsx! {
        span {
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
pub fn ConsumptionItemList(list: Vec<ConsumptionItem>) -> Element {
    rsx! {
        if !list.is_empty() {
            ul { class: "list-disc ml-4",
                for item in &list {
                    li {
                        ConsumptionItemSummary {
                            key: item.id,
                            item: item.clone(),
                        }
                    }
                }
            }
        }
    }
}
