use chrono::{Local, Timelike, Utc};
use dioxus::prelude::*;
use palette::IntoColor;

#[component]
pub fn event_time(time: chrono::DateTime<Utc>) -> Element {
    let time = time.with_timezone(&Local);
    let string = time.format("%H:%M:%S").to_string();

    if time.hour() < 7 {
        return rsx! {
            span { class: "text-error", {string} }
        };
    } else if time.hour() < 21 {
        return rsx! {
            span { class: "text-success", {string} }
        };
    } else {
        return rsx! {
            span { class: "text-warning", {string} }
        };
    }
}

#[component]
pub fn event_urgency(urgency: i32) -> Element {
    rsx! {
        if urgency == 0 {
            span { class: "text-success", {"No urgency"} }
        } else if urgency == 1 {
            span { class: "text-success", {"Low urgency"} }
        } else if urgency == 2 {
            span { class: "text-success", {"Medium-Low urgency"} }
        } else if urgency == 3 {
            span { class: "text-success", {"Medium urgency"} }
        } else if urgency == 4 {
            span { class: "text-warning", {"Medium-High urgency"} }
        } else if urgency == 5 {
            span { class: "text-error", {"High urgency"} }
        } else {
            span { class: "text-error", {"Unknown urgency"} }
        }
    }
}

#[component]
pub fn event_colour(colour: palette::Hsv) -> Element {
    let colour: palette::Srgb = colour.into_color();

    rsx! {
        div {
            class: "w-20 h-20 m-1 inline-block border-2 border-white",
            style: format!(
                "background-color: rgb({}, {}, {})",
                colour.red * 255.0,
                colour.green * 255.0,
                colour.blue * 255.0,
            ),
        }
    }
}
