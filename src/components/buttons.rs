use dioxus::prelude::*;

#[component]
pub fn NavButton(on_click: Callback<()>, children: Element) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: "btn btn-outline btn-accent mr-2",
            onclick: move |_e| on_click(()),
            {children}
        }
    }
}

#[component]
pub fn CreateButton(on_click: Callback<()>, children: Element) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: "btn btn-outline btn-secondary mr-2",
            onclick: move |_e| on_click(()),
            svg {
                "viewBox": "0 0 24 24",
                "stroke-width": "1.5",
                stroke: "currentColor",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                class: "size-6",
                path {
                    "stroke-linecap": "round",
                    d: "M12 4.5v15m7.5-7.5h-15",
                    "stroke-linejoin": "round",
                }
            }
            {children}
        }
    }
}

#[component]
pub fn ChangeButton(on_click: Callback<()>, children: Element) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: "btn btn-outline btn-secondary mr-2",
            onclick: move |_e| on_click(()),
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                "viewBox": "0 0 24 24",
                stroke: "currentColor",
                fill: "none",
                "stroke-width": "1.5",
                class: "size-6",
                path {
                    "stroke-linejoin": "round",
                    "stroke-linecap": "round",
                    d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L6.832 19.82a4.5 4.5 0 0 1-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 0 1 1.13-1.897L16.863 4.487Zm0 0L19.5 7.125",
                }
            }
            {children}
        }
    }
}

#[component]
pub fn DeleteButton(on_click: Callback<()>, children: Element) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: "btn btn-outline btn-secondary mr-2",
            onclick: move |_e| on_click(()),

            svg {
                xmlns: "http://www.w3.org/2000/svg",
                "viewBox": "0 0 24 24",
                stroke: "currentColor",
                fill: "none",
                "stroke-width": "1.5",
                class: "size-6 inline-block",
                path {
                    "stroke-linecap": "round",
                    d: "m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0",
                    "stroke-linejoin": "round",
                }
            }
            {children}
        }
    }
}

#[component]
pub fn ActionButton(on_click: Callback<()>, children: Element) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: "btn btn-outline btn-secondary mr-2",
            onclick: move |_e| on_click(()),
            svg {
                "viewBox": "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                "stroke-width": "1.5",
                stroke: "currentColor",
                fill: "none",
                class: "size-6",
                path {
                    "stroke-linecap": "round",
                    d: "M15.59 14.37a6 6 0 0 1-5.84 7.38v-4.8m5.84-2.58a14.98 14.98 0 0 0 6.16-12.12A14.98 14.98 0 0 0 9.631 8.41m5.96 5.96a14.926 14.926 0 0 1-5.841 2.58m-.119-8.54a6 6 0 0 0-7.381 5.84h4.8m2.581-5.84a14.927 14.927 0 0 0-2.58 5.84m2.699 2.7c-.103.021-.207.041-.311.06a15.09 15.09 0 0 1-2.448-2.448 14.9 14.9 0 0 1 .06-.312m-2.24 2.39a4.493 4.493 0 0 0-1.757 4.306 4.493 4.493 0 0 0 4.306-1.758M16.5 9a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Z",
                    "stroke-linejoin": "round",
                }
            }
            {children}
        }
    }
}
