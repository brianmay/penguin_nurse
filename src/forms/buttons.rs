use dioxus::{prelude::*, signals::Memo};

#[component]
pub fn FormCancelButton(title: String, on_cancel: Callback<()>) -> Element {
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
pub fn FormEditButton(title: String, on_edit: Callback<()>) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: "w-full btn btn-secondary my-2",
            onclick: move |_e| on_edit(()),
            {title}
        }
    }
}

#[component]
pub fn FormDeleteButton(title: String, on_delete: Callback<()>) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: "w-full btn btn-secondary my-2",
            onclick: move |_e| on_delete(()),
            {title}
        }
    }
}

#[component]
pub fn FormSubmitButton(disabled: Memo<bool>, title: String, on_save: Callback<()>) -> Element {
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
pub fn FormCloseButton(title: String, on_close: Callback<()>) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: "w-full btn btn-secondary my-2",
            onclick: move |_e| on_close(()),
            {title}
        }
    }
}
