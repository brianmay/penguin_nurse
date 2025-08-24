use std::{ops::Deref, time::Duration};

use dioxus::{prelude::*, signals::Memo};
use gloo_timers::future::sleep;

#[component]
pub fn FormCancelButton(on_cancel: Callback<()>) -> Element {
    let mut timer = use_signal(|| None);
    let mut confirm = use_signal(|| false);

    let start = use_callback(move |()| {
        let task = spawn(async move {
            sleep(Duration::from_secs(10)).await;
            confirm.set(false);
        });
        timer.set(Some(task));
    });

    rsx! {
        if confirm() {
            {"Really cancel?"}
            div {
                class: "flex gap-2",
                button {
                    r#type: "button",
                    class: "btn btn-secondary my-2",
                    onclick: move |_e| {
                        timer.read().deref().map(|x| x.cancel());
                        confirm.set(false);
                    },
                    "No"
                }
                button {
                    r#type: "button",
                    class: "btn btn-secondary my-2",
                    onclick: move |_e| {
                        timer.read().deref().map(|x| x.cancel());
                        on_cancel(());
                    },
                    "Yes"
                }
            }
        } else {
            button {
                r#type: "button",
                class: "w-full btn btn-secondary my-2",
                onclick: move |_e| {
                    start(());
                    confirm.set(true);
                },
                "Cancel"
            }
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
