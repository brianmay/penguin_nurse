use dioxus::{prelude::*, signals::Memo};

#[component]
pub fn CancelButton(title: String, on_cancel: Callback<()>) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: "w-full btn btn-secondary my-2",
            onclick: move |_e| on_cancel(()),
            {title}
        }
    }
}

#[component]
pub fn DeleteButton(title: String, on_delete: Callback<()>) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: "w-full btn btn-error my-2",
            onclick: move |_e| on_delete(()),
            {title}
        }
    }
}

#[component]
pub fn SubmitButton(disabled: Memo<bool>, title: String, on_save: Callback<()>) -> Element {
    let disabled = disabled();
    rsx! {
        button {
            disabled,
            r#type: "submit",
            class: "w-full btn btn-primary my-2",
            onclick: move |_e| on_save(()),
            {title}
        }
    }
}

#[component]
pub fn CloseButton(title: String, on_close: Callback<()>) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: "w-full btn btn-secondary my-2",
            onclick: move |_e| on_close(()),
            {title}
        }
    }
}
