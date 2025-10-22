use chrono::Utc;
use dioxus::prelude::*;
use dioxus_router::navigator;

use crate::{
    Route,
    components::{buttons::NavButton, timeline::DialogReference},
    dt::get_date_for_dt,
    use_user,
};

#[component]
pub fn Home() -> Element {
    let navigator = navigator();
    let user = use_user().ok().flatten();

    rsx! {
        div {
            h1 { class: "text-green-500", "Welcome to Penguin Nurse" }
            p { "This is a  web application written in Rust using the Dioxus framework." }
            p { "Do not eat." }

            if let Some(user) = user {
                p { class: "text-green-300", "Welcome, {user.full_name}!" }
                NavButton {
                    on_click: move |_| {
                        let new_date = get_date_for_dt(Utc::now());
                        navigator
                            .push(Route::TimelineList {
                                date: new_date,
                                dialog: DialogReference::Idle,
                            });
                    },
                    "Today"
                }
            } else {
                p { class: "text-red-600", "Please log in to continue." }
                NavButton {
                    on_click: move |_| {
                        navigator.push(Route::Login {});
                    },
                    "Login"
                }
            }
        }
    }
}
