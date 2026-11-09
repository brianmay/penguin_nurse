use std::{collections::HashSet, num::ParseIntError, str::FromStr};

use chrono::{Local, NaiveTime, Utc};
use dioxus::prelude::*;
use dioxus_fullstack::ServerFnError;
use dioxus_router::ToQueryArgument;
use itertools::intersperse;
use penguin_nurse::validation::schedule_errors;
use tap::Pipe;
use thiserror::Error;

use crate::{
    components::{
        StrIcon,
        consumables::ConsumableLabel,
        consumptions::{ConsumptionItemList, ConsumptionTypeIcon},
        events::{EventDateTimeShort, Markdown},
    },
    dt::{get_date_for_dt, get_utc_times_for_date},
    forms::{
        Dialog, EditError, FieldValue, FormCloseButton, FormDeleteButton, FormEditButton,
        FormSaveCancelButton, InputBoolean, InputConsumable, InputConsumptionType, InputNumber,
        InputTextArea, InputTime, Saving, ValidationError, validate_comments,
        validate_consumable_millilitres, validate_consumable_quantity, validate_consumption_type,
        validate_time_of_day,
    },
    functions::consumptions::{
        create_consumption, create_consumption_consumable, get_consumptions_for_time_range,
        get_consumptions_incomplete,
    },
    functions::schedules::{
        create_schedule, create_schedule_consumable, delete_schedule, delete_schedule_consumable,
        get_child_schedule_consumables, update_schedule, update_schedule_consumable,
    },
    models::{
        ChangeSchedule, ChangeScheduleConsumable, Consumable, ConsumableId,
        ConsumptionConsumableId, ConsumptionId, ConsumptionType, ConsumptionWithItems, MaybeSet,
        NewConsumption, NewConsumptionConsumable, NewSchedule, NewScheduleConsumable, Schedule,
        ScheduleConsumable, ScheduleConsumableId, ScheduleId, ScheduleItem, ScheduleStatus, UserId,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Create { user_id: UserId },
    Update { schedule: Schedule },
}

#[derive(Debug, Clone)]
struct Validate {
    time_of_day: Memo<Result<NaiveTime, ValidationError>>,
    consumption_type: Memo<Result<ConsumptionType, ValidationError>>,
    comments: Memo<Result<Option<String>, ValidationError>>,
}

async fn do_save(op: &Operation, validate: &Validate, active: bool) -> Result<Schedule, EditError> {
    let time_of_day = validate.time_of_day.read().clone()?;
    let consumption_type = validate.consumption_type.read().clone()?;
    let comments = validate.comments.read().clone()?;

    match op {
        Operation::Create { user_id } => {
            let updates = NewSchedule {
                user_id: *user_id,
                time_of_day,
                consumption_type,
                comments,
                active: true,
            };
            create_schedule(updates).await.map_err(EditError::Server)
        }
        Operation::Update { schedule } => {
            let changes = ChangeSchedule {
                user_id: MaybeSet::NoChange,
                time_of_day: MaybeSet::Set(time_of_day),
                consumption_type: MaybeSet::Set(consumption_type),
                comments: MaybeSet::Set(comments),
                active: MaybeSet::Set(active),
            };
            update_schedule(schedule.id, changes)
                .await
                .map_err(EditError::Server)
        }
    }
}

#[component]
pub fn ScheduleUpdate(op: Operation, on_cancel: Callback, on_save: Callback<Schedule>) -> Element {
    let time_of_day = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { schedule } => schedule.time_of_day.as_raw(),
    });

    let consumption_type = use_signal(|| match &op {
        Operation::Create { .. } => None,
        Operation::Update { schedule } => Some(schedule.consumption_type),
    });

    let comments = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { schedule } => schedule.comments.as_raw(),
    });

    let active = use_signal(|| match &op {
        Operation::Create { .. } => true,
        Operation::Update { schedule } => schedule.active,
    });

    let validate = Validate {
        time_of_day: use_memo(move || validate_time_of_day(&time_of_day())),
        consumption_type: use_memo(move || validate_consumption_type(consumption_type())),
        comments: use_memo(move || validate_comments(&comments())),
    };

    let mut saving = use_signal(|| Saving::No);

    // disable form while waiting for response
    let disabled = use_memo(move || saving.read().is_saving());
    let disabled_save = use_memo(move || {
        validate.time_of_day.read().is_err()
            || validate.consumption_type.read().is_err()
            || validate.comments.read().is_err()
            || disabled()
    });

    let op_clone = op.clone();
    let validate_clone = validate.clone();
    let active_clone = *active.read();
    let on_save = use_callback(move |()| {
        let op = op_clone.clone();
        let validate = validate_clone.clone();
        let active = active_clone;
        spawn(async move {
            saving.set(Saving::Yes);

            let result = do_save(&op, &validate, active).await;

            match result {
                Ok(schedule) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_save(schedule);
                }
                Err(err) => saving.set(Saving::Finished(Err(err))),
            }
        });
    });

    rsx! {
        h3 { class: "text-lg font-bold",
            match &op {
                Operation::Create { .. } => "Create Schedule".to_string(),
                Operation::Update { schedule } => {
                    format!("Edit Schedule {}", schedule.name())
                }
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
            InputTime {
                id: "time_of_day",
                label: "Time of Day",
                value: time_of_day,
                validate: validate.time_of_day,
                disabled,
            }
            InputConsumptionType {
                id: "consumption_type",
                label: "Type",
                value: consumption_type,
                validate: validate.consumption_type,
                disabled,
            }
            InputTextArea {
                id: "comments",
                label: "Comments",
                value: comments,
                validate: validate.comments,
                disabled,
            }
            match &op {
                Operation::Update { .. } => {
                    rsx! {
                        InputBoolean {
                            id: "active",
                            label: "Active",
                            value: active,
                            disabled,
                        }
                    }
                }
                Operation::Create { .. } => rsx! {},
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
pub fn ScheduleDelete(
    schedule: Schedule,
    on_cancel: Callback,
    on_delete: Callback<Schedule>,
) -> Element {
    let mut saving = use_signal(|| Saving::No);

    let disabled = use_memo(move || saving.read().is_saving());

    let schedule_clone = schedule.clone();
    let on_save = use_callback(move |()| {
        let schedule = schedule_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            match delete_schedule(schedule.id).await {
                Ok(_) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_delete(schedule.clone());
                }
                Err(err) => saving.set(Saving::Finished(Err(EditError::Server(err)))),
            }
        });
    });

    rsx! {
        h3 { class: "text-lg font-bold",
            "Delete schedule "
            {schedule.name()}
        }
        p { class: "py-4", "Press ESC key or click the button below to close" }
        ScheduleDetails { schedule: schedule.clone() }
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

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveDialog {
    Change(Operation),
    Delete(Schedule),
    Ingredients(Schedule),
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
        schedule_id: ScheduleId,
    },
    UpdateIngredients {
        schedule_id: ScheduleId,
    },
    Delete {
        schedule_id: ScheduleId,
    },
    #[default]
    Idle,
}

impl ToQueryArgument for ListDialogReference {
    fn display_query_argument(
        &self,
        query_name: &str,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}={}", query_name, self.to_string())
    }
}

impl FromStr for ListDialogReference {
    type Err = ListDialogReferenceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split("-").collect::<Vec<_>>();
        match split[..] {
            ["create"] => Self::Create,
            ["update", id] => {
                let schedule_id = ScheduleId::new(id.parse()?);
                Self::UpdateBasic { schedule_id }
            }
            ["ingredients", id] => {
                let schedule_id = ScheduleId::new(id.parse()?);
                Self::UpdateIngredients { schedule_id }
            }
            ["delete", id] => {
                let schedule_id = ScheduleId::new(id.parse()?);
                Self::Delete { schedule_id }
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
            ListDialogReference::UpdateBasic { schedule_id } => format!("update-{schedule_id}"),
            ListDialogReference::UpdateIngredients { schedule_id } => {
                format!("ingredients-{schedule_id}")
            }
            ListDialogReference::Delete { schedule_id } => format!("delete-{schedule_id}"),
            ListDialogReference::Idle => String::new(),
        }
    }
}

#[component]
pub fn ScheduleDialog(
    dialog: ReadSignal<ActiveDialog>,
    on_change: Callback<Schedule>,
    on_change_ingredients: Callback<Schedule>,
    on_delete: Callback<Schedule>,
    show_update_basic: Callback<Schedule>,
    show_update_ingredients: Callback<Schedule>,
    on_close: Callback<()>,
) -> Element {
    match dialog() {
        ActiveDialog::Idle => rsx! {},
        ActiveDialog::Change(op) => {
            rsx! {
                Dialog {
                    ScheduleUpdate {
                        op,
                        on_cancel: on_close,
                        on_save: move |schedule: Schedule| {
                            on_change(schedule.clone());
                            show_update_ingredients(schedule);
                        },
                    }
                }
            }
        }
        ActiveDialog::Delete(schedule) => {
            rsx! {
                Dialog {
                    ScheduleDelete {
                        schedule,
                        on_cancel: on_close,
                        on_delete: move |schedule| {
                            on_delete(schedule);
                            on_close(());
                        },
                    }
                }
            }
        }
        ActiveDialog::Ingredients(schedule) => {
            rsx! {
                Dialog {
                    // Closures must be used here or it will sometimes panic.
                    // See https://github.com/DioxusLabs/dioxus/discussions/4534
                    ScheduleUpdateIngredients {
                        schedule,
                        on_close,
                        on_change: move |param| {
                            on_change_ingredients(param);
                        },
                        show_update_basic: move |param| {
                            show_update_basic(param);
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
pub fn ScheduleUpdateIngredients(
    schedule: ReadSignal<Schedule>,
    on_close: Callback<()>,
    on_change: Callback<Schedule>,
    show_update_basic: Callback<Schedule>,
) -> Element {
    let mut selected_consumable = use_signal(|| None);
    let create_form = use_signal(|| false);

    let mut schedule_consumables =
        use_resource(move || async move { get_child_schedule_consumables(schedule().id).await });

    use_effect(move || {
        let _trigger = schedule();
        selected_consumable.set(None);
    });

    let schedule = schedule();

    let schedule_clone = schedule.clone();
    let schedule_clone_2 = schedule.clone();
    let schedule_clone_3 = schedule.clone();
    let schedule_clone_4 = schedule.clone();

    let mut state = use_signal(|| State::Idle);

    let mut add_value = use_signal(|| None);
    let add_consumable = use_callback(move |child: Consumable| {
        let schedule = schedule_clone.clone();
        if let Some(Ok(list)) = schedule_consumables.read().as_ref()
            && let Some(existing) = list.iter().find(|item| item.consumable.id == child.id)
        {
            selected_consumable.set(Some(existing.clone()));
            return;
        }

        let schedule_clone = schedule_clone_3.clone();
        spawn(async move {
            state.set(State::Saving);
            let updates = NewScheduleConsumable {
                id: ScheduleConsumableId::new(schedule.id, child.id),
                quantity: None,
                liquid_mls: None,
                comments: None,
            };
            let result = create_schedule_consumable(updates).await;
            if let Ok(nested) = result.clone() {
                selected_consumable.set(Some(ScheduleItem::new(nested, child.clone())));
                schedule_consumables.restart();
            }
            let result = result.map(|_nested| ());
            state.set(State::Finished(result));
            on_change(schedule_clone.clone());
        });
    });

    let remove_consumable = use_callback(move |child: ScheduleConsumable| {
        let schedule = schedule_clone_4.clone();
        spawn(async move {
            state.set(State::Saving);
            let result = delete_schedule_consumable(child.id).await;
            state.set(State::Finished(result));
            schedule_consumables.restart();
            on_change(schedule);
        });
    });

    let disabled = use_memo(move || State::Saving == *state.read());

    let is_selected = |item: &ScheduleItem| {
        if let Some(selected) = selected_consumable() {
            selected.consumable.id == item.consumable.id
        } else {
            false
        }
    };

    rsx! {
        h3 { class: "text-lg font-bold",
            "Schedule Ingredients "
            {schedule.name()}
        }

        if !create_form() {
            match schedule_consumables() {
                Some(Ok(schedule_consumables)) => {
                    rsx! {
                        ScheduleSummary {
                            schedule: schedule.clone(),
                            schedule_consumables: Some(schedule_consumables.clone()),
                        }
                        div { class: "p-4",
                            ul {
                                for item in schedule_consumables {
                                    li {
                                        class: "p-4 mb-1 bg-gray-700 border-2 rounded-lg",
                                        class: if is_selected(&item) { "border-gray-50 text-gray-50" } else { "border-gray-500" },
                                        onclick: move |_| {
                                            selected_consumable.set(Some(item.clone()));
                                        },
                                        ScheduleItemSummary { item: item.clone() }
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
        if let Some(sel) = selected_consumable() {
            {
                rsx! {
                    div { class: "card bg-gray-800 shadow-xl",
                        div { class: "card-body",
                            h2 { class: "card-title",
                                "Selected: "
                                {sel.consumable.name.clone()}
                            }
                            ConsumableScheduleForm {
                                nested: sel.nested.clone(),
                                consumable: sel.consumable.clone(),
                                on_cancel: move |_| {
                                    selected_consumable.set(None);
                                },
                                on_save: move |_schedule| {
                                    selected_consumable.set(None);
                                    schedule_consumables.restart();
                                    on_change(schedule_clone_2.clone());
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
            }
        } else {
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
                    create_form,
                    disabled,
                }
                if !create_form() {
                    FormEditButton {
                        title: "Edit",
                        on_edit: move || {
                            show_update_basic(schedule.clone());
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
}

#[derive(Debug, Clone)]
struct ValidateScheduleItem {
    quantity: Memo<Result<Option<bigdecimal::BigDecimal>, ValidationError>>,
    liquid_mls: Memo<Result<Option<bigdecimal::BigDecimal>, ValidationError>>,
    comments: Memo<Result<Option<String>, ValidationError>>,
}

async fn do_save_schedule_item(
    schedule_consumable: ScheduleConsumable,
    validate: &ValidateScheduleItem,
) -> Result<ScheduleConsumable, EditError> {
    let quantity = validate.quantity.read().clone()?;
    let liquid_mls = validate.liquid_mls.read().clone()?;
    let comments = validate.comments.read().clone()?;

    let updates = ChangeScheduleConsumable {
        quantity: MaybeSet::Set(quantity),
        liquid_mls: MaybeSet::Set(liquid_mls),
        comments: MaybeSet::Set(comments),
    };
    update_schedule_consumable(schedule_consumable.id, updates)
        .await
        .map_err(EditError::Server)
}

#[component]
fn ConsumableScheduleForm(
    nested: ReadSignal<ScheduleConsumable>,
    consumable: ReadSignal<Consumable>,
    on_cancel: Callback<()>,
    on_save: Callback<ScheduleConsumable>,
) -> Element {
    let mut quantity = use_signal(|| nested.read().quantity.as_raw());
    let mut liquid_mls = use_signal(|| nested.read().liquid_mls.as_raw());
    let mut comments = use_signal(|| nested.read().comments.as_raw());

    use_effect(move || {
        let nested = nested();
        quantity.set(nested.quantity.as_raw());
        liquid_mls.set(nested.liquid_mls.as_raw());
        comments.set(nested.comments.as_raw());
    });

    let validate = ValidateScheduleItem {
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

            let result = do_save_schedule_item(nested(), &validate).await;
            match result {
                Ok(schedule_consumable) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_save(schedule_consumable);
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
                saving,
            }
        }
    }
}

/// Records the given scheduled ingredients as a new, incomplete consumption
/// with the current time and the schedule's consumption type.
async fn record_items_as_consumption(
    schedule: &Schedule,
    items: Vec<ScheduleItem>,
) -> Result<(), ServerFnError> {
    let new_consumption = NewConsumption {
        user_id: schedule.user_id,
        time: Local::now().fixed_offset(),
        duration: None,
        liquid_mls: None,
        comments: None,
        consumption_type: schedule.consumption_type,
        complete: false,
    };
    let consumption = create_consumption(new_consumption).await?;

    add_items_to_consumption(consumption.id, items).await
}

/// Adds the given scheduled ingredients to an existing consumption.
async fn add_items_to_consumption(
    consumption_id: ConsumptionId,
    items: Vec<ScheduleItem>,
) -> Result<(), ServerFnError> {
    for item in items {
        let updates = NewConsumptionConsumable {
            id: ConsumptionConsumableId::new(consumption_id, item.consumable.id),
            quantity: item.nested.quantity,
            liquid_mls: item.nested.liquid_mls,
            comments: item.nested.comments,
        };
        create_consumption_consumable(updates).await?;
    }
    Ok(())
}

fn selected_items(status: &ScheduleStatus, selected: &HashSet<ConsumableId>) -> Vec<ScheduleItem> {
    status
        .items
        .iter()
        .filter(|item| !item.recorded)
        .filter(|item| selected.contains(&item.item.consumable.id))
        .map(|item| item.item.clone())
        .collect()
}

/// A schedule that is due today, with checkboxes to select which of its
/// unrecorded ingredients to record.
#[component]
pub fn ScheduleDue(
    status: ReadSignal<ScheduleStatus>,
    on_recorded: Callback<()>,
    on_add_existing: Callback<(Schedule, Vec<ScheduleItem>)>,
) -> Element {
    let unrecorded: Vec<ScheduleItem> = status()
        .items
        .iter()
        .filter(|item| !item.recorded)
        .map(|item| item.item.clone())
        .collect();

    let all_items: Vec<ScheduleItem> = status()
        .items
        .iter()
        .map(|item| item.item.clone())
        .collect();

    let mut selected: Signal<HashSet<ConsumableId>> =
        use_signal(|| unrecorded.iter().map(|item| item.consumable.id).collect());

    // Reset the selection whenever the status changes, e.g. after recording.
    use_effect(move || {
        let status = status();
        selected.set(
            status
                .items
                .iter()
                .filter(|item| !item.recorded)
                .map(|item| item.item.consumable.id)
                .collect(),
        );
    });

    let mut state = use_signal(|| State::Idle);

    let record = use_callback(move |()| {
        let items = selected_items(&status(), &selected());
        if items.is_empty() {
            return;
        }
        let schedule = status().schedule;
        spawn(async move {
            state.set(State::Saving);
            let result = record_items_as_consumption(&schedule, items).await;
            state.set(State::Finished(result));
            on_recorded(());
        });
    });

    let add_existing = use_callback(move |()| {
        let items = selected_items(&status(), &selected());
        if items.is_empty() {
            return;
        }
        let schedule = status().schedule;
        on_add_existing((schedule, items));
    });

    let disabled = use_memo(move || State::Saving == *state.read());

    rsx! {
        div { class: "p-4 mb-2 border-2 rounded-lg border-error",
            div { class: "flex items-center gap-2",
                span { class: "font-bold text-lg", {status().schedule.name()} }
                StrIcon {
                    title: status().schedule.consumption_type.as_title(),
                    icon: rsx! {
                        ConsumptionTypeIcon { consumption_type: status().schedule.consumption_type }
                    },
                }
            }
            if let Some(comments) = &status().schedule.comments {
                Markdown { content: comments.to_string() }
            }

            ScheduleErrors {
                schedule: status().schedule.clone(),
                items: Some(all_items.clone()),
            }

            for item in unrecorded {
                label { class: "flex items-center gap-2 p-2",
                    input {
                        r#type: "checkbox",
                        class: "checkbox",
                        checked: selected().contains(&item.consumable.id),
                        oninput: {
                            let id = item.consumable.id;
                            move |e| {
                                let mut new_selected = selected();
                                if e.checked() {
                                    new_selected.insert(id);
                                } else {
                                    new_selected.remove(&id);
                                }
                                selected.set(new_selected);
                            }
                        },
                    }
                    ScheduleItemSummary { item: item.clone() }
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
                        div { class: "alert alert-success", "Recorded! Finish the details on the Incomplete page." }
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

            div { class: "flex gap-2",
                button {
                    r#type: "button",
                    class: "btn btn-outline btn-secondary align-top",
                    disabled,
                    onclick: move |_e| record(()),
                    "Record as new Consumption"
                }
                button {
                    r#type: "button",
                    class: "btn btn-outline btn-secondary align-top",
                    disabled,
                    onclick: move |_e| add_existing(()),
                    "Add to existing Consumption"
                }
            }
        }
    }
}

/// Dialog to pick an existing consumption to add scheduled ingredients to.
/// Offers today's consumptions plus incomplete ones.
#[component]
pub fn ScheduleAddToConsumption(
    schedule: Schedule,
    items: Vec<ScheduleItem>,
    on_close: Callback<()>,
    on_added: Callback<()>,
) -> Element {
    let user_id = schedule.user_id;

    let mut consumptions: Resource<Result<Vec<ConsumptionWithItems>, ServerFnError>> =
        use_resource(move || async move {
            let date = get_date_for_dt(Utc::now());
            let (start, end) = get_utc_times_for_date(date)?;

            let mut list = get_consumptions_for_time_range(user_id, start, end).await?;

            let incomplete = get_consumptions_incomplete(user_id).await?;
            for consumption in incomplete {
                if !list
                    .iter()
                    .any(|existing| existing.consumption.id == consumption.consumption.id)
                {
                    list.push(consumption);
                }
            }

            Ok(list)
        });

    let mut state = use_signal(|| State::Idle);

    rsx! {
        h3 { class: "text-lg font-bold",
            "Add to existing Consumption"
        }
        p { class: "py-4",
            "Add "
            {items.len().to_string()}
            " scheduled ingredients from "
            {schedule.name()}
            " to:"
        }

        match state() {
            State::Saving => {
                rsx! {
                    div { class: "alert alert-info", "Saving..." }
                }
            }
            State::Finished(Ok(())) => {
                rsx! {
                    div { class: "alert alert-success", "Added!" }
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

        match consumptions() {
            Some(Err(err)) => {
                rsx! {
                    div { class: "alert alert-error",
                        "Error loading consumptions: "
                        {err.to_string()}
                    }
                }
            }
            Some(Ok(consumptions)) if consumptions.is_empty() => {
                rsx! {
                    p { class: "alert alert-info", "No consumptions found for today." }
                }
            }
            Some(Ok(list)) => {
                rsx! {
                    div { class: "p-4",
                        ul {
                            for consumption in list {
                                li {
                                    class: "p-4 mb-1 bg-gray-700 border-2 rounded-lg border-gray-500 hover:border-gray-50",
                                    onclick: {
                                        let consumption_id = consumption.consumption.id;
                                        let items_clone = items.clone();
                                        move |_| {
                                            if *state.read() == State::Saving {
                                                return;
                                            }
                                            let items_clone = items_clone.clone();
                                            spawn(async move {
                                                state.set(State::Saving);
                                                let result = add_items_to_consumption(
                                                    consumption_id,
                                                    items_clone,
                                                )
                                                .await;
                                                state.set(State::Finished(result));
                                                consumptions.restart();
                                                on_added(());
                                            });
                                        }
                                    },
                                    div { class: "flex items-center gap-2",
                                        span { class: "font-bold", {consumption.consumption.name()} }
                                        StrIcon {
                                            title: consumption.consumption.consumption_type.as_title(),
                                            icon: rsx! {
                                                ConsumptionTypeIcon {
                                                    consumption_type: consumption.consumption.consumption_type,
                                                }
                                            },
                                        }
                                        EventDateTimeShort { time: consumption.consumption.time }
                                    }
                                    if !consumption.items.is_empty() {
                                        ConsumptionItemList { list: consumption.items.clone() }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            None => {
                rsx! {
                    div { class: "p-4", "Loading..." }
                }
            }
        }

        FormCloseButton {
            on_close: move || {
                on_close(());
            },
            title: "Cancel",
        }
    }
}

#[component]
pub fn ScheduleDetails(schedule: Schedule) -> Element {
    rsx! {
        div {
            span { class: "font-bold", {schedule.name()} }
            " — "
            {schedule.consumption_type.as_title()}
            if !schedule.active {
                " (inactive)"
            }
        }
        if let Some(comments) = &schedule.comments {
            Markdown { content: comments.to_string() }
        }
    }
}

#[component]
pub fn ScheduleSummary(
    schedule: Schedule,
    schedule_consumables: Option<Vec<ScheduleItem>>,
) -> Element {
    rsx! {
        ScheduleDetails { schedule: schedule.clone() }
        ScheduleErrors {
            schedule,
            items: schedule_consumables.clone(),
        }
        if let Some(items) = &schedule_consumables {
            ScheduleItemList { list: items.clone() }
        }
    }
}

/// Displays any validation errors for the schedule and its items, e.g. an
/// ingredient whose consumption type does not match the schedule.
#[component]
pub fn ScheduleErrors(schedule: Schedule, items: Option<Vec<ScheduleItem>>) -> Element {
    let errors = schedule_errors(&schedule, items.as_ref());

    rsx! {
        if !errors.is_empty() {
            for error in errors {
                div { class: "text-error", {error} }
            }
        }
    }
}

#[component]
pub fn ScheduleItemSummary(item: ScheduleItem, recorded: Option<bool>) -> Element {
    let mut quantity_list = Vec::new();

    if let Some(quantity) = item.nested.quantity {
        quantity_list.push(rsx! {
            span {
                {quantity.to_string()}
                {item.consumable.unit.postfix()}
            }
        });
    }

    if let Some(liquid_mls) = item.nested.liquid_mls {
        quantity_list.push(rsx! {
            span {
                "Liquid: "
                {liquid_mls.to_string()}
                "ml"
            }
        });
    }

    rsx! {
        div {
            class: if recorded == Some(true) { "text-gray-400 line-through" } else { "" },
            if quantity_list.is_empty() {
                {}
            } else {
                div { {intersperse(quantity_list.into_iter(), rsx! { ", " })} }
            }
            if let Some(comments) = &item.nested.comments {
                Markdown { content: comments.to_string() }
            }
            ConsumableLabel { consumable: item.consumable }
            if recorded == Some(true) {
                span { class: "text-success", " ✓" }
            }
        }
    }
}

#[component]
pub fn ScheduleItemList(list: Vec<ScheduleItem>) -> Element {
    rsx! {
        if !list.is_empty() {
            ul { class: "list-disc ml-4",
                for item in &list {
                    li {
                        ScheduleItemSummary { key: item.id, item: item.clone() }
                    }
                }
            }
        }
    }
}
