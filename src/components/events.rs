use chrono::{FixedOffset, Timelike};
use classes::classes;
use dioxus::prelude::*;
use palette::IntoColor;

#[component]
pub fn event_time(time: chrono::DateTime<FixedOffset>) -> Element {
    let string = time.format("%H:%M:%S %z").to_string();

    let classes = if time.hour() < 7 {
        classes!["text-error"]
    } else if time.hour() < 21 {
        classes!["text-success"]
    } else {
        classes!["text-warning"]
    };

    rsx! {
        span { class: classes, {string} }
    }
}

#[component]
pub fn event_date_time(time: chrono::DateTime<FixedOffset>) -> Element {
    let string = time.to_string();

    let classes = if time.hour() < 7 {
        classes!["text-error"]
    } else if time.hour() < 21 {
        classes!["text-success"]
    } else {
        classes!["text-warning"]
    };

    rsx! {
        span { class: classes, {string} }
    }
}

#[component]
pub fn event_urgency(urgency: i32) -> Element {
    let text = match urgency {
        0 => "No urgency",
        1 => "Low urgency",
        2 => "Medium urgency",
        3 => "High urgency",
        4 => "Very high urgency",
        5 => "Critical urgency",
        _ => "Unknown urgency",
    };

    let classes = match urgency {
        0 => classes!["text-success"],
        1 => classes!["text-success"],
        2 => classes!["text-success"],
        3 => classes!["text-success"],
        4 => classes!["text-warning"],
        5 => classes!["text-error"],
        _ => classes!["text-error"],
    };

    rsx! {
        span { class: classes, {text} }
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
