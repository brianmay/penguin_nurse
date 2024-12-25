#![allow(non_snake_case)]
use dioxus::{prelude::*, signals::Signal};

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
        div {
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
pub fn PasswordString<D: 'static + Clone + Eq + PartialEq>(
    id: &'static str,
    label: &'static str,
    value: Signal<String>,
    validate: Memo<Result<D, ValidationError>>,
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

// #[component]
// pub fn InputTextArea<D: 'static + Clone + Eq + PartialEq>(
//     id: &'static str,
//     label: &'static str,
//     validate: Memo<Result<D, ValidationError>>,
//     changed: Signal<bool>,
//     value: Signal<String>,
//     disabled: bool,
// ) -> Element {
//     rsx! {
//         div {
//             class: "form-group",
//             label {
//                 for: id,
//                 "{label}"
//             }
//             textarea {
//                 class: get_input_classes(validate().is_ok(), changed()),
//                 id: id,
//                 placeholder: "Enter input",
//                 value: value(),
//                 disabled: disabled,
//                 oninput: move |e| {
//                     changed.set(true);
//                     value.set(e.value());
//                 }
//             }
//             if let Err(err) = validate() {
//                 div {
//                     class: "invalid-feedback",
//                     "{err}"
//                 }
//             } else {
//                 div {
//                     class: "valid-feedback",
//                     "Looks good!"
//                 }
//             }
//         }
//     }
// }

// #[component]
// pub fn InputSelect<D: 'static + Clone + Eq + PartialEq>(
//     id: &'static str,
//     label: &'static str,
//     validate: Memo<Result<D, ValidationError>>,
//     changed: Signal<bool>,
//     value: Signal<String>,
//     disabled: bool,
//     options: Vec<(&'static str, &'static str)>,
// ) -> Element {
//     rsx! {
//         div {
//             class: "form-group",
//             label {
//                 for: id,
//                 "{label}"
//             }
//             select {
//                 class: get_input_classes(validate().is_ok(), changed()),
//                 id: "input",
//                 disabled: disabled,
//                 oninput: move |e| {
//                     changed.set(true);
//                     value.set(e.value());
//                 },
//                 value: value(),
//                 option {
//                     value: "",
//                     label: "Select..."
//                 }
//                 for (label, value) in options {
//                     option {
//                         value: value,
//                         label
//                     }
//                 }
//             }
//             if let Err(err) = validate() {
//                 div {
//                     class: "invalid-feedback",
//                     "{err}"
//                 }
//             } else {
//                 div {
//                     class: "valid-feedback",
//                     "Looks good!"
//                 }
//             }
//         }
//     }
// }
