use dioxus::prelude::*;

#[component]
pub fn Dialog(children: Element) -> Element {
    rsx! {
        dialog { class: "modal modal-open w-screen h-screen",
            div { class: "modal-box w-full h-full max-h-none md:w-auto md:h-auto md:max-h-[calc(100vh-5em)]",
                {children}
            }
        }
    }
}
