use classes::classes;
use dioxus::prelude::*;
use jiff::Timestamp;
use jiff::ZonedDateTime;
use jiff::civil::Time;
use jiff::civil::time;
use jiff::tz::TimeZone;
use palette::IntoColor;

use crate::dt::display_age;
use crate::models::Urgency;

const DAY_START_TIME: Time = time(6, 30, 0, 0);
const DAY_END_TIME: Time = time(21, 0, 0, 0);

fn get_classes_for_time(date_time: ZonedDateTime) -> String {
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
pub fn EventDateTimeShort(time: ZonedDateTime) -> Element {
    let string = time.strftime("%Y-%m-%d %H:%M").to_string();
    let classes = get_classes_for_time(time);

    rsx! {
        span { class: classes, {string} }
    }
}

#[component]
pub fn EventTime(time: ZonedDateTime) -> Element {
    let string = time.strftime("%H:%M:%S %z").to_string();
    let classes = get_classes_for_time(time);

    rsx! {
        span { class: classes, {string} }
    }
}

#[component]
pub fn RecordTimestamps(created_at: Timestamp, updated_at: Timestamp) -> Element {
    let created_at = created_at
        .to_zoned_datetime(TimeZone::system())
        .strftime("%Y-%m-%d %H:%M:%S%.f %:z")
        .to_string();
    let updated_at = updated_at
        .to_zoned_datetime(TimeZone::system())
        .strftime("%Y-%m-%d %H:%M:%S%.f %:z")
        .to_string();

    rsx! {
        div { class: "text-sm text-base-content/60",
            "Created: {created_at} · Updated: {updated_at}"
        }
    }
}

#[component]
pub fn RecordTimestampsCompact(created_at: Timestamp, updated_at: Timestamp) -> Element {
    let now = Timestamp::now();
    let created = display_age(created_at, now);
    let updated = display_age(updated_at, now);

    rsx! {
        div { class: "text-xs text-base-content/60",
            "created {created}, updated {updated}"
        }
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
pub fn event_colour(colour: Option<palette::Hsv>) -> Element {
    if let Some(colour) = colour {
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
    } else {
        rsx! {}
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
