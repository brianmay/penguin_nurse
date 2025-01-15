use std::ops::Deref;

use chrono::Local;
use dioxus::prelude::*;

use crate::{
    components::consumables::{ActiveDialog, ConsumableDialog, Operation},
    functions::consumables::search_consumables,
    models::{Consumable, ConsumableId, Maybe},
};

#[component]
fn EntryRow(
    consumable: Consumable,
    dialog: Signal<ActiveDialog>,
    selected: Signal<Option<ConsumableId>>,
) -> Element {
    let id = consumable.id;
    let consumable_clone_1 = consumable.clone();
    let consumable_clone_2 = consumable.clone();
    let consumable_clone_3 = consumable.clone();
    let consumable_clone_4 = consumable.clone();

    rsx! {
        tr {
            class: "hover:bg-gray-500 border-blue-300 m-2 p-2 border-2 h-96 w-48 sm:w-auto sm:border-none sm:h-auto inline-block sm:table-row",
            onclick: move |_| { selected.set(Some(id)) },
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

        if selected() == Some(id) {
            tr {
                td { colspan: "6", class: "block sm:table-cell",
                    button {
                        class: "btn btn-primary m-1",
                        onclick: move |_| { dialog.set(ActiveDialog::Details(consumable_clone_1.clone())) },
                        "Details"
                    }
                    button {
                        class: "btn btn-primary m-1",
                        onclick: move |_| { dialog.set(ActiveDialog::Nested(consumable_clone_2.clone())) },
                        "Ingredients"
                    }
                    button {
                        class: "btn btn-primary m-1",
                        onclick: move |_| {
                            dialog
                                .set(
                                    ActiveDialog::Change(Operation::Update {
                                        consumable: consumable_clone_3.clone(),
                                    }),
                                )
                        },
                        "Edit"
                    }
                    button {
                        class: "btn btn-secondary m-1",
                        onclick: move |_| { dialog.set(ActiveDialog::Delete(consumable_clone_4.clone())) },
                        "Delete"
                    }
                }
            }
        }
    }
}

#[component]
pub fn ConsumableList() -> Element {
    let selected: Signal<Option<ConsumableId>> = use_signal(|| None);
    let mut show_only_created = use_signal(|| false);
    let mut show_destroyed = use_signal(|| false);

    let mut query = use_signal(|| "".to_string());
    let mut dialog = use_signal(|| ActiveDialog::Idle);

    let mut list: Resource<Result<Vec<Consumable>, ServerFnError>> =
        use_resource(move || async move {
            search_consumables(query(), show_only_created(), show_destroyed()).await
        });

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

            div {
                label {
                    r#for: "show_only_created",
                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                    "Show only created"
                }
                input {
                    r#type: "checkbox",
                    class: "checkbox",
                    checked: show_only_created(),
                    oninput: move |e| {
                        show_only_created.set(e.checked());
                    },
                }
            }

            div {
                label {
                    r#for: "show_destroyed",
                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                    "Show destroyed"
                }
                input {
                    r#type: "checkbox",
                    class: "checkbox",
                    checked: show_destroyed(),
                    oninput: move |e| {
                        show_destroyed.set(e.checked());
                    },
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
                                selected,
                                dialog,
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
