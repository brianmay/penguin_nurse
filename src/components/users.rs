use std::sync::Arc;

use dioxus::prelude::*;

use crate::{
    forms::{
        validate_1st_password, validate_2nd_password, validate_email, validate_full_name,
        validate_username, CancelButton, EditError, InputBoolean, InputPassword, InputString,
        Saving, SubmitButton, ValidationError,
    },
    functions::users::{create_user, delete_user, update_user},
    models::{NewUser, UpdateUser, User},
};

async fn do_save_new_user(
    username: Result<String, ValidationError>,
    email: Result<String, ValidationError>,
    full_name: Result<String, ValidationError>,
    password: Result<String, ValidationError>,
    password_confirm: Result<String, ValidationError>,
    is_admin: Result<bool, ValidationError>,
) -> Result<User, EditError> {
    let username = username?;
    let email = email?;
    let full_name = full_name?;
    let password = password?;
    let _password_confirm = password_confirm?;
    let is_admin = is_admin?;

    let user_updates = NewUser {
        username,
        email,
        full_name,
        password,
        oidc_id: None,
        is_admin,
    };
    create_user(user_updates).await.map_err(EditError::Server)
}

async fn do_update_existing_user(
    user: &User,
    username: Result<String, ValidationError>,
    email: Result<String, ValidationError>,
    full_name: Result<String, ValidationError>,
    is_admin: Result<bool, ValidationError>,
) -> Result<User, EditError> {
    let username = username?;
    let email = email?;
    let full_name = full_name?;
    let is_admin = is_admin?;

    let user_updates = UpdateUser {
        username: Some(username),
        email: Some(email),
        full_name: Some(full_name),
        password: None,
        oidc_id: None,
        is_admin: Some(is_admin),
    };
    update_user(user.id, user_updates)
        .await
        .map_err(EditError::Server)
}

async fn do_change_password(
    user: &User,
    password: Result<String, ValidationError>,
    password_confirm: Result<String, ValidationError>,
) -> Result<User, EditError> {
    let password = password?;
    let _password_confirm = password_confirm?;

    let user_updates = UpdateUser {
        username: None,
        email: None,
        full_name: None,
        password: Some(password),
        oidc_id: None,
        is_admin: None,
    };
    update_user(user.id, user_updates)
        .await
        .map_err(EditError::Server)
}

#[component]
pub fn CreateUser(on_cancel: Callback, on_save: Callback<User>) -> Element {
    let username = use_signal(String::new);
    let email = use_signal(String::new);
    let full_name = use_signal(String::new);
    let password = use_signal(String::new);
    let password_confirm = use_signal(String::new);
    let is_admin = use_signal(|| false);

    let validate_username = use_memo(move || validate_username(&username()));
    let validate_email = use_memo(move || validate_email(&email()));
    let validate_full_name = use_memo(move || validate_full_name(&full_name()));
    let validate_password = use_memo(move || validate_1st_password(&password()));
    let validate_password_confirm =
        use_memo(move || validate_2nd_password(&password(), &password_confirm()));
    let validate_is_admin = use_memo(move || Ok(is_admin()));

    let mut saving = use_signal(|| Saving::No);

    // disable form while waiting for response
    let disabled = use_memo(move || saving.read().is_saving());
    let disabled_save = use_memo(move || {
        validate_username().is_err()
            || validate_email().is_err()
            || validate_full_name().is_err()
            || validate_password().is_err()
            || validate_password_confirm().is_err()
            || disabled()
    });

    let on_save = use_callback(move |()| {
        spawn(async move {
            saving.set(Saving::Yes);

            let result = do_save_new_user(
                validate_username(),
                validate_email(),
                validate_full_name(),
                validate_password(),
                validate_password_confirm(),
                validate_is_admin(),
            )
            .await;

            match result {
                Ok(user) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_save(user);
                }
                Err(err) => saving.set(Saving::Finished(Err(err))),
            }
        });
    });

    rsx! {

        dialog { class: "modal modal-open", id: "my_modal_1",
            div { class: "modal-box",
                h3 { class: "text-lg font-bold", "Create User" }
                p { class: "py-4", "Press ESC key or click the button below to close" }
                match &*saving.read() {
                    Saving::Yes => {
                        rsx! {
                            div { class: "alert alert-info", "Saving..." }
                        }
                    }
                    Saving::Finished(Ok(())) => {
                        rsx! {
                            div { class: "alert alert-success", "Saved!" }
                        }
                    }
                    Saving::Finished(Err(err)) => {
                        rsx! {
                            div { class: "alert alert-error",
                                "Error: "
                                {err.to_string()}
                            }
                        }
                    }
                    _ => {
                        rsx! {}
                    }
                }
                form {
                    novalidate: true,
                    action: "javascript:void(0)",
                    method: "dialog",
                    onkeyup: move |event| {
                        if event.key() == Key::Escape {
                            on_cancel(());
                        }
                    },
                    InputString {
                        id: "username",
                        label: "Username",
                        value: username,
                        validate: validate_username,
                        disabled,
                    }
                    InputString {
                        id: "email",
                        label: "Email",
                        value: email,
                        validate: validate_email,
                        disabled,
                    }
                    InputString {
                        id: "full_name",
                        label: "Full Name",
                        value: full_name,
                        validate: validate_full_name,
                        disabled,
                    }
                    InputPassword {
                        id: "password",
                        label: "Password",
                        value: password,
                        validate: validate_password,
                        disabled,
                    }
                    InputPassword {
                        id: "password_confirm",
                        label: "Confirm Password",
                        value: password_confirm,
                        validate: validate_password_confirm,
                        disabled,
                    }
                    InputBoolean {
                        id: "is_admin",
                        label: "Is Admin",
                        value: is_admin,
                        disabled,
                    }
                    CancelButton { on_cancel: move |_| on_cancel(()), title: "Close" }
                    SubmitButton {
                        disabled: disabled_save,
                        on_save: move |_| on_save(()),
                        title: "Create",
                    }
                }
            }
        }
    }
}

#[component]
pub fn ChangeUser(user: User, on_cancel: Callback, on_save: Callback<User>) -> Element {
    let user = Arc::new(user);

    let username = use_signal(|| user.username.clone());
    let email = use_signal(|| user.email.clone());
    let full_name = use_signal(|| user.full_name.clone());
    let is_admin = use_signal(|| user.is_admin);

    let validate_username = use_memo(move || validate_username(&username()));
    let validate_email = use_memo(move || validate_email(&email()));
    let validate_full_name = use_memo(move || validate_full_name(&full_name()));
    let validate_is_admin = use_memo(move || Ok(is_admin()));

    let mut saving = use_signal(|| Saving::No);

    // disable form while waiting for response
    let disabled = use_memo(move || saving.read().is_saving());
    let disabled_save = use_memo(move || {
        validate_username().is_err()
            || validate_email().is_err()
            || validate_full_name().is_err()
            || disabled()
    });

    let user_clone = user.clone();
    let on_save = use_callback(move |()| {
        let user_clone = user_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            let result = do_update_existing_user(
                &user_clone,
                validate_username(),
                validate_email(),
                validate_full_name(),
                validate_is_admin(),
            )
            .await;

            match result {
                Ok(user) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_save(user);
                }
                Err(err) => saving.set(Saving::Finished(Err(err))),
            }
        });
    });

    rsx! {

        dialog { class: "modal modal-open", id: "my_modal_1",
            div { class: "modal-box",
                h3 { class: "text-lg font-bold",
                    "Edit User: "
                    {&*user.username}
                }
                p { class: "py-4", "Press ESC key or click the button below to close" }
                match &*saving.read() {
                    Saving::Yes => {
                        rsx! {
                            div { class: "alert alert-info", "Saving..." }
                        }
                    }
                    Saving::Finished(Ok(())) => {
                        rsx! {
                            div { class: "alert alert-success", "Saved!" }
                        }
                    }
                    Saving::Finished(Err(err)) => {
                        rsx! {
                            div { class: "alert alert-error",
                                "Error: "
                                {err.to_string()}
                            }
                        }
                    }
                    _ => {
                        rsx! {}
                    }
                }
                form {
                    novalidate: true,
                    action: "javascript:void(0)",
                    method: "dialog",
                    onkeyup: move |event| {
                        if event.key() == Key::Escape {
                            on_cancel(());
                        }
                    },
                    InputString {
                        id: "username",
                        label: "Username",
                        value: username,
                        validate: validate_username,
                        disabled,
                    }
                    InputString {
                        id: "email",
                        label: "Email",
                        value: email,
                        validate: validate_email,
                        disabled,
                    }
                    InputString {
                        id: "full_name",
                        label: "Full Name",
                        value: full_name,
                        validate: validate_full_name,
                        disabled,
                    }
                    InputBoolean {
                        id: "is_admin",
                        label: "Is Admin",
                        value: is_admin,
                        disabled,
                    }
                    CancelButton { on_cancel: move |_| on_cancel(()), title: "Close" }
                    SubmitButton {
                        disabled: disabled_save,
                        on_save: move |_| on_save(()),
                        title: "Save",
                    }
                }
            }
        }
    }
}

#[component]
pub fn ChangePassword(user: User, on_cancel: Callback, on_save: Callback<User>) -> Element {
    let user = Arc::new(user);

    let password = use_signal(String::new);
    let password_confirm = use_signal(String::new);

    let validate_password = use_memo(move || validate_1st_password(&password()));
    let validate_password_confirm =
        use_memo(move || validate_2nd_password(&password(), &password_confirm()));

    let mut saving = use_signal(|| Saving::No);

    // disable form while waiting for response
    let disabled = use_memo(move || saving.read().is_saving());
    let disabled_save = use_memo(move || {
        validate_password().is_err() || validate_password_confirm().is_err() || disabled()
    });

    let user_clone = user.clone();

    let on_save = use_callback(move |()| {
        let user_clone = user_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            let result = do_change_password(
                &user_clone,
                validate_password(),
                validate_password_confirm(),
            )
            .await;
            match result {
                Ok(user) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_save(user);
                }
                Err(err) => saving.set(Saving::Finished(Err(err))),
            }
        });
    });

    rsx! {
        dialog { class: "modal modal-open", id: "my_modal_1",
            div { class: "modal-box",
                h3 { class: "text-lg font-bold",
                    "Change password for "
                    {&*user.username}
                }
                p { class: "py-4", "Press ESC key or click the button below to close" }
                match &*saving.read() {
                    Saving::Yes => {
                        rsx! {
                            div { class: "alert alert-info", "Saving..." }
                        }
                    }
                    Saving::Finished(Ok(())) => {
                        rsx! {
                            div { class: "alert alert-success", "Saved!" }
                        }
                    }
                    Saving::Finished(Err(err)) => {
                        rsx! {
                            div { class: "alert alert-error",
                                "Error: "
                                {err.to_string()}
                            }
                        }
                    }
                    _ => {
                        rsx! {}
                    }
                }
                form {
                    novalidate: true,
                    action: "javascript:void(0)",
                    method: "dialog",
                    onkeyup: move |event| {
                        if event.key() == Key::Escape {
                            on_cancel(());
                        }
                    },
                    InputPassword {
                        id: "password",
                        label: "Password",
                        value: password,
                        validate: validate_password,
                        disabled,
                    }
                    InputPassword {
                        id: "password_confirm",
                        label: "Confirm Password",
                        value: password_confirm,
                        validate: validate_password_confirm,
                        disabled,
                    }
                    CancelButton { on_cancel: move |_| on_cancel(()), title: "Close" }
                    SubmitButton {
                        disabled: disabled_save,
                        on_save: move |_| on_save(()),
                        title: "Save",
                    }
                
                }
            }
        }
    }
}

#[component]
pub fn DeleteUser(user: User, on_cancel: Callback, on_delete: Callback<User>) -> Element {
    let user = Arc::new(user);

    let mut saving = use_signal(|| Saving::No);

    let disabled = use_memo(move || saving.read().is_saving());

    let user_clone = user.clone();
    let on_save = use_callback(move |()| {
        let user_clone = user_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            match delete_user(user_clone.id).await {
                Ok(_) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_delete((*user_clone).clone());
                }
                Err(err) => saving.set(Saving::Finished(Err(EditError::Server(err)))),
            }
        });
    });

    rsx! {
        dialog { class: "modal modal-open", id: "my_modal_1",
            div { class: "modal-box",
                h3 { class: "text-lg font-bold",
                    "Change password for "
                    {&*user.username}
                }
                p { class: "py-4", "Press ESC key or click the button below to close" }
                match &*saving.read() {
                    Saving::Yes => {
                        rsx! {
                            div { class: "alert alert-info", "Deleting..." }
                        }
                    }
                    Saving::Finished(Ok(())) => {
                        rsx! {
                            div { class: "alert alert-success", "Deleted!" }
                        }
                    }
                    Saving::Finished(Err(err)) => {
                        rsx! {
                            div { class: "alert alert-error",
                                "Error: "
                                {err.to_string()}
                            }
                        }
                    }
                    _ => {
                        rsx! {}
                    }
                }
                form {
                    novalidate: true,
                    action: "javascript:void(0)",
                    method: "dialog",
                    onkeyup: move |event| {
                        if event.key() == Key::Escape {
                            on_cancel(());
                        }
                    },
                    CancelButton { on_cancel: move |_| on_cancel(()), title: "Close" }
                    SubmitButton {
                        disabled,
                        on_save: move |_| on_save(()),
                        title: "Delete",
                    }
                }
            }
        }
    }
}
