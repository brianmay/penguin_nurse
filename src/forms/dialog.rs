use dioxus::prelude::*;

#[component]
pub fn Dialog(children: Element) -> Element {
    rsx! {
        dialog { class: "modal modal-open w-screen h-screen",
            div { class: "modal-box w-full h-full max-h-none md:w-100 md:h-100 md:max-h-100",
                {children}
            }
        }
    }
}
