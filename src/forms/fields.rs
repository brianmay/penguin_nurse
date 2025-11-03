#![allow(non_snake_case)]
use chrono::{DateTime, FixedOffset, Local, TimeDelta, Utc};
use classes::classes;
use dioxus::{prelude::*, signals::Signal};
use dioxus_fullstack::ServerFnError;
use palette::{Hsv, IntoColor, Srgb};
use std::{ops::Deref, rc::Rc};
use tap::Pipe;

use crate::{
    components::{
        ElementIcon,
        buttons::{ActionButton, CreateButton},
        consumables::{self, ConsumableLabel, ConsumableUnitIcon, ConsumableUpdate},
        consumptions::ConsumptionTypeIcon,
        events::{UrgencyIcon, UrgencyLabel},
        exercises::{ExerciseRpeIcon, ExerciseRpeLabel, ExerciseTypeIcon},
        poos::PooBristolIcon,
    },
    forms::{
        Barcode, validate_colour_hue, validate_colour_saturation, validate_colour_value,
        values::FieldLabel,
    },
    functions::consumables::search_consumables,
    models::{
        Bristol, Consumable, ConsumableUnit, ConsumptionType, ExerciseRpe, ExerciseType, Urgency,
    },
};

use super::FieldValue;
use super::errors::ValidationError;

fn get_label_classes() -> String {
    classes![
        "block",
        "mb-2",
        "text-sm",
        "font-medium",
        "text-gray-900",
        "dark:text-white"
    ]
}

fn get_checkbox_classes(is_valid: bool, is_disabled: bool) -> String {
    let classes = classes![
        "bg-gray-100",
        "checkbox",
        "dark:bg-gray-700",
        "dark:focus:ring-blue-500",
        "dark:ring-offset-gray-800",
        "dark:text-white",
        "focus:ring-2",
        "focus:ring-blue-500",
        "h-4",
        "ring-offset-2",
        "ring-offset-gray-100",
        "rounded",
        "text-gray-900",
        "w-4",
        "focus:outline-none"
    ];

    if is_disabled {
        return classes + " " + &classes!["border-gray-300", "dark:border-gray-600"];
    }

    if is_valid {
        return classes + " " + &classes!["border-green-500", "dark:border-green-500"];
    }

    classes + &classes!["border-red-500", "dark:border-red-500"]
}

fn get_input_classes(is_valid: bool, is_disabled: bool) -> String {
    let classes = classes![
        "bg-gray-50",
        "block",
        "border",
        "dark:bg-gray-700",
        "dark:focus:ring-blue-500",
        "dark:ring-offset-gray-800",
        "dark:placeholder-gray-400",
        "dark:text-white",
        "focus:ring-2",
        "focus:ring-blue-500",
        "p-2.5",
        "ring-offset-2",
        "ring-offset-gray-100",
        "rounded-lg",
        "text-gray-900",
        "w-full",
        "focus:outline-none"
    ];

    if is_disabled {
        return classes + " " + &classes!["border-gray-300", "dark:border-gray-600"];
    }

    if is_valid {
        return classes + " " + &classes!["border-green-500", "dark:border-green-500"];
    }

    classes + " " + &classes!["border-red-500", "dark:border-red-500"]
}

#[component]
pub fn FieldMessage<D: 'static + Clone + PartialEq>(
    validate: Memo<Result<D, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    rsx! {
        if disabled() {
            div { class: "text-gray-300", "Inactive" }
        } else if let Err(err) = validate() {
            div { class: "text-red-500", "{err}" }
        } else {
            div { class: "text-green-500", "Looks good!" }
        }
    }
}

#[derive(Clone, PartialEq)]
struct PullDownMenuItem<D: 'static + Clone + PartialEq + FieldLabel> {
    id: String,
    value: Option<D>,
    icon: Element,
    label: Element,
}

#[component]
fn PullDownMenu<D: 'static + Clone + PartialEq + FieldLabel>(
    id: String,
    items: ReadSignal<Vec<PullDownMenuItem<D>>>,
    on_select: Callback<Option<D>>,
    search: Signal<String>,
    on_close: Callback<()>,
) -> Element {
    let mut focus_idx = use_signal(|| 0);
    let mut elements: Signal<Vec<Option<Rc<MountedData>>>> = use_signal(Vec::new);

    use_effect(move || {
        let len = items.read().len() + 1; // +1 for the input element
        elements.write().resize(len, None);
    });

    use_effect(move || {
        let idx = focus_idx();
        if let Some(Some(el)) = elements.read().get(idx).cloned() {
            let el = el.clone();
            spawn(async move { _ = el.set_focus(true).await });
        }
    });

    // Keyboard handling
    let onkeydown_input = {
        move |evt: KeyboardEvent| match evt.key() {
            Key::ArrowDown => {
                evt.prevent_default();
                let items_len = items.read().len() + 1;
                if items_len > 0 {
                    focus_idx.set(1);
                }
            }
            Key::ArrowUp => {
                evt.prevent_default();
                let items_len = items.read().len() + 1;
                if items_len > 0 {
                    focus_idx.set(items_len - 1);
                }
            }
            Key::Enter => {
                evt.prevent_default();
                if let Some(item) = items.read().first() {
                    on_select.call(item.value.clone());
                }
            }
            Key::Tab => {
                on_close.call(());
            }
            Key::Escape => {
                evt.prevent_default();
                tracing::debug!("Escape pressed");
                on_close.call(());
            }
            _ => {}
        }
    };
    let onkeydown_list = {
        let items_len = items.read().len() + 1;
        move |evt: KeyboardEvent| match evt.key() {
            Key::ArrowDown => {
                evt.prevent_default();
                let idx = focus_idx();
                let next = (idx + 1) % items_len;
                focus_idx.set(next);
            }
            Key::ArrowUp => {
                evt.prevent_default();
                let idx = focus_idx();
                let prev = (idx + items_len - 1) % items_len;
                focus_idx.set(prev);
            }
            Key::Tab => {
                on_close.call(());
            }
            Key::Escape => {
                evt.prevent_default();
                on_close.call(());
            }
            Key::Character(c) => {
                evt.prevent_default();
                search.set(c);
                focus_idx.set(0);
            }
            _ => {}
        }
    };

    let set_element = |mut elements: Signal<Vec<Option<Rc<MountedData>>>>,
                       index: usize,
                       el: Option<Rc<MountedData>>| {
        let mut els = elements.write();
        if index >= els.len() {
            els.resize(index + 1, None);
        }
        els[index] = el;
    };

    rsx! {
        div {
            class: "absolute z-10 shadow-lg bg-gray-50 border border-gray-50 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
            onblur: move |_| {
                on_close.call(());
            },
            input {
                r#type: "text",
                class: get_input_classes(true, false),
                id,
                placeholder: "Enter input",
                value: search(),
                oninput: move |e| {
                    search.set(e.value());
                },
                onmounted: move |e| {
                    set_element(elements, 0, Some(e.data()));
                    focus_idx.set(0);
                },
                onfocus: move |_| {
                    focus_idx.set(0);
                },
                onkeydown: onkeydown_input,
                tabindex: "-1",
            }
            ul { class: "p-2 shadow rounded-box", onkeydown: onkeydown_list,
                if items.is_empty() {
                    li { "No entries found." }
                } else {

                    for (i , item) in items.read().deref().iter().enumerate() {
                        li {
                            key: "{item.id}",
                            class: "flex px-4 py-2 hover:bg-gray-800 hover:text-gray-100 cursor-pointer gap-4",
                            onclick: {
                                let value = item.value.clone();
                                move |_| {
                                    on_select.call(value.clone());
                                }
                            },
                            tabindex: "-1",
                            onkeydown: {
                                let value = item.value.clone();
                                move |e| {
                                    if e.key() == Key::Enter || e.key() == Key::Character(" ".to_string()) {
                                        e.prevent_default();
                                        on_select.call(value.clone());
                                    }
                                }
                            },
                            onmounted: {
                                move |e| {
                                    set_element(elements, i + 1, Some(e.data()));
                                }
                            },
                            ElementIcon {
                                title: item.label.clone(),
                                icon: item.icon.clone(),
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn InputSearch<D: 'static + Clone + Eq + FieldLabel, T: 'static + Clone + Eq>(
    id: &'static str,
    label: &'static str,
    validate: Memo<Result<T, ValidationError>>,
    mut value: Signal<Option<D>>,
    disabled: Memo<bool>,
    options: Memo<Vec<PullDownMenuItem<D>>>,
    search: Signal<String>,
    on_change: Option<Callback<Option<D>>>,
) -> Element {
    let mut button: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let mut open = use_signal(|| false);
    let search_id = format!("{}_search", id);

    use_effect(move || {
        if open() {
            search.set(String::new());
        } else {
            spawn(async move {
                if let Some(button) = button.read().as_ref() {
                    _ = button.set_focus(true).await;
                }
            });
        }
    });

    rsx! {
        div { class: "mb-5",
            label { class: get_label_classes(), r#for: id, "{label}" }
            div { class: "relative inline-block text-left w-full",
                button {
                    id,
                    class: "inline-flex gap-4 ".to_string() + " "
                        + &get_input_classes(validate().is_ok(), disabled()),
                    onclick: move |_| open.set(!open()),
                    onkeydown: move |evt| {
                        if evt.key() == Key::ArrowDown || evt.key() == Key::Enter {
                            evt.prevent_default();
                            open.set(true);
                        }
                    },
                    onmounted: move |e| {
                        button.set(Some(e.data()));
                    },
                    {
                        if let Some(selected_option) = value.read().deref() {
                            { selected_option.as_label() }
                        } else {
                            rsx! { "Select..." }
                        }
                    }
                }
                if open() {
                    PullDownMenu {
                        id: search_id,
                        items: options(),
                        on_select: {
                            Callback::new(move |d: Option<D>| {
                                open.set(false);
                                if let Some(on_change) = &on_change {
                                    value.set(d.clone());
                                    on_change.call(d);
                                } else {
                                    value.set(d);
                                }
                            })
                        },
                        search,
                        on_close: Callback::new(move |_| {
                            tracing::debug!("Closing pull down menu");
                            open.set(false);
                        }),
                    }
                }
                FieldMessage { validate, disabled }
            }
        }
    }
}

#[component]
pub fn InputString<D: 'static + Clone + PartialEq>(
    id: &'static str,
    label: &'static str,
    value: Signal<String>,
    validate: Memo<Result<D, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    rsx! {
        div { class: "mb-5",
            label { r#for: id, class: get_label_classes(), "{label}" }
            input {
                r#type: "text",
                class: get_input_classes(validate().is_ok(), disabled()),
                id,
                placeholder: "Enter input",
                value: "{value()}",
                disabled,
                oninput: move |e| {
                    value.set(e.value());
                },
            }
            FieldMessage { validate, disabled }
        }
    }
}

#[component]
pub fn InputNumber<D: 'static + Clone + PartialEq>(
    id: &'static str,
    label: String,
    value: Signal<String>,
    validate: Memo<Result<D, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    rsx! {
        div { class: "mb-5",
            label { r#for: id, class: get_label_classes(), "{label}" }
            input {
                r#type: "text",
                class: get_input_classes(validate().is_ok(), disabled()),
                id,
                r#type: "number",
                pattern: "[0-9]*",
                inputmode: "numeric",
                placeholder: "Enter input",
                value: "{value()}",
                disabled,
                oninput: move |e| {
                    value.set(e.value());
                },
            }
            FieldMessage { validate, disabled }
        }
    }
}

#[component]
pub fn InputSymptomIntensity(
    id: &'static str,
    label: &'static str,
    value: Signal<String>,
    validate: Memo<Result<i32, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    rsx! {
        InputNumber {
            id,
            label: label.to_string() + " (0-10)",
            value,
            validate,
            disabled,
        }
    }
}

#[component]
pub fn InputPassword<D: 'static + Clone + Eq + PartialEq>(
    id: &'static str,
    label: &'static str,
    value: Signal<String>,
    validate: Memo<Result<D, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    rsx! {
        div { class: "my-5",
            label { r#for: id, class: get_label_classes(), "{label}" }
            input {
                r#type: "password",
                class: get_input_classes(validate().is_ok(), disabled()),
                id,
                placeholder: "Enter input",
                value: value(),
                disabled,
                oninput: move |e| {
                    value.set(e.value());
                },
            }
            FieldMessage { validate, disabled }
        }
    }
}

#[component]
pub fn InputTextArea<D: 'static + Clone + Eq + PartialEq>(
    id: &'static str,
    label: &'static str,
    value: Signal<String>,
    validate: Memo<Result<D, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    rsx! {
        div { class: "mb-5",
            label { r#for: id, class: get_label_classes(), "{label}" }
            textarea {
                class: get_input_classes(validate().is_ok(), disabled()),
                id,
                placeholder: "Enter input",
                value: "{value()}",
                disabled,
                oninput: move |e| {
                    value.set(e.value());
                },
            }
            FieldMessage { validate, disabled }
        }
    }
}

#[component]
pub fn InputDateTime(
    id: &'static str,
    label: &'static str,
    value: Signal<String>,
    validate: Memo<Result<DateTime<FixedOffset>, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    rsx! {
        div { class: "mb-5",
            label { r#for: id, class: get_label_classes(), "{label}" }
            input {
                r#type: "text",
                class: get_input_classes(validate().is_ok(), disabled()),
                id,
                placeholder: "Enter input",
                value: "{value()}",
                disabled,
                oninput: move |e| {
                    value.set(e.value());
                },
            }
            ActionButton {
                on_click: move |_e| {
                    value.set(Local::now().to_rfc3339());
                },
                "Now"
            }
            FieldMessage { validate, disabled }
        }
    }
}

#[component]
pub fn InputOptionDateTimeUtc(
    id: &'static str,
    label: &'static str,
    value: Signal<String>,
    validate: Memo<Result<Option<DateTime<Utc>>, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    rsx! {
        div { class: "mb-5",
            label { r#for: id, class: get_label_classes(), "{label}" }
            input {
                r#type: "text",
                class: get_input_classes(validate().is_ok(), disabled()),
                id,
                placeholder: "Enter input",
                value: "{value()}",
                disabled,
                oninput: move |e| {
                    value.set(e.value());
                },
            }
            ActionButton {
                on_click: move |_e| {
                    value.set(Local::now().to_rfc3339());
                },
                "Now"
            }
            FieldMessage { validate, disabled }
        }
    }
}

#[component]
pub fn InputDuration(
    id: &'static str,
    label: &'static str,
    value: Signal<String>,
    start_time: Memo<Result<DateTime<FixedOffset>, ValidationError>>,
    validate: Memo<Result<TimeDelta, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    rsx! {
        div { class: "mb-5",
            label { r#for: id, class: get_label_classes(), "{label}" }
            input {
                r#type: "text",
                class: get_input_classes(validate().is_ok(), disabled()),
                id,
                r#type: "number",
                pattern: "[0-9]*",
                inputmode: "numeric",
                placeholder: "Enter input",
                value: "{value()}",
                disabled,
                oninput: move |e| {
                    value.set(e.value());
                },
            }
            if let Ok(start_time) = start_time() {
                ActionButton {
                    on_click: move |_e| {
                        let now: DateTime<FixedOffset> = Utc::now().into();
                        value.set((now - start_time).as_raw());
                    },
                    "Stop"
                }
            }
            FieldMessage { validate, disabled }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct InputOption<D: 'static + Clone + Eq + PartialEq + FieldLabel> {
    id: String,
    value: Option<D>,
    icon: Element,
    title: String,
    label: Element,
}

#[component]
pub fn InputSelect<D: 'static + Clone + Eq + FieldLabel, T: 'static + Clone + Eq>(
    id: &'static str,
    label: &'static str,
    validate: Memo<Result<T, ValidationError>>,
    value: Signal<Option<D>>,
    disabled: Memo<bool>,
    options: Vec<InputOption<D>>,
) -> Element {
    let search = use_signal(String::new);
    let filtered_options = use_memo(move || {
        let query = search.read().to_lowercase();
        options
            .iter()
            .filter(|opt| {
                query.is_empty()
                    || opt.title.to_lowercase().contains(&query)
                    || opt.id.to_lowercase() == query
            })
            .map(|opt| PullDownMenuItem {
                id: opt.id.clone(),
                value: opt.value.clone(),
                label: opt.label.clone(),
                icon: opt.icon.clone(),
            })
            .collect::<Vec<_>>()
    });

    rsx! {
        InputSearch {
            id,
            label,
            validate,
            value,
            disabled,
            options: filtered_options,
            search,
        }
    }
}

#[component]
pub fn InputConsumptionType(
    id: &'static str,
    label: &'static str,
    value: Signal<Option<ConsumptionType>>,
    validate: Memo<Result<ConsumptionType, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    let options = ConsumptionType::all_values()
        .iter()
        .map(|consumption_type| {
            let id = consumption_type.as_id();
            let icon = rsx! {
                ConsumptionTypeIcon { consumption_type: *consumption_type }
            };
            let label = consumption_type.as_title();
            InputOption {
                id: id.to_string(),
                value: Some(*consumption_type),
                icon,
                title: label.to_string(),
                label: rsx! { "{label}" },
            }
        })
        .collect::<Vec<_>>();

    rsx! {
        InputSelect {
            id,
            label,
            validate,
            value,
            disabled,
            options,
        }
    }
}

#[component]
pub fn InputConsumableUnitType(
    id: &'static str,
    label: &'static str,
    value: Signal<Option<ConsumableUnit>>,
    validate: Memo<Result<crate::models::ConsumableUnit, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    let options = ConsumableUnit::all_values()
        .iter()
        .map(|consumable_unit| {
            let id = consumable_unit.as_id();
            let icon = rsx! {
                ConsumableUnitIcon { consumable_unit: *consumable_unit }
            };
            let label = consumable_unit.as_title();
            InputOption {
                id: id.to_string(),
                value: Some(*consumable_unit),
                icon,
                title: label.to_string(),
                label: rsx! { "{label}" },
            }
        })
        .collect::<Vec<_>>();

    rsx! {
        InputSelect {
            id,
            label,
            validate,
            value,
            disabled,
            options,
        }
    }
}

#[component]
pub fn InputUrgency(
    id: &'static str,
    label: &'static str,
    value: Signal<Option<Urgency>>,
    validate: Memo<Result<Urgency, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    let options = Urgency::all_values()
        .iter()
        .map(|urgency| {
            let id = urgency.as_id();
            let icon = rsx! {
                UrgencyIcon { urgency: *urgency }
            };
            let label = urgency.as_title();
            InputOption {
                id: id.to_string(),
                value: Some(*urgency),
                icon,
                title: label.to_string(),
                label: rsx! {
                    UrgencyLabel { urgency: *urgency }
                },
            }
        })
        .collect::<Vec<_>>();

    rsx! {
        InputSelect {
            id,
            label,
            validate,
            value,
            disabled,
            options,
        }
    }
}

#[component]
pub fn InputPooBristolType(
    id: &'static str,
    label: &'static str,
    value: Signal<Option<Bristol>>,
    validate: Memo<Result<Bristol, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    let options = Bristol::all_values()
        .iter()
        .map(|bristol| {
            let id = bristol.as_id();
            let icon = rsx! {
                PooBristolIcon { bristol: *bristol }
            };
            let label = bristol.as_title();
            InputOption {
                id: id.to_string(),
                value: Some(*bristol),
                icon,
                title: label.to_string(),
                label: rsx! { "{label}" },
            }
        })
        .collect::<Vec<_>>();

    rsx! {
        InputSelect {
            id,
            label,
            validate,
            value,
            disabled,
            options,
        }
    }
}

#[component]
pub fn InputExerciseType(
    id: &'static str,
    label: &'static str,
    value: Signal<Option<ExerciseType>>,
    validate: Memo<Result<ExerciseType, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    let options = ExerciseType::all_values()
        .iter()
        .map(|exercise_type| {
            let id = exercise_type.as_id();
            let icon = rsx! {
                ExerciseTypeIcon { exercise_type: *exercise_type }
            };
            let label = exercise_type.as_title();
            InputOption {
                id: id.to_string(),
                value: Some(*exercise_type),
                icon,
                title: label.to_string(),
                label: rsx! { "{label}" },
            }
        })
        .collect::<Vec<_>>();

    rsx! {
        InputSelect {
            id,
            label,
            validate,
            value,
            disabled,
            options,
        }
    }
}

#[component]
pub fn InputExerciseRpe(
    id: &'static str,
    label: &'static str,
    value: Signal<Option<ExerciseRpe>>,
    validate: Memo<Result<Option<ExerciseRpe>, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    let mut options = ExerciseRpe::all_values()
        .iter()
        .map(|rpe| {
            let id = rpe.as_id();
            let icon = rsx! {
                ExerciseRpeIcon { rpe: *rpe }
            };
            let label = rpe.as_title();
            InputOption {
                id: id.to_string(),
                value: Some(*rpe),
                icon,
                title: label.to_string(),
                label: rsx! {
                    ExerciseRpeLabel { rpe: *rpe }
                },
            }
        })
        .collect::<Vec<_>>();

    options.insert(
        0,
        InputOption {
            id: "none".to_string(),
            value: None,
            icon: rsx! {},
            title: "None".to_string(),
            label: rsx! { "None" },
        },
    );

    rsx! {
        InputSelect {
            id,
            label,
            validate,
            value,
            disabled,
            options,
        }
    }
}

#[component]
pub fn InputExerciseCalories(
    id: &'static str,
    label: String,
    value: Signal<String>,
    validate: Memo<Result<Option<i32>, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    rsx! {
        InputNumber {
            id,
            label,
            value,
            validate,
            disabled,
        }
    }
}

#[component]
pub fn InputBoolean(
    id: &'static str,
    label: &'static str,
    mut value: Signal<bool>,
    // validate: Memo<Result<D, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    rsx! {
        div {
            label { r#for: id, class: get_label_classes(), "{label}" }
            input {
                r#type: "checkbox",
                class: get_checkbox_classes(true, disabled()),
                id,
                checked: value(),
                disabled,
                oninput: move |e| {
                    value.set(e.checked());
                },
            }
        }
    }
}

#[component]
pub fn ColourButton(colour: Hsv, name: String, on_click: Callback<Hsv>, selected: bool) -> Element {
    let rgb: Srgb = colour.into_color();

    rsx! {
        button {
            class: "p-5 m-1 inline-block",
            class: if selected { "border-4 border-green-400" } else { "border-2 border-white" },
            class: if colour.value < 0.5 { "text-white" } else { "text-black" },
            style: format!(
                "background-color: rgb({}, {}, {})",
                rgb.red * 255.0,
                rgb.green * 255.0,
                rgb.blue * 255.0,
            ),
            onclick: move |_e| on_click(colour),
            {name}
        }
    }
}

#[component]
pub fn InputColour(
    id: &'static str,
    label: &'static str,
    value: Signal<(String, String, String)>,
    validate: Memo<Result<Hsv, ValidationError>>,
    colours: Vec<(String, Hsv)>,
    disabled: Memo<bool>,
) -> Element {
    let (hue, saturation, brightness) = value();

    let hue_id = format!("{}-hue", id);
    let saturation_id = format!("{}-saturation", id);
    let value_id = format!("{}-value", id);

    let validate_hue = use_memo(move || validate_colour_hue(&value().0));
    let validate_saturation = use_memo(move || validate_colour_saturation(&value().1));
    let validate_value = use_memo(move || validate_colour_value(&value().2));

    let colour: Option<Hsv> = validate().ok();
    let rgb_colour: Option<Srgb> = colour.map(|x| x.into_color());

    rsx! {
        label { r#for: id, class: get_label_classes(), "{label}" }

        div {
            div { class: "mb-5 w-20 ml-10 mr-2 inline-block",
                label { r#for: hue_id.clone(), class: get_label_classes(), "Hue" }
                input {
                    r#type: "text",
                    class: get_input_classes(validate_hue().is_ok(), disabled()),
                    id: hue_id,
                    r#type: "number",
                    pattern: "[0-9]*",
                    inputmode: "numeric",
                    placeholder: "Enter input",
                    value: hue,
                    disabled,
                    oninput: move |e| {
                        let mut v = value();
                        v.0 = e.value();
                        value.set(v);
                    },
                }
                FieldMessage { validate: validate_hue, disabled }
            }

            div { class: "mb-5 w-20 mr-2 inline-block",
                label { r#for: saturation_id.clone(), class: get_label_classes(), "Saturation" }
                input {
                    r#type: "text",
                    class: get_input_classes(validate_saturation().is_ok(), disabled()),
                    id: saturation_id,
                    r#type: "number",
                    pattern: "[0-9]*",
                    inputmode: "numeric",
                    placeholder: "Enter input",
                    value: saturation,
                    disabled,
                    oninput: move |e| {
                        let mut v = value();
                        v.1 = e.value();
                        value.set(v);
                    },
                }
                FieldMessage { validate: validate_saturation, disabled }
            }

            div { class: "mb-5 w-20 mr-2 inline-block",
                label { r#for: value_id.clone(), class: get_label_classes(), "Value" }
                input {
                    r#type: "text",
                    class: get_input_classes(validate_value().is_ok(), disabled()),
                    id: value_id,
                    r#type: "number",
                    pattern: "[0-9]*",
                    inputmode: "numeric",
                    placeholder: "Enter input",
                    value: brightness,
                    disabled,
                    oninput: move |e| {
                        let mut v = value();
                        v.2 = e.value();
                        value.set(v);
                    },
                }
                FieldMessage { validate: validate_value, disabled }
            }


            if let Some(colour) = rgb_colour {
                div {
                    class: "w-40 h-40 m-1 inline-block border-2 border-white",
                    style: format!(
                        "background-color: rgb({}, {}, {})",
                        colour.red * 255.0,
                        colour.green * 255.0,
                        colour.blue * 255.0,
                    ),
                }
            }
        }

        div {
            for (button_name , button_colour) in colours {
                ColourButton {
                    colour: button_colour,
                    name: button_name,
                    on_click: move |c: Hsv| {
                        value
                            .set((
                                c.hue.into_inner().to_string(),
                                c.saturation.to_string(),
                                c.value.to_string(),
                            ))
                    },
                    selected: Some(button_colour) == colour,
                }
            }
        }
        FieldMessage { validate, disabled }
    }
}

#[component]
pub fn InputConsumable(
    id: &'static str,
    label: &'static str,
    value: Signal<Option<Consumable>>,
    disabled: Memo<bool>,
    on_create: Callback<Consumable>,
    on_change: Callback<Option<Consumable>>,
    mut create_form: Signal<bool>,
) -> Element {
    let search = use_signal(String::new);

    let list: Resource<Result<Vec<PullDownMenuItem<Consumable>>, ServerFnError>> =
        use_resource(move || async move {
            let query = search();
            if query.is_empty() {
                return Ok(Vec::new());
            }

            search_consumables(query, false, false)
                .await
                .unwrap()
                .into_iter()
                .map(|consumable| {
                    let id = consumable.id.to_string();
                    let icon = rsx! {
                        consumables::ConsumableIcon {}
                    };
                    let label = rsx! {
                        div {
                            ConsumableLabel { consumable: consumable.clone() }
                        }
                    };
                    PullDownMenuItem {
                        id,
                        value: Some(consumable.clone()),
                        label,
                        icon,
                    }
                })
                .collect::<Vec<_>>()
                .pipe(Ok)
        });

    let filtered_options =
        use_memo(move || list().unwrap_or_else(|| Ok(Vec::new())).unwrap_or_default());

    let validate = use_memo(move || Ok(value()));

    rsx! {
        if create_form() {
            ConsumableUpdate {
                op: consumables::Operation::Create {},
                on_cancel: move || create_form.set(false),
                on_save: move |consumable: Consumable| {
                    value.set(Some(consumable.clone()));
                    create_form.set(false);
                    on_create(consumable);
                },
            }
        } else {
            if let Some(Err(err)) = list.read().deref() {
                div { class: "alert alert-error",
                    "Error loading consumables: "
                    {err.to_string()}
                }
            }

            // FIXME: Should be we do something to indicate loading state? Spinner?
            // if let None = list.read().deref() {
            //     p { class: "alert alert-info", "Loading..." }
            // }

            {
                value
                    .read()
                    .deref()
                    .as_ref()
                    .map(|consumable| rsx! {
                        div {
                            class: "bg-green-500 rounded-sm border-green-100 text-white p-2 mb-2",
                            onclick: move |_e| {
                                value.set(None);
                                on_change(None);
                            },
                            {consumable.name.clone()}
                        }
                    })
            }
            InputSearch {
                id,
                label,
                validate,
                value,
                disabled,
                options: filtered_options,
                search,
                on_change: Some(on_change),
            }
            div { class: "gap-2",
                CreateButton { on_click: move |_e| create_form.set(true), "Create" }
                Barcode { barcode: search }
            }
        }
    }
}
