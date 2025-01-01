use chrono::Local;
use dioxus::prelude::*;

use crate::components::{ChangePassword, ChangeUser, CreateUser, DeleteUser};
use crate::functions::users::{get_user, get_users};
use crate::models::{User, UserId};
use crate::Route;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ActiveDialog {
    Create,
    Change(User),
    ChangePassword(User),
    Delete(User),
    Idle,
}

#[component]
pub fn UserItem(user: ReadOnlySignal<User>, on_click: Callback<User>) -> Element {
    let user = user();
    let user_clone_0 = user.clone();

    rsx! {
        tr {
            class: "hover:bg-gray-500 border-blue-300 m-2 p-2 border-2 h-96 w-48 sm:w-auto sm:border-none sm:h-auto inline-block sm:table-row",
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
pub fn UserDialog(dialog: Signal<ActiveDialog>, reload: Callback<()>) -> Element {
    match dialog() {
        ActiveDialog::Idle => {
            rsx! {}
        }
        ActiveDialog::Create => {
            rsx! {
                CreateUser {
                    on_cancel: move || dialog.set(ActiveDialog::Idle),
                    on_save: move |_user| {
                        reload(());
                        dialog.set(ActiveDialog::Idle);
                    },
                }
            }
        }
        ActiveDialog::Change(user) => {
            rsx! {
                ChangeUser {
                    user,
                    on_cancel: move || dialog.set(ActiveDialog::Idle),
                    on_save: move |_user| {
                        reload(());
                        dialog.set(ActiveDialog::Idle);
                    },
                }
            }
        }
        ActiveDialog::ChangePassword(user) => {
            rsx! {
                ChangePassword {
                    user,
                    on_cancel: move || dialog.set(ActiveDialog::Idle),
                    on_save: move |_user| {
                        reload(());
                        dialog.set(ActiveDialog::Idle);
                    },
                }
            }
        }
        ActiveDialog::Delete(user) => {
            rsx! {
                DeleteUser {
                    user,
                    on_cancel: move || dialog.set(ActiveDialog::Idle),
                    on_delete: move |_user| {
                        reload(());
                        dialog.set(ActiveDialog::Idle);
                    },
                }
            }
        }
    }
}

#[component]
pub fn UserDetail(user_id: UserId) -> Element {
    let mut user = use_resource(move || async move { get_user(user_id).await });
    let mut dialog: Signal<ActiveDialog> = use_signal(|| ActiveDialog::Idle);

    match user() {
        Some(Ok(Some(obj))) => {
            let user_clone_1 = obj.clone();
            let user_clone_2 = obj.clone();
            let user_clone_3 = obj.clone();

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

                div { class: "p-4",
                    button {
                        class: "btn btn-secondary me-2 mb-2",
                        onclick: move |_| dialog.set(ActiveDialog::Change(user_clone_1.clone())),
                        "Change"
                    }

                    button {
                        class: "btn btn-secondary me-2 mb-2",
                        onclick: move |_| dialog.set(ActiveDialog::ChangePassword(user_clone_2.clone())),
                        "Password"
                    }
                    button {
                        class: "btn btn-error me-2 mb-2",
                        onclick: move |_| dialog.set(ActiveDialog::Delete(user_clone_3.clone())),
                        "Delete"
                    }
                }
                div { class: "p-4",
                    button {
                        class: "btn btn-secondary me-2 mb-2",
                        onclick: move |_| {
                            use_navigator().push(Route::UserList {});
                        },
                        "User List"
                    }
                }

                UserDialog { dialog, reload: move || (user.restart()) }
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
pub fn UserList() -> Element {
    let mut users = use_resource(|| async { get_users().await });
    let mut dialog: Signal<ActiveDialog> = use_signal(|| ActiveDialog::Idle);
    let navigator = use_navigator();

    rsx! {
        match users() {
            Some(Ok(users)) => {
                rsx! {
                    if users.is_empty() {
                        p { {"No users found."} }
                    } else {
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
                                        user,
                                        on_click: move |user: User| {
                                            navigator
                                                .push(Route::UserDetail {
                                                    user_id: user.id,
                                                });
                                        },
                                    
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

        div { class: "p-4",
            button {
                class: "btn btn-secondary",
                onclick: move |_| dialog.set(ActiveDialog::Create),
                "Create User"
            }
        }

        UserDialog { dialog, reload: move || (users.restart()) }
    }
}
