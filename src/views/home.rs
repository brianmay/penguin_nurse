use chrono::Local;
use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Home() -> Element {
    let navigator = navigator();
    let date = Local::now().date_naive();
    navigator.replace(Route::TimelineList { date });
    rsx! {
        p { {"Redirecting..."} }
    }
}
