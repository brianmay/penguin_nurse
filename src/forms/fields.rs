#![allow(non_snake_case)]
use chrono::Utc;
use dioxus::{prelude::*, signals::Signal};
use futures::{select, StreamExt};
use gloo_timers::future::IntervalStream;
use palette::{Hsv, IntoColor, Srgb};

use crate::forms::{validate_colour_hue, validate_colour_saturation, validate_colour_value};

use super::errors::ValidationError;

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
pub fn InputString<D: 'static + Clone + Eq + PartialEq>(
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
            if disabled() {

            }
            if !changed() {

            } else if let Err(err) = validate() {
                div { class: "text-red-500", "{err}" }
            } else {
                div { class: "text-green-500", "Looks good!" }
            }
        }
    }
}

#[component]
pub fn InputNumber<D: 'static + Clone + Eq + PartialEq>(
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
                type: "number",
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
            if disabled() {

            }
            if !changed() {

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
            if disabled() {

            }
            if !changed() {

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
            if disabled() {

            }
            if !changed() {

            } else if let Err(err) = validate() {
                div { class: "text-red-500", "{err}" }
            } else {
                div { class: "text-green-500", "Looks good!" }
            }
        }
    }
}

enum TimerButton {
    Restart,
    Stop,
}

#[component]
pub fn InputDuration<D: 'static + Clone + Eq + PartialEq>(
    id: &'static str,
    label: &'static str,
    value: Signal<String>,
    validate: Memo<Result<D, ValidationError>>,
    disabled: Memo<bool>,
) -> Element {
    let mut changed = use_signal(|| false);

    let mut stop_watch_running = use_signal(|| false);

    let stopwatch = use_coroutine(move |mut rx: UnboundedReceiver<TimerButton>| async move {
        let mut start = Utc::now();
        let mut interval = IntervalStream::new(1000).fuse();
        let mut running = false;

        loop {
            stop_watch_running.set(running);

            select! {
                maybe_button = rx.next() => {
                    match maybe_button {
                        Some(TimerButton::Restart) => {
                            start = Utc::now();
                            running = true;
                        }
                        Some(TimerButton::Stop) => {
                            running = false;
                        }
                        None => break,
                    }
                }
                _ = interval.next() => {
                }
            }

            if running {
                let duration = Utc::now() - start;
                value.set(duration.num_seconds().to_string());
            }
        }
    });

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
                type: "number",
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
            button {
                class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                onclick: move |_e| {
                    stopwatch.send(TimerButton::Restart);
                },
                "Start"
            }
            if stop_watch_running() {
                button {
                    class: "bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded",
                    onclick: move |_e| {
                        stopwatch.send(TimerButton::Stop);
                    },
                    "Stop"
                }
            }
            if disabled() {

            }
            if !changed() {

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
            if disabled() {

            }
            if !changed() {

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
            if disabled() {

            }
                // if !changed() {

        // } else if let Err(err) = validate() {
        //     div { class: "text-red-500", "{err}" }
        // } else {
        //     div { class: "text-green-500", "Looks good!" }
        // }
        }
    }
}

#[component]
pub fn ColourButton(colour: Hsv, name: String, on_click: Callback<Hsv>) -> Element {
    #[allow(clippy::let_and_return)]
    let class = if colour.value < 0.5 {
        let class = "text-white";
        class
    } else {
        let class = "text-black";
        class
    };

    let rgb: Srgb = colour.into_color();

    rsx! {
        button {
            class: "p-5 m-1 inline-block border-2 border-white {class}",
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

    let colour: Option<Srgb> = validate().ok().map(|x| x.into_color());

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
                    type: "number",
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
                    type: "number",
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
                if disabled() {

                }
                if !changed() {

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
                    type: "number",
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


            if let Some(colour) = colour {
                div {
                    class: "w-20 h-20 m-1 inline-block border-2 border-white",
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
            for (name , colour) in colours {
                ColourButton {
                    colour,
                    name,
                    on_click: move |c: Hsv| {
                        value
                            .set((
                                c.hue.into_inner().to_string(),
                                c.saturation.to_string(),
                                c.value.to_string(),
                            ))
                    },
                }
            }
        }
        if disabled() {

        }
        if !changed() {

        } else if let Err(err) = validate() {
            div { class: "text-red-500", "{err}" }
        } else {
            div { class: "text-green-500", "Looks good!" }
        }
    }
}
