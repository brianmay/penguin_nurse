#![allow(non_snake_case)]
use chrono::{DateTime, FixedOffset, Local, TimeDelta, Utc};
use classes::classes;
use dioxus::{prelude::*, signals::Signal};
use palette::{Hsv, IntoColor, Srgb};
use std::ops::Deref;
use tap::Pipe;

use crate::{
    components::{
        buttons::{ActionButton, CreateButton},
        consumables::{self, ConsumableLabel, ConsumableUpdate},
        consumptions::{CONSUMPTION_TYPES, consumption_icon, consumption_id, consumption_title},
        exercises::{
            EXERCISE_TYPES, exercise_calories, exercise_icon, exercise_id, exercise_rpe,
            exercise_title,
        },
    },
    forms::{Barcode, validate_colour_hue, validate_colour_saturation, validate_colour_value},
    functions::consumables::search_consumables,
    models::Consumable,
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

#[derive(Clone, PartialEq)]
struct PullDownMenuItem {
    id: String,
    icon: Element,
    label: Element,
    on_click: Callback<()>,
}

#[component]
fn PullDownMenu(items: Vec<PullDownMenuItem>) -> Element {
    rsx! {
        div { class: "absolute z-10 shadow-lg bg-gray-50 border border-gray-50 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
            ul { tabindex: "0", class: "p-2 shadow rounded-box",
                if items.is_empty() {
                    li { "No entries found." }
                } else {
                    for item in items {
                        li {
                            key: "{item.id}",
                            class: "flex px-4 py-2 hover:bg-gray-800 hover:text-gray-100 cursor-pointer gap-4",
                            onclick: move |_| {
                                item.on_click.call(());
                            },
                            div { class: "w-10 dark:invert inline-block", {item.icon.clone()} }
                            {item.label.clone()}
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn FieldMessage<D: 'static + Clone + PartialEq>(
    validate: Memo<Result<D, ValidationError>>,
    disabled: Memo<bool>,
    message: Option<Element>,
) -> Element {
    rsx! {
        if disabled() {
            div { class: "text-gray-300", "Inactive" }
        } else if let Err(err) = validate() {
            div { class: "text-red-500", "{err}" }
        } else {
            div { class: "text-green-500",
                if message.is_some() {
                    {message}
                } else {
                    "Looks good!"
                }
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
    message: Option<Element>,
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
            FieldMessage { validate, disabled, message }
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
    message: Option<Element>,
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
            FieldMessage { validate, disabled, message }
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
                        value.set((now - start_time).as_string());
                    },
                    "Stop"
                }
            }
            FieldMessage { validate, disabled }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct InputOption {
    id: String,
    icon: Element,
    label: String,
}

#[component]
pub fn InputSelect<D: 'static + Clone + Eq + PartialEq>(
    id: &'static str,
    label: &'static str,
    validate: Memo<Result<D, ValidationError>>,
    value: Signal<String>,
    disabled: Memo<bool>,
    options: Vec<InputOption>,
    message: Option<Element>,
) -> Element {
    let mut open = use_signal(|| false);
    let selected_option = options
        .iter()
        .find(|InputOption { id, .. }| *id == value.read().deref().deref());

    rsx! {
        div { class: "form-group",
            label { class: get_label_classes(), r#for: id, "{label}" }
            div { class: "relative inline-block text-left w-full",
                button {
                    id,
                    class: "inline-flex gap-4 ".to_string() + " "
                        + &get_input_classes(validate().is_ok(), disabled()),
                    onclick: move |_| open.set(!open()),
                    {
                        selected_option
                            .map(|opt| rsx! {
                                div { class: "w-10 dark:invert inline-block", {opt.icon.clone()} }
                                {opt.label.clone()}
                            })
                            .unwrap_or(rsx! { "Select..." })
                    }
                }
                if open() {
                    PullDownMenu {
                        items: options
                            .iter()
                            .map(|opt| {
                                let option_id = opt.id.to_string();
                                PullDownMenuItem {
                                    id: option_id.clone(),
                                    icon: opt.icon.clone(),
                                    label: rsx! {
                                        {opt.label.clone()}
                                    },
                                    on_click: Callback::new(move |()| {
                                        let mut value = value;
                                        let mut open = open;
                                        open.set(false);
                                        value.set(option_id.clone());
                                    }),
                                }
                            })
                            .collect(),
                    }
                }
                FieldMessage { validate, disabled, message }
            }
        }
    }
}

#[component]
pub fn InputConsumptionType(
    id: &'static str,
    label: &'static str,
    value: Signal<String>,
    validate: Memo<Result<crate::models::ConsumptionType, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    let options = CONSUMPTION_TYPES
        .iter()
        .map(|consumption_type| {
            let id = consumption_id(*consumption_type).to_string();
            let icon = rsx! {
                consumption_icon { consumption_type: *consumption_type }
            };
            let label = consumption_title(*consumption_type).to_string();
            InputOption { id, icon, label }
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
    value: Signal<String>,
    validate: Memo<Result<crate::models::ConsumableUnit, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    let options = vec![
        InputOption {
            id: "millilitres".to_string(),
            icon: rsx! { "ml" },
            label: "Millilitres".to_string(),
        },
        InputOption {
            id: "grams".to_string(),
            icon: rsx! { "g" },
            label: "Grams".to_string(),
        },
        InputOption {
            id: "international_units".to_string(),
            icon: rsx! { "IU" },
            label: "International Units".to_string(),
        },
        InputOption {
            id: "number".to_string(),
            icon: rsx! { "N" },
            label: "Number".to_string(),
        },
    ];

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
    value: Signal<String>,
    validate: Memo<Result<crate::models::Bristol, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    let options = vec![
        InputOption {
            id: "0".to_string(),
            icon: rsx! { "0" },
            label: "0. No poo".to_string(),
        },
        InputOption {
            id: "1".to_string(),
            icon: rsx! { "1" },
            label: "1. Separate hard lumps. Rabbit Droppings.".to_string(),
        },
        InputOption {
            id: "2".to_string(),
            icon: rsx! { "2" },
            label: "2. Lumpy and sausage-like. Bunch of Grapes.".to_string(),
        },
        InputOption {
            id: "3".to_string(),
            icon: rsx! { "3" },
            label: "3. Sausage shape with cracks. Corn on Cobb.".to_string(),
        },
        InputOption {
            id: "4".to_string(),
            icon: rsx! { "4" },
            label: "4. Smooth and soft. Sausage.".to_string(),
        },
        InputOption {
            id: "5".to_string(),
            icon: rsx! { "5" },
            label: "5. Soft blobs with clear-cut edges. Chicken Nuggets.".to_string(),
        },
        InputOption {
            id: "6".to_string(),
            icon: rsx! { "6" },
            label: "6. Mushy. Porridge.".to_string(),
        },
        InputOption {
            id: "7".to_string(),
            icon: rsx! { "7" },
            label: "7. Watery. Gravy.".to_string(),
        },
    ];

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
    value: Signal<String>,
    validate: Memo<Result<crate::models::ExerciseType, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    let options = EXERCISE_TYPES
        .iter()
        .map(|exercise_type| {
            let id = exercise_id(*exercise_type).to_string();
            let icon = rsx! {
                exercise_icon { exercise_type: *exercise_type }
            };
            let label = exercise_title(*exercise_type).to_string();
            InputOption { id, icon, label }
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
            message: rsx! {
                if let Ok(calories) = validate.read().as_ref() {
                    div {
                        exercise_calories { calories: *calories }
                    }
                } else {
                    div { "Enter a value between 0 and 10,000" }
                }
            },
        }
    }
}

#[component]
pub fn InputExerciseRpe(
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
            message: rsx! {
                if let Ok(rpe) = validate.read().as_ref() {
                    div {
                        exercise_rpe { rpe: *rpe }
                    }
                } else {
                    div { "Enter a value between 1 and 10" }
                }
            },
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
    let mut query = use_signal(|| "".to_string());

    let list: Resource<Option<Result<Vec<Consumable>, ServerFnError>>> =
        use_resource(move || async move {
            let query = query();
            if query.is_empty() {
                None
            } else {
                search_consumables(query, false, false).await.pipe(Some)
            }
        });

    rsx! {

        div {
            if create_form() {
                ConsumableUpdate {
                    op: consumables::Operation::Create {},
                    on_cancel: move || create_form.set(false),
                    on_save: move |consumable: Consumable| {
                        value.set(Some(consumable.clone()));
                        on_create(consumable);
                        create_form.set(false);
                    },
                }
            } else if let Some(consumable) = value() {
                div {
                    class: "bg-green-500 rounded-sm border-green-100 text-white p-2",
                    onclick: move |_e| {
                        value.set(None);
                        on_change(None);
                    },
                    {consumable.name.clone()}
                }
            } else {
                div { class: "form-group",
                    label { r#for: id, class: get_label_classes(), "{label}" }
                    div { class: "relative inline-block text-left w-full",
                        input {
                            class: "inline-flex gap-4 ".to_string() + " " + &get_input_classes(true, disabled()),
                            r#type: "text",
                            value: query(),
                            oninput: move |e| query.set(e.value()),
                            id,
                            placeholder: "Search...",
                        }
                        match list.read().deref() {
                            Some(Some(Err(err))) => rsx! {
                                div { class: "alert alert-error",
                                    "Error loading consumables: "
                                    {err.to_string()}
                                }
                            },
                            Some(Some(Ok(list))) => rsx! {
                                PullDownMenu {
                                    items: list.iter()
                                        .map(|consumable| {
                                            let consumable = consumable.clone();
                                            PullDownMenuItem {
                                                id: consumable.id.to_string(),
                                                icon: rsx! {
                                                    consumables::consumable_icon {}
                                                },
                                                label: rsx! {
                                                    div {
                                                        ConsumableLabel { consumable: consumable.clone() }
                                                    }
                                                },
                                                on_click: {
                                                    let consumable = consumable.clone();
                                                    Callback::new(move |_| {
                                                        value.set(Some(consumable.clone()));
                                                        on_change(Some(consumable.clone()));
                                                        query.set("".to_string());
                                                    })
                                                },
                                            }
                                        })
                                        .collect(),
                                }
                            },
                            Some(None) => rsx! {},
                            None => {
                                rsx! {
                                    p { class: "alert alert-info", "Loading..." }
                                }
                            }
                        }
                        FieldMessage { validate: use_memo(|| Ok(())), disabled }
                        div { class: "gap-2",
                            CreateButton { on_click: move |_e| create_form.set(true), "Create" }
                            Barcode { barcode: query }
                        }
                    }
                }
            }
        }
    }
}
