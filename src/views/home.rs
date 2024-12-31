use std::sync::Arc;

use crate::{
    components::{ChangePoo, ChangeWee, DeletePoo, DeleteWee, PooOperation, WeeOperation},
    functions::{poos::get_poos_for_time_range, wees::get_wees_for_time_range},
    models::{Bristol, Entry, EntryData, MaybeString, Poo, Timeline, User, Wee},
};
use chrono::{DateTime, Local, NaiveDate, TimeZone, Timelike, Utc};
use dioxus::prelude::*;
use palette::IntoColor;
use server_fn::error::NoCustomError;
use tap::Pipe;
use tracing::error;

const WEE_SVG: Asset = asset!("/assets/wee.svg");
const POO_SVG: Asset = asset!("/assets/poo.svg");

#[component]
fn event_time(time: chrono::DateTime<Utc>) -> Element {
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
fn event_urgency(urgency: i32) -> Element {
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
fn event_colour(colour: palette::Hsv) -> Element {
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
fn wee_delta(delta: chrono::Duration) -> Element {
    rsx! {
        if delta.num_seconds() == 0 {
            span { class: "text-error", {delta.num_seconds().to_string() + " seconds"} }
        } else if delta.num_seconds() < 120 {
            span { class: "text-success", {delta.num_seconds().to_string() + " seconds"} }
        } else if delta.num_minutes() < 60 {
            span { class: "text-warning", {delta.num_minutes().to_string() + " minutes"} }
        } else if delta.num_hours() < 24 {
            span { class: "text-error", {delta.num_hours().to_string() + " hours"} }
        } else {
            span { class: "text-error", {delta.num_days().to_string() + " days"} }
        }
    }
}

#[component]
fn wee_mls(mls: i32) -> Element {
    rsx! {
        if mls == 0 {
            span { class: "text-error", {mls.to_string() + " ml"} }
        } else if mls < 100 {
            span { class: "text-warning", {mls.to_string() + " ml"} }
        } else if mls < 500 {
            span { class: "text-success", {mls.to_string() + " ml"} }
        } else {
            span { class: "text-error", {mls.to_string() + " ml"} }
        }
    }
}

#[component]
fn poo_delta(delta: chrono::Duration) -> Element {
    rsx! {
        if delta.num_seconds() == 0 {
            span { class: "text-error", {delta.num_seconds().to_string() + " seconds"} }
        } else if delta.num_seconds() < 60 {
            span { class: "text-success", {delta.num_seconds().to_string() + " seconds"} }
        } else if delta.num_minutes() < 60 {
            span { class: "text-warning", {delta.num_minutes().to_string() + " minutes"} }
        } else if delta.num_hours() < 24 {
            span { class: "text-error", {delta.num_hours().to_string() + " hours"} }
        } else {
            span { class: "text-error", {delta.num_days().to_string() + " days"} }
        }
    }
}

#[component]
fn poo_bristol(bristol: Bristol) -> Element {
    let bristol_string = bristol.as_str();

    match bristol {
        Bristol::B0 => rsx! {
            span { class: "text-error", {bristol_string} }
        },
        Bristol::B1 => rsx! {
            span { class: "text-error", {bristol_string} }
        },
        Bristol::B2 => rsx! {
            span { class: "text-success", {bristol_string} }
        },
        Bristol::B3 => rsx! {
            span { class: "text-success", {bristol_string} }
        },
        Bristol::B4 => rsx! {
            span { class: "text-success", {bristol_string} }
        },
        Bristol::B5 => rsx! {
            span { class: "text-warning", {bristol_string} }
        },

        Bristol::B6 => rsx! {
            span { class: "text-warning", {bristol_string} }
        },
        Bristol::B7 => rsx! {
            span { class: "text-error", {bristol_string} }
        },
    }
}

#[component]
fn poo_quantity(quantity: i32) -> Element {
    rsx! {
        if quantity == 0 {
            span { class: "text-error", {quantity.to_string() + " out of 5"} }
        } else if quantity < 2 {
            span { class: "text-warning", {quantity.to_string() + " out of 5"} }
        } else {
            span { class: "text-success", {quantity.to_string() + " out of 5"} }
        }
    }
}

fn get_utc_times_for_date(
    date: NaiveDate,
) -> Result<(DateTime<Utc>, DateTime<Utc>), ServerFnError> {
    let today = date;
    let tomorrow = today.succ_opt().ok_or_else(|| {
        error!("Failed to get tomorrow's date for date: {:?}", today);
        ServerFnError::<NoCustomError>::ServerError("Failed to get tomorrow's date".to_string())
    })?;

    let start = today.and_hms_opt(7, 0, 0).map_or_else(
        || {
            error!("Failed to create start time for date: {:?}", today);
            Err(ServerFnError::<NoCustomError>::ServerError(
                "Failed to create start time".to_string(),
            ))
        },
        |x| Ok(Local.from_local_datetime(&x)),
    )?;

    let end = tomorrow.and_hms_opt(7, 0, 0).map_or_else(
        || {
            error!("Failed to create end time for date: {:?}", tomorrow);
            Err(ServerFnError::<NoCustomError>::ServerError(
                "Failed to create end time".to_string(),
            ))
        },
        |x| Ok(Local.from_local_datetime(&x)),
    )?;

    let start = start.single().unwrap_or_else(|| {
        error!("Failed to convert start time to UTC for date: {:?}", today);
        panic!("Failed to convert start time to UTC");
    });

    let end = end.single().unwrap_or_else(|| {
        error!("Failed to convert end time to UTC for date: {:?}", tomorrow);
        panic!("Failed to convert end time to UTC");
    });

    let start = start.with_timezone(&Utc);
    let end = end.with_timezone(&Utc);

    Ok((start, end))
}

#[component]
fn EntryRow(entry: Entry, on_click: Callback<Entry>) -> Element {
    rsx! {
        tr {
            class: "hover:bg-gray-500 border-blue-300 m-2 p-2 border-2 h-96 w-48 sm:w-auto sm:border-none sm:h-auto inline-block sm:table-row",
            onclick: move |_| on_click(entry.clone()),
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                event_time { time: entry.time }
            }
            match &entry.data {
                EntryData::Wee(wee) => {
                    rsx! {
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            img { class: "w-10 invert inline-block", alt: "Wee", src: WEE_SVG }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            wee_delta { delta: wee.duration }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            event_colour { colour: wee.colour }
                            div {
                                div {
                                    wee_mls { mls: wee.mls }
                                }
                                div {
                                    event_urgency { urgency: wee.urgency }
                                }
                                if let MaybeString::Some(comments) = &wee.comments {
                                    div { {comments.to_string()} }
                                }
                            }
                        }
                    }
                }
                EntryData::Poo(poo) => {
                    rsx! {
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            img { class: "w-10 invert inline-block", alt: "Poo", src: POO_SVG }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            poo_delta { delta: poo.duration }
                        }
                        td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                            event_colour { colour: poo.colour }
                            div {
                                div {
                                    poo_bristol { bristol: poo.bristol }
                                }
                                div {
                                    poo_quantity { quantity: poo.quantity }
                                }
                                div {
                                    event_urgency { urgency: poo.urgency }
                                }
                                if let MaybeString::Some(comments) = &poo.comments {
                                    div { {comments.to_string()} }
                                }
                            }
                        }
                    }
                }
            }
        
        }
    }
}

enum ActiveDialog {
    ChangeWee(WeeOperation),
    ChangePoo(PooOperation),
    DeleteWee(Arc<Wee>),
    DeletePoo(Arc<Poo>),
    None,
}

#[component]
pub fn Home() -> Element {
    let user: Signal<Arc<Option<User>>> = use_context();

    let user: &Option<User> = &user.read();
    let Some(user) = user.as_ref() else {
        return rsx! {
            p { class: "alert alert-danger", "You are not logged in." }
        };
    };

    let user_id = user.pipe(|x| x.id);

    let mut active_dialog = use_signal(|| ActiveDialog::None);

    let mut date = use_signal(|| {
        let now = Local::now();
        now.date_naive()
    });

    let mut timeline: Resource<Result<Timeline, ServerFnError>> =
        use_resource(move || async move {
            let today = &*date.read();
            let (start, end) = get_utc_times_for_date(*today)?;

            let mut timeline = Timeline::new();
            let wees = get_wees_for_time_range(user_id, start, end).await?;
            timeline.add_wees(wees);

            let poos = get_poos_for_time_range(user_id, start, end).await?;
            timeline.add_poos(poos);

            timeline.sort();

            Ok(timeline)
        });

    let x = timeline.read();

    rsx! {
        div { class: "ml-2",
            div { class: "mb-2",
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        active_dialog.set(ActiveDialog::ChangeWee(WeeOperation::Create { user_id }))
                    },
                    "Wee"
                }
                button {
                    class: "btn btn-primary ml-2",
                    onclick: move |_| {
                        active_dialog.set(ActiveDialog::ChangePoo(PooOperation::Create { user_id }))
                    },
                    "Poo"
                }
            }

            div { class: "mb-2",
                button {
                    class: "btn btn-primary inline-block mr-2",
                    onclick: move |_| {
                        let new_date = date.read().pred_opt();
                        if let Some(new_date) = new_date {
                            date.set(new_date);
                        }
                    },
                    "<"
                }
                p { class: "inline-block", {date.read().to_string()} }
                button {
                    class: "btn btn-primary inline-block ml-2",
                    onclick: move |_| {
                        let new_date = Local::now().date_naive();
                        date.set(new_date);
                    },
                    "today"
                }
                button {
                    class: "btn btn-primary inline-block ml-2",
                    onclick: move |_| {
                        let new_date = date.read().succ_opt();
                        if let Some(new_date) = new_date {
                            date.set(new_date);
                        }
                    },
                    ">"
                }
            }
        }

        match &*x {
            Some(Err(err)) => rsx! {
                div { class: "alert alert-danger",
                    "Error loading timeline"
                    {err.to_string()}
                }
            },
            Some(Ok(timeline)) if timeline.is_empty() => rsx! {
                p { class: "alert alert-info", "No entries found for this date." }
            },
            Some(Ok(timeline)) => rsx! {
                table { class: "block sm:table",
                    thead { class: "hidden sm:table-header-group",
                        tr {
                            th { "When" }
                            th { "What" }
                            th { "How Long" }
                            th { "Details" }
                        }
                    }
                    tbody { class: "block sm:table-row-group",
                        for entry in timeline.iter() {
                            EntryRow {
                                key: "{entry.get_id().as_str()}",
                                entry: entry.clone(),
                                on_click: move |entry: Entry| {
                                    match &entry.data {
                                        EntryData::Wee(wee) => {
                                            active_dialog
                                                .set(
                                                    ActiveDialog::ChangeWee(WeeOperation::Update {
                                                        wee: wee.clone(),
                                                    }),
                                                );
                                        }
                                        EntryData::Poo(poo) => {
                                            active_dialog
                                                .set(
                                                    ActiveDialog::ChangePoo(PooOperation::Update {
                                                        poo: poo.clone(),
                                                    }),
                                                );
                                        }
                                    }
                                },
                            }
                        }
                    }
                }
            },
            None => {
                rsx! {
                    p { class: "alert alert-info", "Loading..." }
                }
            }
        }

        match &*active_dialog.read() {
            ActiveDialog::ChangeWee(op) => {
                rsx! {
                    ChangeWee {
                        op: op.clone(),
                        on_cancel: move || active_dialog.set(ActiveDialog::None),
                        on_save: move |_wee| {
                            active_dialog.set(ActiveDialog::None);
                            timeline.restart();
                        },
                        on_delete: move |wee| {
                            active_dialog.set(ActiveDialog::DeleteWee(wee));
                            timeline.restart();
                        },
                    }
                }
            }
            ActiveDialog::DeleteWee(wee) => {
                rsx! {
                    DeleteWee {
                        wee: wee.clone(),
                        on_cancel: move || active_dialog.set(ActiveDialog::None),
                        on_delete: move |_wee| {
                            active_dialog.set(ActiveDialog::None);
                            timeline.restart();
                        },
                    }
                }
            }
            ActiveDialog::ChangePoo(op) => {
                rsx! {
                    ChangePoo {
                        op: op.clone(),
                        on_cancel: move || active_dialog.set(ActiveDialog::None),
                        on_save: move |_poo| {
                            active_dialog.set(ActiveDialog::None);
                            timeline.restart();
                        },
                        on_delete: move |poo| {
                            active_dialog.set(ActiveDialog::DeletePoo(poo));
                            timeline.restart();
                        },
                    }
                }
            }
            ActiveDialog::DeletePoo(poo) => {
                rsx! {
                    DeletePoo {
                        poo: poo.clone(),
                        on_cancel: move || active_dialog.set(ActiveDialog::None),
                        on_delete: move |_poo| {
                            active_dialog.set(ActiveDialog::None);
                            timeline.restart();
                        },
                    }
                }
            }
            ActiveDialog::None => {
                rsx! {}
            }
        }
    }
}
