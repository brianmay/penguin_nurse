use std::sync::Arc;

use crate::{models::User, Route, UserLoadError};
use dioxus::prelude::*;
use tap::Pipe;
use tracing::error;

const NURSE_SVG: Asset = asset!("/assets/nurse.svg");

#[component]
pub fn Login() -> Element {
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut result: Signal<Option<Result<(), ServerFnError>>> = use_signal(|| None);
    let mut user: Signal<Arc<Option<User>>> = use_context();
    let user_load_error: UserLoadError = use_context();

    let on_save = use_callback(move |()| async move {
        let maybe_new_user = login_with_password(username(), password()).await;
        match maybe_new_user {
            Ok(new_user) => {
                user.set(Arc::new(Some(new_user)));
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
                    if let Err(err) = user_load_error.0() {
                        div { class: "bg-red-500 text-white p-2 text-center", {err.to_string()} }
                    }
                    div { class: "p-6 space-y-4 md:space-y-6 sm:p-8",
                        if let Some(user_obj) = &*user() {
                            div {
                                h1 { "Welcome back, " }
                                h2 { {&*user_obj.username} }
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
                                                button {
                                                    r#type: "submit",
                                                    class: "w-full text-white bg-green-600 hover:bg-green-700 focus:ring-4 focus:outline-none focus:ring-primary-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-primary-600 dark:hover:bg-primary-700 dark:focus:ring-primary-800",
                                                    autofocus: true,
                                                    onclick: move |_| {
                                                        user.set(Arc::new(None));
                                                        result.set(None);
                                                    },
                                                    "Try again"
                                                }
                                            }
                                        }
                                    }
                                }
                                None => {
                                    rsx! {
                                        h1 { class: "text-xl font-bold leading-tight tracking-tight text-gray-900 md:text-2xl dark:text-white",
                                            "Sign in to your account"
                                        }
                                        form {
                                            novalidate: true,
                                            action: "javascript:void(0);",
                                            class: "space-y-4 md:space-y-6",
                                            div {
                                                label {
                                                    r#for: "username",
                                                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                                                    "Your username"
                                                }
                                                input {
                                                    id: "username",
                                                    name: "username",
                                                    r#type: "username",
                                                    placeholder: "name",
                                                    required: "",
                                                    class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                                    autofocus: true,
                                                    value: username(),
                                                    oninput: move |e| {
                                                        username.set(e.value());
                                                    },
                                                }
                                            }
                                            div {
                                                label {
                                                    r#for: "password",
                                                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                                                    "Password"
                                                }
                                                input {
                                                    id: "password",
                                                    required: "",
                                                    r#type: "password",
                                                    name: "password",
                                                    placeholder: "••••••••",
                                                    class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                                    value: password(),
                                                    oninput: move |e| password.set(e.value()),
                                                }
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
                                            button {
                                                r#type: "submit",
                                                class: "w-full text-white bg-green-600 hover:bg-green-700 focus:ring-4 focus:outline-none focus:ring-primary-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-primary-600 dark:hover:bg-primary-700 dark:focus:ring-primary-800",
                                                onclick: move |_e| async move { on_save(()).await },
                                                "Sign in"
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
    }
}

#[component]
pub fn Logout() -> Element {
    let mut result: Signal<Option<Result<(), ServerFnError>>> = use_signal(|| None);
    let mut user: Signal<Arc<Option<User>>> = use_context();
    let user_load_error: UserLoadError = use_context();

    let on_save = use_callback(move |()| async move {
        let results = do_logout().await;
        if results.is_ok() {
            let navigator = navigator();
            navigator.push(Route::Home {});
        }
        result.set(Some(results));
        user.set(Arc::new(None));
    });

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
                    div { class: "p-6 space-y-4 md:space-y-6 sm:p-8",
                        if let Err(err) = user_load_error.0() {
                            div { class: "bg-red-500 text-white p-2 text-center", {err.to_string()} }
                        }
                        if let Some(_user_object) = &*user() {
                            match result() {
                                Some(Ok(())) => {
                                    rsx! {
                                        div {
                                            h1 { "Logout success!" }
                                            form { novalidate: true, action: "javascript:void(0);",
                                                button {
                                                    r#type: "submit",
                                                    class: "w-full text-white bg-green-600 hover:bg-green-700 focus:ring-4 focus:outline-none focus:ring-primary-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-primary-600 dark:hover:bg-primary-700 dark:focus:ring-primary-800",
                                                    autofocus: true,
                                                    onclick: move |_| {
                                                        let navigator = navigator();
                                                        navigator.push(Route::Home {});
                                                    },
                                                    "Home"
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
                                            form { novalidate: true, action: "javascript:void(0);",
                                                button {
                                                    r#type: "submit",
                                                    class: "w-full text-white bg-green-600 hover:bg-green-700 focus:ring-4 focus:outline-none focus:ring-primary-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-primary-600 dark:hover:bg-primary-700 dark:focus:ring-primary-800",
                                                    autofocus: true,
                                                    onclick: move |_| async move {
                                                        on_save(()).await;
                                                    },
                                                    "Logout"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            h1 { "You are not logged in!" }
                            form {
                                novalidate: true,
                                action: "javascript:void(0);",
                                button {
                                    r#type: "submit",
                                    class: "w-full text-white bg-green-600 hover:bg-green-700 focus:ring-4 focus:outline-none focus:ring-primary-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-primary-600 dark:hover:bg-primary-700 dark:focus:ring-primary-800",
                                    autofocus: true,
                                    onclick: move |_| {
                                        let navigator = navigator();
                                        navigator.push(Route::Login {});
                                    },
                                    "Login"
                                }
                            }
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

    let user = match session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            error!("Invalid credentials: {:?}", creds);
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
