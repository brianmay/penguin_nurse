use dioxus::prelude::*;

#[component]
pub fn Dialog(children: Element) -> Element {
    rsx! {
        dialog { class: "modal modal-open w-screen h-[100dvh]",
            div { class: "modal-box w-full h-full max-h-none md:w-[48rem] md:h-auto md:max-h-[calc(100dvh-5em)]",
                {children}
            }
        }
    }
}
