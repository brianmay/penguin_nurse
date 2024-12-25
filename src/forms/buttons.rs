use dioxus::{prelude::*, signals::Memo};

#[component]
pub fn SubmitButton(disabled: Memo<bool>, title: String, on_save: Callback<()>) -> Element {
    let disabled = disabled();
    rsx! {
        button {
            disabled,
            r#type: "submit",
            class: "w-full text-white bg-green-600 hover:bg-green-700 focus:ring-4 focus:outline-none focus:ring-primary-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-primary-600 dark:hover:bg-primary-700 dark:focus:ring-primary-800 disabled:bg-gray-300 disabled:cursor-not-allowed",
            onclick: move |_e| on_save(()),
            {title}
        }
    }
}
