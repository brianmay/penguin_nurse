use chrono::FixedOffset;
use classes::classes;
use dioxus::prelude::*;
use palette::IntoColor;

use crate::models::Urgency;

const DAY_START_TIME: chrono::NaiveTime = chrono::NaiveTime::from_hms_opt(6, 30, 0).unwrap();
const DAY_END_TIME: chrono::NaiveTime = chrono::NaiveTime::from_hms_opt(21, 0, 0).unwrap();

fn get_classes_for_time(date_time: chrono::DateTime<FixedOffset>) -> String {
    let time = date_time.time();
    if time < DAY_START_TIME {
        classes!["text-error"]
    } else if time < DAY_END_TIME {
        classes!["text-success"]
    } else {
        classes!["text-warning"]
    }
}

#[component]
pub fn EventDateTimeShort(time: chrono::DateTime<FixedOffset>) -> Element {
    let string = time.format("%Y-%m-%d %H:%M").to_string();
    let classes = get_classes_for_time(time);

    rsx! {
        span { class: classes, {string} }
    }
}

#[component]
pub fn EventTime(time: chrono::DateTime<FixedOffset>) -> Element {
    let string = time.format("%H:%M:%S %z").to_string();
    let classes = get_classes_for_time(time);

    rsx! {
        span { class: classes, {string} }
    }
}

#[component]
pub fn UrgencyIcon(urgency: Urgency) -> Element {
    let (icon, classes) = match urgency {
        Urgency::U0 => ("0", classes!["text-success"]),
        Urgency::U1 => ("1", classes!["text-success"]),
        Urgency::U2 => ("2", classes!["text-success"]),
        Urgency::U3 => ("3", classes!["text-success"]),
        Urgency::U4 => ("4", classes!["text-warning"]),
        Urgency::U5 => ("5", classes!["text-error"]),
    };

    rsx! {
        div { class: classes + "text-sm w-10 dark:invert inline-block", {icon} }
    }
}

#[component]
pub fn UrgencyLabel(urgency: Urgency) -> Element {
    let text = urgency.as_title();

    let classes = match urgency {
        Urgency::U0 => classes!["text-success"],
        Urgency::U1 => classes!["text-success"],
        Urgency::U2 => classes!["text-success"],
        Urgency::U3 => classes!["text-success"],
        Urgency::U4 => classes!["text-warning"],
        Urgency::U5 => classes!["text-error"],
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

#[component]
pub fn Markdown(content: String) -> Element {
    use pulldown_cmark::{Options, Parser, html};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&content, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    rsx! {
        div { class: "prose", dangerous_inner_html: "{html_output}" }
    }
}
