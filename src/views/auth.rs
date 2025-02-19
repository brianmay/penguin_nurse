use crate::{
    forms::{
        validate_password, validate_username, FormCancelButton, InputPassword, InputString, MyForm,
        FormSubmitButton,
    },
    models::User,
    reload_user, use_user, Route,
};
use dioxus::prelude::*;
use tap::Pipe;
use tracing::error;

const NURSE_SVG: Asset = asset!("/assets/nurse.svg");

#[component]
pub fn LoginWindow(children: Element) -> Element {
    rsx! {
        section { class: "bg-gray-50 dark:bg-gray-900",
            div { class: "flex flex-col items-center justify-center px-6 py-8 mx-auto md:h-screen lg:py-0",
                a {
                    href: "#",
                    class: "flex items-center mb-6 text-2xl font-semibold text-gray-900 dark:text-white",
                    img { alt: "Nurse Logo", src: NURSE_SVG, class: "h-8" }
                    span { class: "self-center text-2xl font-semibold whitespace-nowrap dark:text-white",
                        "Penguin Nurse"
                    }
                }
                div { class: "w-full bg-white rounded-lg shadow dark:border md:mt-0 sm:max-w-md xl:p-0 dark:bg-gray-800 dark:border-gray-700",
                    div { class: "p-6 space-y-4 md:space-y-6 sm:p-8", {children} }
                }
            }
        }
    }
}

#[component]
pub fn Login() -> Element {
    let is_oidc_enabled = use_resource(is_oidc_enabled);

    let username = use_signal(String::new);
    let password = use_signal(String::new);
    let validate_username = use_memo(move || validate_username(&username()));
    let validate_password = use_memo(move || validate_password(&password()));

    let mut result: Signal<Option<Result<(), ServerFnError>>> = use_signal(|| None);
    let user_load_error = use_user();

    // disable form while waiting for response
    let disabled = use_memo(move || result().is_some());
    let disabled_save = use_memo(move || {
        validate_username().is_err() || validate_password().is_err() || disabled()
    });

    let on_save = use_callback(move |()| async move {
        let maybe_new_user = login_with_password(username(), password()).await;
        match maybe_new_user {
            Ok(_new_user) => {
                reload_user();
                result.set(None);
                let navigator = navigator();
                navigator.push(Route::Home {});
            }
            Err(err) => {
                result.set(Some(Err(err)));
            }
        }
    });

    rsx! {
        LoginWindow {
            if let Err(err) = &user_load_error {
                div { class: "bg-red-500 text-white p-2 text-center", {err.to_string()} }
            }
            if let Ok(Some(user_obj)) = user_load_error {
                div {
                    h1 { "Welcome back, " }
                    h2 { {&*user_obj.username} }
                    form { novalidate: true, action: "javascript:void(0);",
                        FormSubmitButton {
                            disabled: Memo::new(|| false),
                            on_save: move |_| {
                                let navigator = navigator();
                                navigator.push(Route::Home {});
                            },
                            title: "Home",
                        }
                    }
                }
            } else {
                match result() {
                    Some(Ok(())) => {
                        rsx! {
                            div {
                                h1 { "Login succeeded but you are not logged in" }
                            }
                        }
                    }
                    Some(Err(err)) => {
                        rsx! {
                            div {
                                h1 { "Login failed!" }
                                h2 { {err.to_string()} }
                                form { novalidate: true, action: "javascript:void(0);",
                                    FormSubmitButton {
                                        disabled: Memo::new(|| false),
                                        on_save: move |_| {
                                            reload_user();
                                            result.set(None);
                                        },
                                        title: "Retry",
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        rsx! {
                            div {
                                h1 { class: "text-xl font-bold leading-tight tracking-tight text-gray-900 md:text-2xl dark:text-white",
                                    "Sign in to your account"
                                }
                                MyForm {
                                    InputString {
                                        id: "username",
                                        label: "Username",
                                        value: username,
                                        validate: validate_username,
                                        disabled,
                                    }
                                    InputPassword {
                                        id: "password",
                                        label: "Password",
                                        value: password,
                                        validate: validate_password,
                                        disabled,
                                    }
                                    div { class: "flex items-center justify-between",
                                        div { class: "flex items-start",
                                            div { class: "flex items-center h-5",
                                                input {
                                                    id: "remember",
                                                    r#type: "checkbox",
                                                    required: "",
                                                    "aria-describedby": "remember",
                                                    class: "w-4 h-4 border border-gray-300 rounded bg-gray-50 focus:ring-3 focus:ring-primary-300 dark:bg-gray-700 dark:border-gray-600 dark:focus:ring-primary-600 dark:ring-offset-gray-800",
                                                }
                                            }
                                            div { class: "ml-3 text-sm",
                                                label {
                                                    r#for: "remember",
                                                    class: "text-gray-500 dark:text-gray-300",
                                                    "Remember me"
                                                }
                                            }
                                        }
                                        a {
                                            href: "#",
                                            class: "text-sm font-medium text-primary-600 hover:underline dark:text-primary-500",
                                            "Forgot password?"
                                        }
                                    }
                                    FormSubmitButton {
                                        disabled: disabled_save,
                                        title: "Sign in",
                                        on_save: move |_e| async move { on_save(()).await },
                                    }
                                    if is_oidc_enabled().unwrap_or(Ok(false)).unwrap_or(false) {
                                        div { class: "flex items-center justify-center",
                                            button {
                                                r#type: "button",
                                                class: "w-full btn btn-primary my-2",
                                                onclick: move |_| async move {
                                                    let url = login_with_oidc().await;
                                                    match url {
                                                        Ok(url) => {
                                                            result.set(None);
                                                            let navigator = navigator();
                                                            navigator.push(NavigationTarget::<Route>::External(url));
                                                        }
                                                        Err(err) => result.set(Some(Err(err))),
                                                    }
                                                },
                                                "Sign in with OIDC"
                                            }
                                        }
                                    }
                                    p { class: "text-sm font-light text-gray-500 dark:text-gray-400",
                                        "Don’t have an account yet?"
                                        a {
                                            href: "#",
                                            class: "font-medium text-primary-600 hover:underline dark:text-primary-500",
                                            "Sign up"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Logout() -> Element {
    let mut result: Signal<Option<Result<(), ServerFnError>>> = use_signal(|| None);
    let user_load_error = use_user();

    let on_save = use_callback(move |()| async move {
        let results = do_logout().await;
        if results.is_ok() {
            let navigator = navigator();
            navigator.push(Route::Home {});
        }
        result.set(Some(results));
        reload_user();
    });

    rsx! {
        LoginWindow {
            if let Err(err) = &user_load_error {
                div { class: "bg-red-500 text-white p-2 text-center", {err.to_string()} }
            }
            if let Ok(Some(_user_object)) = user_load_error {
                match result() {
                    Some(Ok(())) => {
                        rsx! {
                            div {
                                h1 { "Logout success!" }
                                form { novalidate: true, action: "javascript:void(0);",
                                    FormSubmitButton {
                                        disabled: Memo::new(|| false),
                                        title: "Home",
                                        on_save: move |_| {
                                            let navigator = navigator();
                                            navigator.push(Route::Home {});
                                        },
                                    }
                                }
                            }
                        }
                    }
                    Some(Err(err)) => {
                        rsx! {
                            div {
                                h1 { "Logout failed!" }
                                h2 { {err.to_string()} }
                            }
                        }
                    }
                    None => {
                        rsx! {
                            div {
                                h1 { "Are you sure you want to logout?" }
                                form { novalidate: true, action: "javascript:void(0);",
                                    FormCancelButton {
                                        on_cancel: move |_| {
                                            let navigator = navigator();
                                            navigator.push(Route::Home {});
                                        },
                                        title: "Cancel",
                                    }
                                    FormSubmitButton {
                                        disabled: Memo::new(|| false),
                                        title: "Logout",
                                        on_save: move |_e| async move { on_save(()).await },
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                div {
                    h1 { "You are not logged in!" }
                    form { novalidate: true, action: "javascript:void(0);",
                        FormCancelButton {
                            on_cancel: move |_| {
                                let navigator = navigator();
                                navigator.push(Route::Home {});
                            },
                            title: "Home",
                        }
                    }
                }
            }
        }
    }
}

#[server]
async fn login_with_password(username: String, password: String) -> Result<User, ServerFnError> {
    use crate::server::auth::{Credentials, Session};

    let mut session: Session = extract().await?;

    let creds = Credentials {
        username,
        password,
        // next: None,
    };

    let user = match session.authenticate(creds).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            error!("Invalid credentials");
            return Err(ServerFnError::ServerError(
                "Invalid credentials".to_string(),
            ));
        }
        Err(err) => {
            error!("Error authenticating user: {:?}", err);
            return Err(ServerFnError::ServerError(
                "Invalid server error".to_string(),
            ));
        }
    };

    if let Err(err) = session.login(&user).await {
        error!("Error logging in user: {:?}", err);
        return Err(ServerFnError::ServerError(
            "Invalid server error".to_string(),
        ));
    }

    Ok(user.into())
}

#[server]
async fn do_logout() -> Result<(), ServerFnError> {
    use crate::server::auth::Session;

    let mut session: Session = extract().await?;
    session.logout().await?;
    Ok(())
}

#[server]
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    use crate::server::auth::Session;

    let session: Session = extract().await?;
    session.user.clone().map(|x| x.into()).pipe(Ok)
}

#[server]
pub async fn is_oidc_enabled() -> Result<bool, ServerFnError> {
    use crate::server::OidcClientState;
    use axum::Extension;

    let Extension(client): Extension<OidcClientState> = extract().await?;
    Ok(client.load().is_some())
}

#[server]
pub async fn login_with_oidc() -> Result<String, ServerFnError> {
    use crate::server::OidcClientState;
    use axum::Extension;

    let Extension(oidc_client): Extension<OidcClientState> = extract().await?;

    let oidc_client = oidc_client.load();
    let Some(oidc_client) = oidc_client.as_ref() else {
        return Err(ServerFnError::ServerError(
            "OIDC not initialized".to_string(),
        ));
    };
    let auth_url = oidc_client.get_auth_url("/");
    Ok(auth_url)
}
