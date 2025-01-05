use std::{ops::Deref, sync::Arc};

use chrono::{Local, NaiveDate};
use dioxus::prelude::*;
use tap::Pipe;

use crate::{
    components::{
        ChangeConsumable, ChangePoo, ChangeWee, ConsumableOperation, PooOperation, WeeOperation,
    },
    dt::get_utc_times_for_date,
    functions::{
        consumables::search_consumables, poos::get_poos_for_time_range,
        wees::get_wees_for_time_range,
    },
    models::{Consumable, Entry, EntryData, MaybeString, Timeline, User},
    views::{
        event::{event_colour, event_time, event_urgency},
        poos::{poo_bristol, poo_duration, poo_icon, poo_quantity},
        wees::{wee_duration, wee_icon, wee_mls},
    },
    Route,
};

#[component]
fn EntryRow(consumable: Consumable, on_click: Callback<Consumable>) -> Element {
    let consumable_clone = consumable.clone();

    rsx! {
        tr {
            class: "hover:bg-gray-500 border-blue-300 m-2 p-2 border-2 h-96 w-48 sm:w-auto sm:border-none sm:h-auto inline-block sm:table-row",
            onclick: move |_| on_click(consumable_clone.clone()),
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2", {consumable.name} }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                if let MaybeString::Some(brand) = &consumable.brand {
                    div { {brand.clone()} }
                }
            }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                {consumable.unit.to_string()}
            }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                if let MaybeString::Some(comments) = &consumable.comments {
                    div { {comments.to_string()} }
                }
            }

        }
    }
}

enum ActiveDialog {
    Change(ConsumableOperation),
    None,
}

#[component]
pub fn ConsumableList() -> Element {
    let user: Signal<Arc<Option<User>>> = use_context();
    let navigator = navigator();

    let user: &Option<User> = &user.read();
    let Some(_user) = user.as_ref() else {
        return rsx! {
            p { class: "alert alert-danger", "You are not logged in." }
        };
    };

    let mut query = use_signal(|| "".to_string());
    let mut active_dialog = use_signal(|| ActiveDialog::None);

    let mut list: Resource<Result<Vec<Consumable>, ServerFnError>> =
        use_resource(move || async move {
            if query().is_empty() {
                Ok(vec![])
            } else {
                search_consumables(query()).await
            }
        });

    rsx! {
        div { class: "ml-2",
            div { class: "mb-2",
                button {
                    class: "btn btn-primary",
                    onclick: move |_| { active_dialog.set(ActiveDialog::Change(ConsumableOperation::Create {})) },
                    "Create"
                }
            }

            div { class: "mb-2",
                input {
                    class: "form-control",
                    r#type: "text",
                    value: query(),
                    oninput: move |e| query.set(e.value()),
                    placeholder: "Search...",
                }
            }
        }

        match list.read().deref() {
            Some(Err(err)) => rsx! {
                div { class: "alert alert-danger",
                    "Error loading consumables: "
                    {err.to_string()}
                }
            },
            Some(Ok(list)) if list.is_empty() => rsx! {
                p { class: "alert alert-info", "No entries found." }
            },
            Some(Ok(list)) => rsx! {
                table { class: "block sm:table",
                    thead { class: "hidden sm:table-header-group",
                        tr {
                            th { "Name" }
                            th { "Brand" }
                            th { "Unit" }
                            th { "Comments" }
                        }
                    }
                    tbody { class: "block sm:table-row-group",
                        for consumable in list.iter() {
                            EntryRow {
                                key: "{consumable.id.as_inner().to_string()}",
                                consumable: consumable.clone(),
                                on_click: move |consumable: Consumable| {},
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
            ActiveDialog::Change(op) => {
                rsx! {
                    ChangeConsumable {
                        op: op.clone(),
                        on_cancel: move || active_dialog.set(ActiveDialog::None),
                        on_save: move |_wee| {
                            active_dialog.set(ActiveDialog::None);
                            list.restart();
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
