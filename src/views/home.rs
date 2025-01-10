use chrono::Utc;
use dioxus::prelude::*;

use crate::{dt::get_date_for_dt, Route};

#[component]
pub fn Home() -> Element {
    let navigator = navigator();
    rsx! {
        div {
            h1 { "Welcome to Penguin Nurse" }
            p { "This is a  web application written in Rust using the Dioxus framework." }
            p { "Do not eat." }
            button {
                class: "btn btn-primary",
                onclick: move |_| {
                    let new_date = get_date_for_dt(Utc::now());
                    if let Ok(new_date) = new_date {
                        navigator
                            .push(Route::TimelineList {
                                date: new_date,
                            });
                    }
                },
                "Today"
            }
        }
    }
}
