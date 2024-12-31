use dioxus::prelude::*;

use crate::components::{ChangePassword, ChangeUser, CreateUser, DeleteUser};
use crate::functions::users::get_users;
use crate::models::User;

#[derive(Debug, Clone)]
pub enum ActiveDialog {
    Create,
    Change(User),
    ChangePassword(User),
    Delete(User),
    Idle,
}

#[component]
pub fn UserItem(
    user: User,
    on_delete: Callback<User>,
    on_change: Callback<User>,
    on_change_password: Callback<User>,
) -> Element {
    let user_clone_1 = user.clone();
    let user_clone_2 = user.clone();
    let user_clone_3 = user.clone();

    rsx! {
        tr { class: "hover:bg-gray-500 border-blue-300 m-2 p-2 border-2 h-96 w-48 sm:w-auto sm:border-none sm:h-auto inline-block sm:table-row",
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
            td { class: "block sm:table-cell border-blue-300 sm:border-t-2",
                button {
                    class: "btn btn-secondary mx-1 my-1",
                    onclick: move |_| on_change(user_clone_3.clone()),
                    "Edit"
                }
                button {
                    class: "btn btn-secondary mx-1 my-1",
                    onclick: move |_| on_change_password(user_clone_2.clone()),
                    "Password"
                }
                button {
                    class: "btn btn-warning mx-1 my-1",
                    onclick: move |_| on_delete(user_clone_1.clone()),
                    "Delete"
                }
            }
        }
    }
}

#[component]
pub fn UserList() -> Element {
    let mut users = use_resource(|| async { get_users().await });
    let mut dialog: Signal<ActiveDialog> = use_signal(|| ActiveDialog::Idle);

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
                                    th { "Actions" }
                                }
                            }
                            tbody { class: "block sm:table-row-group",
                                for user in users {
                                    UserItem {
                                        key: user.id,
                                        user,
                                        on_delete: move |user| dialog.set(ActiveDialog::Delete(user)),
                                        on_change: move |user| dialog.set(ActiveDialog::Change(user)),
                                        on_change_password: move |user| dialog.set(ActiveDialog::ChangePassword(user)),
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

        button {
            class: "btn btn-secondary",
            onclick: move |_| dialog.set(ActiveDialog::Create),
            "Create User"
        }

        match dialog() {
            ActiveDialog::Idle => {
                rsx! {}
            }
            ActiveDialog::Create => {
                rsx! {
                    CreateUser {
                        on_cancel: move || dialog.set(ActiveDialog::Idle),
                        on_save: move |_user| {
                            users.restart();
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
                            users.restart();
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
                            users.restart();
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
                            users.restart();
                            dialog.set(ActiveDialog::Idle);
                        },
                    }
                }
            }
        }
    }
}
