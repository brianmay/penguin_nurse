use std::{num::ParseIntError, str::FromStr, sync::Arc};

use dioxus::{prelude::*, router::ToQueryArgument};
use tap::Pipe;
use thiserror::Error;

use crate::{
    forms::{
        Dialog, EditError, FieldValue, FormSaveCancelButton, InputBoolean, InputPassword,
        InputString, Saving, ValidationError, validate_1st_password, validate_2nd_password,
        validate_email, validate_full_name, validate_username,
    },
    functions::users::{create_user, delete_user, update_user},
    models::{ChangeUser, MaybeSet, MaybeString, NewUser, User},
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

    let changes = ChangeUser {
        username: MaybeSet::Set(username),
        email: MaybeSet::Set(email),
        full_name: MaybeSet::Set(full_name),
        oidc_id: MaybeSet::NoChange,
        is_admin: MaybeSet::Set(is_admin),
    };
    update_user(user.id, changes, None)
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

    let changes = ChangeUser {
        username: MaybeSet::NoChange,
        email: MaybeSet::NoChange,
        full_name: MaybeSet::NoChange,
        oidc_id: MaybeSet::NoChange,
        is_admin: MaybeSet::NoChange,
    };
    update_user(user.id, changes, Some(password))
        .await
        .map_err(EditError::Server)
}

#[component]
pub fn UserCreate(on_cancel: Callback, on_save: Callback<User>) -> Element {
    let username = use_signal(String::new);
    let email = use_signal(String::new);
    let full_name = use_signal(String::new);
    let password = use_signal(String::new);
    let password_confirm = use_signal(String::new);
    let is_admin = use_signal(|| false);

    let validate = {
        let password_validate = use_memo(move || validate_1st_password(&password()));
        ValidateSaveNewUser {
            username: use_memo(move || validate_username(&username())),
            email: use_memo(move || validate_email(&email())),
            full_name: use_memo(move || validate_full_name(&full_name())),
            password: password_validate,
            password_confirm: use_memo(move || {
                validate_2nd_password(&password_validate.read(), &password_confirm())
            }),
            is_admin: use_memo(move || Ok(is_admin())),
        }
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
                FormSaveCancelButton {
                    disabled: disabled_save,
                    on_save: move |()| on_save(()),
                    on_cancel: move |_| on_cancel(()),
                    title: "Create",
                    saving,
                }
            }
        }
    }
}

#[component]
pub fn UserUpdate(user: User, on_cancel: Callback, on_save: Callback<User>) -> Element {
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
                FormSaveCancelButton {
                    disabled: disabled_save,
                    on_save: move |()| on_save(()),
                    on_cancel: move |_| on_cancel(()),
                    title: "Save",
                    saving,
                }
            }
        }
    }
}

#[component]
pub fn UserUpdatePassword(user: User, on_cancel: Callback, on_save: Callback<User>) -> Element {
    let user = Arc::new(user);

    let password = use_signal(String::new);
    let password_confirm = use_signal(String::new);

    let validate = {
        let password_validate = use_memo(move || validate_1st_password(&password()));
        ValidateChangePassword {
            password: password_validate,
            password_confirm: use_memo(move || {
                validate_2nd_password(&password_validate.read(), &password_confirm())
            }),
        }
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
                FormSaveCancelButton {
                    disabled: disabled_save,
                    on_save: move |()| on_save(()),
                    on_cancel: move |_| on_cancel(()),
                    title: "Save",
                    saving,
                }
            }
        }
    }
}

#[component]
pub fn UserDelete(user: User, on_cancel: Callback, on_delete: Callback<User>) -> Element {
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
            form {
                novalidate: true,
                action: "javascript:void(0)",
                method: "dialog",
                onkeyup: move |event| {
                    if event.key() == Key::Escape {
                        on_cancel(());
                    }
                },
                FormSaveCancelButton {
                    disabled,
                    on_save: move |()| on_save(()),
                    on_cancel: move |_| on_cancel(()),
                    title: "Delete",
                    saving,
                }
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ActiveDialog {
    Create,
    Change(User),
    Password(User),
    Delete(User),
    Idle,
}

#[derive(Error, Debug)]
pub enum ListDialogReferenceError {
    #[error("Invalid integer")]
    ParseIntError(#[from] ParseIntError),

    #[error("Invalid reference")]
    ReferenceError,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ListDialogReference {
    Create,
    // Update {
    //     user_id: UserId,
    // },
    // Password {
    //     user_id: UserId,
    // },
    // Delete {
    //     user_id: UserId,
    // },
    #[default]
    Idle,
}

impl ToQueryArgument for ListDialogReference {
    fn display_query_argument(
        &self,
        query_name: &str,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}={}", query_name, self.to_string())
    }
}

impl FromStr for ListDialogReference {
    type Err = ListDialogReferenceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split("-").collect::<Vec<_>>();
        match split[..] {
            ["create"] => Self::Create,
            // ["update", id] => {
            //     let user_id = UserId::new(id.parse()?);
            //     Self::Update { user_id }
            // }
            // ["password", id] => {
            //     let user_id = UserId::new(id.parse()?);
            //     Self::Password { user_id }
            // }
            // ["delete", id] => {
            //     let user_id = UserId::new(id.parse()?);
            //     Self::Delete { user_id }
            // }
            [""] | [] => Self::Idle,
            _ => return Err(ListDialogReferenceError::ReferenceError),
        }
        .pipe(Ok)
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for ListDialogReference {
    fn to_string(&self) -> String {
        match self {
            ListDialogReference::Create => "create".to_string(),
            // ListDialogReference::Update { user_id } => format!("update-{user_id}"),
            // ListDialogReference::Password { user_id } => {
            //     format!("password-{user_id}")
            // }
            // ListDialogReference::Delete { user_id } => format!("delete-{user_id}"),
            ListDialogReference::Idle => String::new(),
        }
    }
}

#[derive(Error, Debug)]
pub enum DetailsDialogReferenceError {
    #[error("Invalid reference")]
    ReferenceError,
}

impl ToQueryArgument for DetailsDialogReference {
    fn display_query_argument(
        &self,
        query_name: &str,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}={}", query_name, self.to_string())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum DetailsDialogReference {
    Update,
    Password,
    Delete,
    #[default]
    Idle,
}

impl FromStr for DetailsDialogReference {
    type Err = DetailsDialogReferenceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split("-").collect::<Vec<_>>();
        match split[..] {
            ["update"] => Self::Update,
            ["password"] => Self::Password,
            ["delete"] => Self::Delete,
            [""] | [] => Self::Idle,
            _ => return Err(DetailsDialogReferenceError::ReferenceError),
        }
        .pipe(Ok)
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for DetailsDialogReference {
    fn to_string(&self) -> String {
        match self {
            DetailsDialogReference::Update => "update".to_string(),
            DetailsDialogReference::Password => "password".to_string(),
            DetailsDialogReference::Delete => "delete".to_string(),
            DetailsDialogReference::Idle => String::new(),
        }
    }
}
