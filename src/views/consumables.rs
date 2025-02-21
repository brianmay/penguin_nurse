use std::ops::Deref;

use chrono::Local;
use dioxus::prelude::*;

use crate::{
    components::{
        buttons::{ChangeButton, CreateButton, DeleteButton, NavButton},
        consumables::{self, ActiveDialog, ConsumableDialog, ConsumableItemList, Operation},
    },
    functions::consumables::{
        get_child_consumables, get_consumable_by_id, search_consumables_with_nested,
    },
    models::{ConsumableId, ConsumableWithItems, Maybe},
    use_user, Route,
};

#[component]
fn EntryRow(
    consumable_with_items: ConsumableWithItems,
    dialog: Signal<ActiveDialog>,
    selected: Signal<Option<ConsumableId>>,
) -> Element {
    let consumable = consumable_with_items.consumable;
    let items = consumable_with_items.items;

    let id = consumable.id;
    let consumable_clone_2 = consumable.clone();
    let consumable_clone_3 = consumable.clone();
    let consumable_clone_4 = consumable.clone();

    rsx! {
        tr {
            class: "hover:bg-gray-500 border-blue-300 mt-2 mb-2 p-2 border-2 w-full sm:w-auto sm:border-none inline-block sm:table-row",
            onclick: move |_| { selected.set(Some(id)) },
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2", {consumable.name} }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                if let Maybe::Some(brand) = &consumable.brand {
                    div { {brand.clone()} }
                }
            }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                span { class: "sm:hidden", "Unit: " }
                {consumable.unit.to_string()}
            }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                ConsumableItemList { list: items }
            }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                if let Maybe::Some(comments) = &consumable.comments {
                    div { {comments.to_string()} }
                }
            }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                if let Maybe::Some(created) = &consumable.created {
                    span { class: "sm:hidden", "Created: " }
                    {created.with_timezone(&Local).to_string()}
                }
            }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                if let Maybe::Some(destroyed) = &consumable.destroyed {
                    span { class: "sm:hidden", "Destroyed: " }
                    {destroyed.with_timezone(&Local).to_string()}
                }
            }
        }

        if selected() == Some(id) {
            tr {
                td { colspan: "6", class: "block sm:table-cell",
                    NavButton {
                        on_click: move |_| {
                            navigator()
                                .push(Route::ConsumableDetail {
                                    consumable_id: id,
                                });
                        },
                        "Details"
                    }
                    ChangeButton { on_click: move |_| { dialog.set(ActiveDialog::Nested(consumable_clone_2.clone())) },
                        "Ingredients"
                    }
                    ChangeButton {
                        on_click: move |_| {
                            dialog
                                .set(
                                    ActiveDialog::Change(Operation::Update {
                                        consumable: consumable_clone_3.clone(),
                                    }),
                                )
                        },
                        "Edit"
                    }
                    ChangeButton { on_click: move |_| { dialog.set(ActiveDialog::Delete(consumable_clone_4.clone())) },
                        "Delete"
                    }
                }
            }
        }
    }
}

#[component]
pub fn ConsumableList() -> Element {
    let user = use_user().ok().flatten();

    let Some(_user) = user.as_ref() else {
        return rsx! {
            p { class: "alert alert-danger", "You are not logged in." }
        };
    };

    let selected: Signal<Option<ConsumableId>> = use_signal(|| None);
    let mut show_only_created = use_signal(|| false);
    let mut show_destroyed = use_signal(|| false);

    let mut query = use_signal(|| "".to_string());
    let mut dialog = use_signal(|| ActiveDialog::Idle);

    let mut list: Resource<Result<Vec<ConsumableWithItems>, ServerFnError>> =
        use_resource(move || async move {
            search_consumables_with_nested(query(), show_only_created(), show_destroyed()).await
        });

    rsx! {
        div { class: "ml-2 mr-2",
            div { class: "mb-2",
                CreateButton { on_click: move |_| { dialog.set(ActiveDialog::Change(Operation::Create {})) },
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
                div { class: "ml-2 mr-2 sm:ml-0 sm:mr-0",
                    table { class: "block sm:table",
                        thead { class: "hidden sm:table-header-group",
                            tr {
                                th { "Name" }
                                th { "Brand" }
                                th { "Unit" }
                                th { "Ingredients" }
                                th { "Comments" }
                                th { "Created" }
                                th { "Destroyed" }
                            }
                        }
                        tbody { class: "block sm:table-row-group",
                            for consumable in list.iter() {
                                EntryRow {
                                    key: "{consumable.consumable.id.as_inner().to_string()}",
                                    consumable_with_items: consumable.clone(),
                                    selected,
                                    dialog,
                                }
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

#[component]
pub fn ConsumableDetail(consumable_id: ReadOnlySignal<ConsumableId>) -> Element {
    let mut maybe_consumable =
        use_resource(move || async move { get_consumable_by_id(consumable_id()).await });

    let mut maybe_items =
        use_resource(move || async move { get_child_consumables(consumable_id()).await });

    let mut active_dialog: Signal<ActiveDialog> = use_signal(|| ActiveDialog::Idle);

    match (maybe_consumable(), maybe_items()) {
        (Some(Ok(Some(consumable))), Some(Ok(items))) => {
            let consumable_clone_2 = consumable.clone();
            let consumable_clone_3 = consumable.clone();
            let consumable_clone_4 = consumable.clone();

            rsx! {
                consumables::ConsumableDetail { consumable, list: items }
                ConsumableDialog {
                    dialog: active_dialog,
                    on_change: move |_consumption| {
                        active_dialog.set(ActiveDialog::Idle);
                        maybe_items.restart();
                        maybe_consumable.restart();
                    },
                    on_delete: move |_consumption| {
                        active_dialog.set(ActiveDialog::Idle);
                        maybe_items.restart();
                        maybe_consumable.restart();
                    },
                }
                ChangeButton { on_click: move |_| { active_dialog.set(ActiveDialog::Nested(consumable_clone_2.clone())) },
                    "Ingredients"
                }
                ChangeButton {
                    on_click: move |_| {
                        active_dialog
                            .set(
                                consumables::ActiveDialog::Change(consumables::Operation::Update {
                                    consumable: consumable_clone_3.clone(),
                                }),
                            )
                    },
                    "Edit"
                }
                DeleteButton {
                    on_click: move |_| {
                        active_dialog.set(consumables::ActiveDialog::Delete(consumable_clone_4.clone()))
                    },
                    "Delete"
                }
            }
        }
        (Some(Ok(None)), _) => {
            rsx! {
                div { class: "alert alert-error", "Consumption not found." }
            }
        }
        (Some(Err(err)), _) | (_, Some(Err(err))) => {
            rsx! {
                div { class: "alert alert-error",
                    "Error: "
                    {err.to_string()}
                }
            }
        }
        (None, _) | (_, None) => {
            rsx! {
                div { class: "alert alert-info", "Loading..." }
            }
        }
    }
}
