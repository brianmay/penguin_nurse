use chrono::Local;
use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Home() -> Element {
    let date = Local::now().date_naive();
    rsx! {
        div {
            h1 { "Welcome to Penguin Nurse" }
            p { "This is a simple web application written in Rust using the Dioxus framework." }
            button {
                class: "btn btn-primary",
                onclick: move |_| {
                    navigator().push(Route::TimelineList { date });
                },
                "Today"
            }
        }
    }
}
