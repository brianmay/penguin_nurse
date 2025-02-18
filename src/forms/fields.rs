#![allow(non_snake_case)]
use chrono::{DateTime, FixedOffset, Local, TimeDelta, Utc};
use dioxus::{prelude::*, signals::Signal};
use palette::{Hsv, IntoColor, Srgb};
use std::ops::Deref;
use tap::Pipe;

use crate::{
    components::consumables::{self, ChangeConsumable},
    forms::{validate_colour_hue, validate_colour_saturation, validate_colour_value},
    functions::consumables::search_consumables,
    models::{Consumable, Maybe, MaybeDateTime},
};

use super::errors::ValidationError;
use super::FieldValue;

fn get_input_classes(is_valid: bool, changed: bool, is_disabled: bool) -> &'static str {
    if is_disabled {
        return "border-gray-300 dark:border-gray-600";
    }

    if is_valid {
        return "border-green-500 dark:border-green-500";
    }

    if !changed {
        return "";
    }

    "border-red-500 dark:border-red-500"
}

#[component]
pub fn InputString<D: 'static + Clone + PartialEq>(
    id: &'static str,
    label: &'static str,
    value: Signal<String>,
    validate: Memo<Result<D, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    let mut changed = use_signal(|| false);

    rsx! {
        div { class: "mb-5",
            label {
                r#for: id,
                class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                "{label}"
            }
            input {
                r#type: "text",
                class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 "
                    .to_string() + get_input_classes(validate().is_ok(), changed(), disabled()),
                id,
                placeholder: "Enter input",
                value: "{value()}",
                disabled,
                oninput: move |e| {
                    changed.set(true);
                    value.set(e.value());
                },
            }
            if disabled() || !changed() {

            } else if let Err(err) = validate() {
                div { class: "text-red-500", "{err}" }
            } else {
                div { class: "text-green-500", "Looks good!" }
            }
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
    let mut changed = use_signal(|| false);

    rsx! {
        div { class: "mb-5",
            label {
                r#for: id,
                class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                "{label}"
            }
            input {
                r#type: "text",
                class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 "
                    .to_string() + get_input_classes(validate().is_ok(), changed(), disabled()),
                id,
                r#type: "number",
                pattern: "[0-9]*",
                inputmode: "numeric",
                placeholder: "Enter input",
                value: "{value()}",
                disabled,
                oninput: move |e| {
                    changed.set(true);
                    value.set(e.value());
                },
            }
            if disabled() || !changed() {

            } else if let Err(err) = validate() {
                div { class: "text-red-500", "{err}" }
            } else {
                div { class: "text-green-500", "Looks good!" }
            }
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
    let mut changed: Signal<bool> = use_signal(|| false);

    rsx! {
        div { class: "my-5",
            label {
                r#for: id,
                class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                "{label}"
            }
            input {
                r#type: "password",
                class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 "
                    .to_string() + get_input_classes(validate().is_ok(), changed(), disabled()),
                id,
                placeholder: "Enter input",
                value: value(),
                disabled,
                oninput: move |e| {
                    changed.set(true);
                    value.set(e.value());
                },
            }
            if disabled() || !changed() {

            } else if let Err(err) = validate() {
                div { class: "text-red-500", "{err}" }
            } else {
                div { class: "text-green-500", "Looks good!" }
            }
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
    let mut changed = use_signal(|| false);

    rsx! {
        div { class: "mb-5",
            label {
                r#for: id,
                class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                "{label}"
            }
            textarea {
                class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 "
                    .to_string() + get_input_classes(validate().is_ok(), changed(), disabled()),
                id,
                placeholder: "Enter input",
                value: "{value()}",
                disabled,
                oninput: move |e| {
                    changed.set(true);
                    value.set(e.value());
                },
            }
            if disabled() || !changed() {

            } else if let Err(err) = validate() {
                div { class: "text-red-500", "{err}" }
            } else {
                div { class: "text-green-500", "Looks good!" }
            }
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
    let mut changed = use_signal(|| false);

    rsx! {
        div { class: "mb-5",
            label {
                r#for: id,
                class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                "{label}"
            }
            input {
                r#type: "text",
                class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 "
                    .to_string() + get_input_classes(validate().is_ok(), changed(), disabled()),
                id,
                placeholder: "Enter input",
                value: "{value()}",
                disabled,
                oninput: move |e| {
                    changed.set(true);
                    value.set(e.value());
                },
            }
            button {
                class: "btn btn-secondary",
                onclick: move |_e| {
                    value.set(Local::now().to_rfc3339());
                },
                "Now"
            }
            if disabled() || !changed() {

            } else if let Err(err) = validate() {
                div { class: "text-red-500", "{err}" }
            } else {
                div { class: "text-green-500", "Looks good!" }
            }
        }
    }
}

#[component]
pub fn InputMaybeDateTime(
    id: &'static str,
    label: &'static str,
    value: Signal<String>,
    validate: Memo<Result<MaybeDateTime, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    let mut changed = use_signal(|| false);

    rsx! {
        div { class: "mb-5",
            label {
                r#for: id,
                class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                "{label}"
            }
            input {
                r#type: "text",
                class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 "
                    .to_string() + get_input_classes(validate().is_ok(), changed(), disabled()),
                id,
                placeholder: "Enter input",
                value: "{value()}",
                disabled,
                oninput: move |e| {
                    changed.set(true);
                    value.set(e.value());
                },
            }
            button {
                class: "btn btn-secondary",
                onclick: move |_e| {
                    value.set(Local::now().to_rfc3339());
                },
                "Now"
            }
            if disabled() || !changed() {

            } else if let Err(err) = validate() {
                div { class: "text-red-500", "{err}" }
            } else {
                div { class: "text-green-500", "Looks good!" }
            }
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
    let mut changed = use_signal(|| false);

    rsx! {
        div { class: "mb-5",
            label {
                r#for: id,
                class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                "{label}"
            }
            input {
                r#type: "text",
                class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 "
                    .to_string() + get_input_classes(validate().is_ok(), changed(), disabled()),
                id,
                r#type: "number",
                pattern: "[0-9]*",
                inputmode: "numeric",
                placeholder: "Enter input",
                value: "{value()}",
                disabled,
                oninput: move |e| {
                    changed.set(true);
                    value.set(e.value());
                },
            }
            if let Ok(start_time) = start_time() {
                button {
                    class: "btn btn-secondary",
                    onclick: move |_e| {
                        let now: DateTime<FixedOffset> = Utc::now().into();
                        value.set((now - start_time).as_string());
                    },
                    "Stop"
                }
            }
            if disabled() || !changed() {

            } else if let Err(err) = validate() {
                div { class: "text-red-500", "{err}" }
            } else {
                div { class: "text-green-500", "Looks good!" }
            }
        }
    }
}

#[component]
pub fn InputSelect<D: 'static + Clone + Eq + PartialEq>(
    id: &'static str,
    label: &'static str,
    validate: Memo<Result<D, ValidationError>>,
    value: Signal<String>,
    disabled: Memo<bool>,
    options: Vec<(&'static str, &'static str)>,
) -> Element {
    let mut changed: Signal<bool> = use_signal(|| false);

    rsx! {
        div { class: "form-group",
            label { r#for: id, "{label}" }
            select {
                class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 "
                    .to_string() + get_input_classes(validate().is_ok(), changed(), disabled()),
                id: "input",
                disabled,
                oninput: move |e| {
                    changed.set(true);
                    value.set(e.value());
                },
                value: value(),
                option { value: "", label: "Select..." }
                for (id , label) in options {
                    option { value: id, label, selected: id == value() }
                }
            }
            if disabled() || !changed() {

            } else if let Err(err) = validate() {
                div { class: "text-red-500", "{err}" }
            } else {
                div { class: "text-green-500", "Looks good!" }
            }
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
    let mut changed = use_signal(|| false);

    rsx! {
        div {
            label {
                r#for: id,
                class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                "{label}"
            }
            input {
                r#type: "checkbox",
                class: "checkbox ".to_string() + get_input_classes(true, changed(), disabled()),
                id,
                checked: value(),
                disabled,
                oninput: move |e| {
                    changed.set(true);
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
    let mut changed = use_signal(|| false);
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
        label {
            r#for: id,
            class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
            "{label}"
        }

        div {
            div { class: "mb-5 w-20 ml-10 mr-2 inline-block",
                label {
                    r#for: hue_id.clone(),
                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                    "Hue"
                }
                input {
                    r#type: "text",
                    class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 "
                        .to_string() + get_input_classes(validate_hue().is_ok(), changed(), disabled()),
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
                        changed.set(true);
                        value.set(v);
                    },
                }
                if disabled() {

                }
                if !changed() {

                } else if let Err(err) = validate_hue() {
                    div { class: "text-red-500", "{err}" }
                } else {
                    div { class: "text-green-500", "Looks good!" }
                }
            }

            div { class: "mb-5 w-20 mr-2 inline-block",
                label {
                    r#for: saturation_id.clone(),
                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                    "Saturation"
                }
                input {
                    r#type: "text",
                    class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 "
                        .to_string()
                        + get_input_classes(validate_saturation().is_ok(), changed(), disabled()),
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
                        changed.set(true);
                    },
                }
                if disabled() || !changed() {

                } else if let Err(err) = validate_saturation() {
                    div { class: "text-red-500", "{err}" }
                } else {
                    div { class: "text-green-500", "Looks good!" }
                }
            }

            div { class: "mb-5 w-20 mr-2 inline-block",
                label {
                    r#for: value_id.clone(),
                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                    "Value"
                }
                input {
                    r#type: "text",
                    class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 "
                        .to_string()
                        + get_input_classes(validate_value().is_ok(), changed(), disabled()),
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
                        changed.set(true);
                        value.set(v);
                    },
                }
                if disabled() {

                }
                if !changed() {

                } else if let Err(err) = validate_value() {
                    div { class: "text-red-500", "{err}" }
                } else {
                    div { class: "text-green-500", "Looks good!" }
                }
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
        if disabled() || !changed() {

        } else if let Err(err) = validate() {
            div { class: "text-red-500", "{err}" }
        } else {
            div { class: "text-green-500", "Looks good!" }
        }
    }
}

#[component]
pub fn InputConsumable(
    id: &'static str,
    label: &'static str,
    value: Signal<Option<Consumable>>,
    disabled: Memo<bool>,
    on_change: Callback<Option<Consumable>>,
) -> Element {
    let mut query = use_signal(|| "".to_string());
    let mut create_form = use_signal(|| false);

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
                ChangeConsumable {
                    op: consumables::Operation::Create {},
                    on_cancel: move || create_form.set(false),
                    on_save: move |consumable: Consumable| {
                        value.set(Some(consumable.clone()));
                        on_change(Some(consumable));
                        create_form.set(false);
                    },
                }
            } else if let Some(consumable) = value() {
                div {
                    class: "bg-green-500 rounded border-green-100 text-white p-2",
                    onclick: move |_e| {
                        value.set(None);
                        on_change(None);
                    },
                    {consumable.name.clone()}
                }
            } else {
                div { class: "mb-5 w-20 mr-2 inline-block",
                    label {
                        r#for: id,
                        class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                        "{label}"
                    }
                    input {
                        class: "form-control",
                        r#type: "text",
                        value: query(),
                        oninput: move |e| query.set(e.value()),
                        id,
                        placeholder: "Search...",
                    }
                    match list.read().deref() {
                        Some(Some(Err(err))) => rsx! {
                            div { class: "alert alert-danger",
                                "Error loading consumables: "
                                {err.to_string()}
                            }
                        },
                        Some(Some(Ok(list))) if list.is_empty() => rsx! {
                            p { class: "alert alert-info", "No entries found." }
                        },
                        Some(Some(Ok(list))) => rsx! {
                            ul { class: "menu dropdown-content bg-gray-800 rounded-box z-[1] w-52 p-2 shadow",
                                for consumable in list.iter().cloned() {
                                    li {
                                        a {
                                            onclick: move |_e| {
                                                value.set(Some(consumable.clone()));
                                                on_change(Some(consumable.clone()));
                                                query.set("".to_string());
                                            },
                                            {consumable.name.clone()}
                                            if let Maybe::Some(brand) = &consumable.brand {
                                                ", "
                                                {brand.clone()}
                                            }
                                            if let Maybe::Some(dt) = consumable.created {
                                                {dt.with_timezone(&Local).format(" %Y-%m-%d").to_string()}
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        Some(None) => rsx! {
                            button { class: "btn btn-secondary", onclick: move |_e| create_form.set(true), "Create" }
                        },
                        None => {
                            rsx! {
                                p { class: "alert alert-info", "Loading..." }
                            }
                        }
                    }
                }
            }
        }
    }
}
