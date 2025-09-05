use std::ops::Deref;

use chrono::Local;
use dioxus::prelude::*;
use tap::Pipe;

use crate::{
    Route,
    components::{
        buttons::{ChangeButton, CreateButton},
        consumables::{
            ActiveDialog, ConsumableDialog, ConsumableItemList, ListDialogReference, Operation,
            organic_icon,
        },
        events::Markdown,
    },
    forms::Barcode,
    functions::consumables::{get_consumable_by_id, search_consumables_with_nested},
    models::{Consumable, ConsumableId, ConsumableWithItems},
    use_user,
};

#[component]
fn EntryRow(
    consumable_with_items: ConsumableWithItems,
    selected: Signal<Option<ConsumableId>>,
) -> Element {
    let consumable = consumable_with_items.consumable;
    let items = consumable_with_items.items;

    let id = consumable.id;

    let navigator = navigator();
    rsx! {
        tr {
            class: "hover:bg-gray-500 border-blue-300 mt-2 mb-2 p-2 border-2 w-full sm:w-auto sm:border-none inline-block sm:table-row",
            onclick: move |_| { selected.set(Some(id)) },
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                if consumable.is_organic {
                    organic_icon {}
                }
                {consumable.name}
            }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                if let Some(brand) = &consumable.brand {
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
                if let Some(comments) = &consumable.comments {
                    Markdown { content: comments.to_string() }
                }
            }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                if let Some(created) = &consumable.created {
                    span { class: "sm:hidden", "Created: " }
                    {created.with_timezone(&Local).to_string()}
                }
            }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                if let Some(destroyed) = &consumable.destroyed {
                    span { class: "sm:hidden", "Destroyed: " }
                    {destroyed.with_timezone(&Local).to_string()}
                }
            }
        }

        if selected() == Some(id) {
            tr {
                td { colspan: "6", class: "block sm:table-cell",
                    div { class: "flex gap-2",
                        ChangeButton {
                            on_click: move |_| {
                                navigator
                                    .push(Route::ConsumableList {
                                        dialog: ListDialogReference::UpdateIngredients {
                                            consumable_id: id,
                                        },
                                    });
                            },
                            "Ingredients"
                        }
                        ChangeButton {
                            on_click: move |_| {
                                navigator
                                    .push(Route::ConsumableList {
                                        dialog: ListDialogReference::UpdateBasic {
                                            consumable_id: id,
                                        },
                                    });
                            },
                            "Edit"
                        }
                        ChangeButton {
                            on_click: move |_| {
                                navigator
                                    .push(Route::ConsumableList {
                                        dialog: ListDialogReference::Delete {
                                            consumable_id: id,
                                        },
                                    });
                            },
                            "Delete"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ConsumableList(dialog: ReadOnlySignal<Option<ListDialogReference>>) -> Element {
    let user = use_user().ok().flatten();

    let Some(_user) = user.as_ref() else {
        return rsx! {
            p { class: "alert alert-error", "You are not logged in." }
        };
    };

    let selected: Signal<Option<ConsumableId>> = use_signal(|| None);
    let mut show_only_created = use_signal(|| false);
    let mut show_destroyed = use_signal(|| false);

    let mut query = use_signal(|| "".to_string());

    let dialog: Resource<Result<ActiveDialog, ServerFnError>> = use_resource(move || async move {
        let Some(dialog) = dialog() else {
            return Ok(ActiveDialog::Idle);
        };
        match dialog {
            ListDialogReference::Create => ActiveDialog::Change(Operation::Create).pipe(Ok),
            ListDialogReference::UpdateBasic { consumable_id } => {
                let consumable = get_consumable_by_id(consumable_id).await?.ok_or(
                    ServerFnError::<String>::ServerError("Cannot find consumable".to_string()),
                )?;
                ActiveDialog::Change(Operation::Update { consumable }).pipe(Ok)
            }
            ListDialogReference::UpdateIngredients { consumable_id } => {
                let consumable = get_consumable_by_id(consumable_id).await?.ok_or(
                    ServerFnError::<String>::ServerError("Cannot find consumable".to_string()),
                )?;
                ActiveDialog::Ingredients(consumable).pipe(Ok)
            }
            ListDialogReference::IngredientUpdateBasic {
                parent_id,
                consumable_id,
            } => {
                let parent = get_consumable_by_id(parent_id).await?.ok_or(
                    ServerFnError::<String>::ServerError("Cannot find consumable".to_string()),
                )?;
                let consumable = get_consumable_by_id(consumable_id).await?.ok_or(
                    ServerFnError::<String>::ServerError("Cannot find consumable".to_string()),
                )?;
                ActiveDialog::NestedIngredient(parent, consumable).pipe(Ok)
            }
            ListDialogReference::IngredientUpdateIngredients {
                parent_id,
                consumable_id,
            } => {
                let parent = get_consumable_by_id(parent_id).await?.ok_or(
                    ServerFnError::<String>::ServerError("Cannot find consumable".to_string()),
                )?;
                let consumable = get_consumable_by_id(consumable_id).await?.ok_or(
                    ServerFnError::<String>::ServerError("Cannot find consumable".to_string()),
                )?;
                ActiveDialog::NestedIngredients(parent, consumable).pipe(Ok)
            }
            ListDialogReference::Delete { consumable_id } => {
                let consumable = get_consumable_by_id(consumable_id).await?.ok_or(
                    ServerFnError::<String>::ServerError("Cannot find consumable".to_string()),
                )?;
                ActiveDialog::Delete(consumable).pipe(Ok)
            }
            ListDialogReference::Idle => Ok(ActiveDialog::Idle),
        }
    });

    let navigator = navigator();
    let mut list: Resource<Result<Vec<ConsumableWithItems>, ServerFnError>> =
        use_resource(move || async move {
            search_consumables_with_nested(query(), show_only_created(), show_destroyed()).await
        });

    rsx! {
        div { class: "ml-2 mr-2",
            div { class: "mb-2",
                CreateButton {
                    on_click: move |_| {
                        navigator
                            .push(Route::ConsumableList {
                                dialog: ListDialogReference::Create,
                            });
                    },
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
                Barcode { barcode: query }
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
                div { class: "alert alert-error",
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
                                    // Borken, See https://github.com/dioxuslabs/dioxus/issues/4066
                                    // key: "{consumable.consumable.id.as_inner().to_string()}",
                                    consumable_with_items: consumable.clone(),
                                    selected,
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

        match dialog.read().deref() {
            Some(Err(err)) => rsx! {
                div { class: "alert alert-error",
                    "Error loading dialog: "
                    {err.to_string()}
                }
            },
            Some(Ok(dialog)) => rsx! {
                ConsumableDialog {
                    dialog: dialog.clone(),
                    on_change: move |_consumable: Consumable| { list.restart() },
                    on_change_ingredients: move |_consumable: Consumable| { list.restart() },
                    on_delete: move |_consumable| list.restart(),
                    show_update_basic: move |consumable: Consumable| {
                        navigator
                            .push(Route::ConsumableList {
                                dialog: ListDialogReference::UpdateBasic {
                                    consumable_id: consumable.id,
                                },
                            });
                    },
                    show_update_ingredients: move |consumable: Consumable| {
                        navigator
                            .push(Route::ConsumableList {
                                dialog: ListDialogReference::UpdateIngredients {
                                    consumable_id: consumable.id,
                                },
                            });
                    },
                    show_ingredient_update_basic: move |(parent, consumable): (Consumable, Consumable)| {
                        navigator
                            .push(Route::ConsumableList {
                                dialog: ListDialogReference::IngredientUpdateBasic {
                                    parent_id: parent.id,
                                    consumable_id: consumable.id,
                                },
                            });
                    },
                    show_ingredient_update_ingredients: move |(parent, consumable): (Consumable, Consumable)| {
                        navigator
                            .push(Route::ConsumableList {
                                dialog: ListDialogReference::IngredientUpdateIngredients {
                                    parent_id: parent.id,
                                    consumable_id: consumable.id,
                                },
                            });
                    },
                    on_close: move |()| {
                        navigator
                            .push(Route::ConsumableList {
                                dialog: ListDialogReference::Idle,
                            });
                    },
                }
            },
            None => {
                rsx! {
                    p { class: "alert alert-info", "Loading..." }
                }
            }
        }
    }
}
