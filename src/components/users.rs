use std::sync::Arc;

use dioxus::prelude::*;

use crate::{
    forms::{
        validate_1st_password, validate_2nd_password, validate_email, validate_full_name,
        validate_username, CancelButton, Dialog, EditError, FieldValue, InputBoolean,
        InputPassword, InputString, Saving, SubmitButton, ValidationError,
    },
    functions::users::{create_user, delete_user, update_user},
    models::{MaybeString, NewUser, UpdateUser, User},
};

#[derive(Debug, Clone)]
struct ValidateSaveNewUser {
    username: Memo<Result<String, ValidationError>>,
    email: Memo<Result<String, ValidationError>>,
    full_name: Memo<Result<String, ValidationError>>,
    password: Memo<Result<String, ValidationError>>,
    password_confirm: Memo<Result<String, ValidationError>>,
    is_admin: Memo<Result<bool, ValidationError>>,
}

async fn do_save_new_user(validate: &ValidateSaveNewUser) -> Result<User, EditError> {
    let username = validate.username.read().clone()?;
    let email = validate.email.read().clone()?;
    let full_name = validate.full_name.read().clone()?;
    let password = validate.password.read().clone()?;
    let _password_confirm = validate.password_confirm.read().clone()?;
    let is_admin = validate.is_admin.read().clone()?;

    let user_updates = NewUser {
        username,
        email,
        full_name,
        password,
        oidc_id: MaybeString::None,
        is_admin,
    };
    create_user(user_updates).await.map_err(EditError::Server)
}

#[derive(Debug, Clone)]
struct ValidateUpdateExistingUser {
    username: Memo<Result<String, ValidationError>>,
    email: Memo<Result<String, ValidationError>>,
    full_name: Memo<Result<String, ValidationError>>,
    is_admin: Memo<Result<bool, ValidationError>>,
}

async fn do_update_existing_user(
    user: &User,
    validate: &ValidateUpdateExistingUser,
) -> Result<User, EditError> {
    let username = validate.username.read().clone()?;
    let email = validate.email.read().clone()?;
    let full_name = validate.full_name.read().clone()?;
    let is_admin = validate.is_admin.read().clone()?;

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

#[derive(Debug, Clone)]
struct ValidateChangePassword {
    password: Memo<Result<String, ValidationError>>,
    password_confirm: Memo<Result<String, ValidationError>>,
}

async fn do_change_password(
    user: &User,
    validate: &ValidateChangePassword,
) -> Result<User, EditError> {
    let password = validate.password.read().clone()?;
    let _password_confirm = validate.password_confirm.read().clone()?;

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

    let validate = ValidateSaveNewUser {
        username: use_memo(move || validate_username(&username())),
        email: use_memo(move || validate_email(&email())),
        full_name: use_memo(move || validate_full_name(&full_name())),
        password: use_memo(move || validate_1st_password(&password())),
        password_confirm: use_memo(move || validate_2nd_password(&password(), &password_confirm())),
        is_admin: use_memo(move || Ok(is_admin())),
    };

    let mut saving = use_signal(|| Saving::No);

    // disable form while waiting for response
    let disabled = use_memo(move || saving.read().is_saving());
    let disabled_save = use_memo(move || {
        validate.username.read().is_err()
            || validate.email.read().is_err()
            || validate.full_name.read().is_err()
            || validate.password.read().is_err()
            || validate.password_confirm.read().is_err()
            || validate.is_admin.read().is_err()
            || disabled()
    });

    let validate_clone = validate.clone();
    let on_save = use_callback(move |()| {
        let validate = validate_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            let result = do_save_new_user(&validate).await;
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

        Dialog {
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
                    validate: validate.username,
                    disabled,
                }
                InputString {
                    id: "email",
                    label: "Email",
                    value: email,
                    validate: validate.email,
                    disabled,
                }
                InputString {
                    id: "full_name",
                    label: "Full Name",
                    value: full_name,
                    validate: validate.full_name,
                    disabled,
                }
                InputPassword {
                    id: "password",
                    label: "Password",
                    value: password,
                    validate: validate.password,
                    disabled,
                }
                InputPassword {
                    id: "password_confirm",
                    label: "Confirm Password",
                    value: password_confirm,
                    validate: validate.password_confirm,
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

#[component]
pub fn ChangeUser(user: User, on_cancel: Callback, on_save: Callback<User>) -> Element {
    let user = Arc::new(user);

    let username = use_signal(|| user.username.as_string());
    let email = use_signal(|| user.email.as_string());
    let full_name = use_signal(|| user.full_name.as_string());
    let is_admin = use_signal(|| user.is_admin);

    let validate = ValidateUpdateExistingUser {
        username: use_memo(move || validate_username(&username())),
        email: use_memo(move || validate_email(&email())),
        full_name: use_memo(move || validate_full_name(&full_name())),
        is_admin: use_memo(move || Ok(is_admin())),
    };

    let mut saving = use_signal(|| Saving::No);

    // disable form while waiting for response
    let disabled = use_memo(move || saving.read().is_saving());
    let disabled_save = use_memo(move || {
        validate.username.read().is_err()
            || validate.email.read().is_err()
            || validate.full_name.read().is_err()
            || validate.is_admin.read().is_err()
            || disabled()
    });

    let user_clone = user.clone();
    let validate_clone = validate.clone();
    let on_save = use_callback(move |()| {
        let user = user_clone.clone();
        let validate = validate_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            let result = do_update_existing_user(&user, &validate).await;

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
        Dialog {
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
                    validate: validate.username,
                    disabled,
                }
                InputString {
                    id: "email",
                    label: "Email",
                    value: email,
                    validate: validate.email,
                    disabled,
                }
                InputString {
                    id: "full_name",
                    label: "Full Name",
                    value: full_name,
                    validate: validate.full_name,
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

#[component]
pub fn ChangePassword(user: User, on_cancel: Callback, on_save: Callback<User>) -> Element {
    let user = Arc::new(user);

    let password = use_signal(String::new);
    let password_confirm = use_signal(String::new);

    let validate = ValidateChangePassword {
        password: use_memo(move || validate_1st_password(&password())),
        password_confirm: use_memo(move || validate_2nd_password(&password(), &password_confirm())),
    };

    let mut saving = use_signal(|| Saving::No);

    // disable form while waiting for response
    let disabled = use_memo(move || saving.read().is_saving());
    let disabled_save = use_memo(move || {
        validate.password.read().is_err() || validate.password_confirm.read().is_err() || disabled()
    });

    let user_clone = user.clone();
    let validate_clone = validate.clone();
    let on_save = use_callback(move |()| {
        let user = user_clone.clone();
        let validate = validate_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            let result = do_change_password(&user, &validate).await;
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
        Dialog {
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
                    validate: validate.password,
                    disabled,
                }
                InputPassword {
                    id: "password_confirm",
                    label: "Confirm Password",
                    value: password_confirm,
                    validate: validate.password_confirm,
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
        Dialog {
            h3 { class: "text-lg font-bold",
                "Delete user "
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
