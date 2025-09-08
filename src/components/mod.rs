pub mod buttons;
pub mod consumables;
pub mod consumptions;
pub mod events;
pub mod exercises;
pub mod health_metrics;
pub mod navbar;
pub mod notes;
pub mod poos;
pub mod refluxs;
pub mod symptoms;
pub mod timeline;
pub mod users;
pub mod wee_urges;
pub mod wees;

mod times;

use dioxus::prelude::*;

#[component]
pub fn StrIcon(title: &'static str, icon: Element) -> Element {
    rsx! {
        div { class: "text-sm w-10 dark:invert inline-block", {icon} }
        span { class: "text-sm my-auto text-left", {title} }
    }
}

#[component]
pub fn ElementIcon(title: Element, icon: Element) -> Element {
    rsx! {
        div { class: "text-sm w-10 dark:invert inline-block", {icon} }
        span { class: "text-sm my-auto text-left", {title} }
    }
}
