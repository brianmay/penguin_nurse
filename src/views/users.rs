use std::ops::Deref;

use chrono::Local;
use dioxus::prelude::*;
use tap::Pipe;

use crate::Route;
use crate::components::buttons::{ChangeButton, DeleteButton, NavButton};
use crate::components::users::{
    ActiveDialog, DetailsDialogReference, ListDialogReference, UserCreate, UserDelete, UserUpdate,
    UserUpdatePassword,
};
use crate::functions::users::{get_user_by_id, get_users};
use crate::models::{User, UserId};

#[component]
pub fn UserItem(user: ReadOnlySignal<User>, on_click: Callback<User>) -> Element {
    let user = user();
    let user_clone_0 = user.clone();

    rsx! {
        tr {
            class: "hover:bg-gray-500 border-blue-300 mt-2 mb-2 p-2 border-2 w-full sm:w-auto sm:border-none inline-block sm:table-row",
            onclick: move |_| {
                on_click(user_clone_0.clone());
            },
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                {user.id.as_inner().to_string()}
            }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2", {user.username} }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2", {user.full_name} }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2 text-xs",
                {user.email}
            }
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                {if user.is_admin { "Admin" } else { "User" }}
            }
        }
    }
}

#[component]
pub fn UserDialog(
    dialog: ReadOnlySignal<ActiveDialog>,
    reload: Callback<()>,
    on_close: Callback<()>,
) -> Element {
    match dialog() {
        ActiveDialog::Idle => {
            rsx! {}
        }
        ActiveDialog::Create => {
            rsx! {
                UserCreate {
                    on_cancel: on_close,
                    on_save: move |user: User| {
                        navigator().replace(Route::UserDetail{ user_id: user.id, dialog: DetailsDialogReference::Update});
                        reload(());
                        on_close(());
                    },
                }
            }
        }
        ActiveDialog::Change(user) => {
            rsx! {
                UserUpdate {
                    user,
                    on_cancel: on_close,
                    on_save: move |_user| {
                        reload(());
                        on_close(());
                    },
                }
            }
        }
        ActiveDialog::Password(user) => {
            rsx! {
                UserUpdatePassword {
                    user,
                    on_cancel: on_close,
                    on_save: move |_user| {
                        reload(());
                        on_close(());
                    },
                }
            }
        }
        ActiveDialog::Delete(user) => {
            rsx! {
                UserDelete {
                    user,
                    on_cancel: on_close,
                    on_delete: move |_user| {
                        reload(());
                        on_close(());
                    },
                }
            }
        }
    }
}

#[component]
pub fn UserDetail(
    user_id: UserId,
    dialog: ReadOnlySignal<Option<DetailsDialogReference>>,
) -> Element {
    let mut maybe_user = use_resource(move || async move { get_user_by_id(user_id).await });

    let active_dialog: Memo<ActiveDialog> = use_memo(move || {
        let Some(dialog) = dialog() else {
            return ActiveDialog::Idle;
        };
        let Some(Ok(Some(user))) = maybe_user() else {
            return ActiveDialog::Idle;
        };
        match dialog {
            DetailsDialogReference::Update => ActiveDialog::Change(user),
            DetailsDialogReference::Password => ActiveDialog::Password(user),
            DetailsDialogReference::Delete => ActiveDialog::Delete(user),
            DetailsDialogReference::Idle => ActiveDialog::Idle,
        }
    });

    let navigator = navigator();
    match maybe_user() {
        Some(Ok(Some(obj))) => {
            rsx! {
                table { class: "table table-striped",
                    tbody {
                        tr {
                            td { "ID" }
                            td { {obj.id.as_inner().to_string()} }
                        }
                        tr {
                            td { "Username" }
                            td { {obj.username} }
                        }
                        tr {
                            td { "Full Name" }
                            td { {obj.full_name} }
                        }
                        tr {
                            td { "Email" }
                            td { {obj.email} }
                        }
                        tr {
                            td { "Role" }
                            td { {if obj.is_admin { "Admin" } else { "User" }} }
                        }
                        tr {
                            td { "Created" }
                            td { {obj.created_at.with_timezone(&Local).to_string()} }
                        }
                        tr {
                            td { "Updated" }
                            td { {obj.updated_at.with_timezone(&Local).to_string()} }
                        }
                    }
                }

                div { class: "flex gap-2 p-4",
                    ChangeButton {
                        on_click: move |_| {
                            navigator
                            .push(Route::UserDetail {
                                user_id,
                                dialog: DetailsDialogReference::Update,
                            });
                        },
                        "Change"
                    }
                    ChangeButton {
                        on_click: move |_| {
                            navigator
                            .push(Route::UserDetail {
                                user_id,
                                dialog: DetailsDialogReference::Password,
                            });
                        },
                        "Password"
                    }
                    DeleteButton {
                        on_click: move |_| {
                        navigator
                            .push(Route::UserDetail {
                                user_id,
                                dialog: DetailsDialogReference::Delete,
                            });
                        },
                        "Delete"
                    }
                }
                div { class: "p-4",
                    NavButton {
                        on_click: move |_| {
                            navigator.push(Route::UserList {
                                dialog: ListDialogReference::Idle
                            });
                        },
                        "User List"
                    }
                }

                UserDialog {
                    dialog: active_dialog,
                    on_close: move |()| {
                        navigator.push(Route::UserDetail{
                            user_id,
                            dialog: DetailsDialogReference::Idle
                        });
                    },
                    reload: move || (maybe_user.restart())
                }
            }
        }
        Some(Ok(None)) => {
            rsx! {
                div { class: "alert alert-error", "User not found." }
            }
        }
        Some(Err(err)) => {
            rsx! {
                div { class: "alert alert-error",
                    "Error: "
                    {err.to_string()}
                }
            }
        }
        None => {
            rsx! {
                div { class: "alert alert-info", "Loading..." }
            }
        }
    }
}

#[component]
pub fn UserList(dialog: ReadOnlySignal<Option<ListDialogReference>>) -> Element {
    let mut users = use_resource(|| async { get_users().await });

    let dialog: Resource<Result<ActiveDialog, ServerFnError>> = use_resource(move || async move {
        let Some(dialog) = dialog() else {
            return Ok(ActiveDialog::Idle);
        };
        match dialog {
            ListDialogReference::Create => ActiveDialog::Create.pipe(Ok),
            // ListDialogReference::Update { user_id } => {
            //     let user = get_user_by_id(user_id).await?.ok_or(
            //         ServerFnError::<NoCustomError>::ServerError("Cannot find user".to_string()),
            //     )?;
            //     ActiveDialog::Change(user).pipe(Ok)
            // }
            // ListDialogReference::Password { user_id } => {
            //     let user = get_user_by_id(user_id).await?.ok_or(
            //         ServerFnError::<NoCustomError>::ServerError("Cannot find user".to_string()),
            //     )?;
            //     ActiveDialog::Password(user).pipe(Ok)
            // }
            // ListDialogReference::Delete { user_id } => {
            //     let user = get_user_by_id(user_id).await?.ok_or(
            //         ServerFnError::<NoCustomError>::ServerError("Cannot find user".to_string()),
            //     )?;
            //     ActiveDialog::Delete(user).pipe(Ok)
            // }
            ListDialogReference::Idle => Ok(ActiveDialog::Idle),
        }
    });

    let navigator = navigator();

    rsx! {
       match users() {
            Some(Ok(users)) => {
                rsx! {
                    if users.is_empty() {
                        p { {"No users found."} }
                    } else {
                        div { class: "ml-2 mr-2 sm:ml-0 sm:mr-0",
                            table { class: "block sm:table",
                                thead { class: "hidden sm:table-header-group",
                                    tr {
                                        th { "ID" }
                                        th { "Username" }
                                        th { "Name" }
                                        th { "Email" }
                                        th { "Role" }
                                    }
                                }
                                tbody { class: "block sm:table-row-group",
                                    for user in users {
                                        UserItem {
                                            key: user.id,
                                            user: user.clone(),
                                            on_click: move |user: User| {
                                                navigator
                                                    .push(Route::UserDetail {
                                                        user_id: user.id,
                                                        dialog: DetailsDialogReference::Idle
                                                    });
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Some(Err(err)) => {
                rsx! {
                    div {
                        "Error: "
                        {err.to_string()}
                    }
                }
            }
            None => {
                rsx! {
                    div { "Loading..." }
                }
            }
        }

        div { class: "ml-2 mr-2",
            button {
                class: "btn btn-secondary",
                onclick: move |_| {
                    navigator.push(Route::UserList { dialog: ListDialogReference::Create });
                },
                "Create User"
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
                UserDialog {
                    dialog: dialog.clone(),
                    reload: move || (users.restart()),
                    on_close: move |()| {
                        navigator
                            .push(Route::UserList {
                                dialog: ListDialogReference::Idle
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
