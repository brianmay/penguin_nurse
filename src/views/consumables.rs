use std::{ops::Deref, sync::Arc};

use chrono::Local;
use dioxus::prelude::*;

use crate::{
    components::consumables::{ActiveDialog, ConsumableDialog, Operation},
    functions::consumables::search_consumables,
    models::{Consumable, Maybe, User},
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
                if let Maybe::Some(brand) = &consumable.brand {
                    div { {brand.clone()} }
                }
            }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                {consumable.unit.to_string()}
            }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                if let Maybe::Some(comments) = &consumable.comments {
                    div { {comments.to_string()} }
                }
            }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                if let Maybe::Some(created) = &consumable.created {
                    {created.with_timezone(&Local).to_string()}
                }
            }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                if let Maybe::Some(destroyed) = &consumable.destroyed {
                    {destroyed.with_timezone(&Local).to_string()}
                }
            }
        }
    }
}

#[component]
pub fn ConsumableList() -> Element {
    let user: Signal<Arc<Option<User>>> = use_context();

    let user: &Option<User> = &user.read();
    let Some(_user) = user.as_ref() else {
        return rsx! {
            p { class: "alert alert-danger", "You are not logged in." }
        };
    };

    let mut query = use_signal(|| "".to_string());
    let mut dialog = use_signal(|| ActiveDialog::Idle);

    let mut list: Resource<Result<Vec<Consumable>, ServerFnError>> =
        use_resource(move || async move { search_consumables(query()).await });

    rsx! {
        div { class: "ml-2",
            div { class: "mb-2",
                button {
                    class: "btn btn-primary",
                    onclick: move |_| { dialog.set(ActiveDialog::Change(Operation::Create {})) },
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
                            th { "Created" }
                            th { "Destroyed" }
                        }
                    }
                    tbody { class: "block sm:table-row-group",
                        for consumable in list.iter() {
                            EntryRow {
                                key: "{consumable.id.as_inner().to_string()}",
                                consumable: consumable.clone(),
                                on_click: move |consumable: Consumable| {
                                    dialog.set(ActiveDialog::Details(consumable));
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

        ConsumableDialog {
            dialog,
            on_change: move |_consumable| list.restart(),
            on_delete: move |_consumable| list.restart(),
        }
    }
}
