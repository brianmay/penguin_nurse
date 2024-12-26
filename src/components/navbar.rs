use std::sync::Arc;

use crate::{models::User, Route, UserLoadError};
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");
const NURSE_SVG: Asset = asset!("/assets/nurse.svg");

#[component]
pub fn MenuItem(route: Route, title: String) -> Element {
    let current: Route = use_route();

    let class = if current == route {
        "block py-2 px-3 text-white bg-blue-700 rounded md:bg-transparent md:text-blue-700 md:p-0 md:dark:text-blue-500 dark:bg-blue-600 md:dark:bg-transparent"
    } else {
        "block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent"
    };

    rsx! {
        li {
            Link { to: route, "aria-current": "page", class, {title} }
        }
    }
}

#[component]
pub fn Navbar() -> Element {
    let mut show_menu = use_signal(|| false);
    let user: Signal<Arc<Option<User>>> = use_context();
    let user_load_error: UserLoadError = use_context();

    let menu_class = if show_menu() { "" } else { "hidden" };

    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        nav { class: "bg-white border-gray-200 dark:bg-gray-900 dark:border-gray-700",
            div { class: "max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4",
                a {
                    href: "#",
                    class: "flex items-center space-x-3 rtl:space-x-reverse",
                    img { alt: "Nurse Logo", src: NURSE_SVG, class: "h-8" }
                    span { class: "self-center text-2xl font-semibold whitespace-nowrap dark:text-white",
                        "Penguin Nurse"
                    }
                }
                button {
                    "data-collapse-toggle": "navbar-multi-level",
                    "aria-controls": "navbar-multi-level",
                    "aria-expanded": show_menu(),
                    r#type: "button",
                    class: "inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600",
                    onclick: move |_e| {
                        show_menu.set(!show_menu());
                    },
                    span { class: "sr-only", "Open main menu" }
                    svg {
                        "aria-hidden": "true",
                        fill: "none",
                        "viewBox": "0 0 17 14",
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "w-5 h-5",
                        path {
                            "stroke-width": "2",
                            "stroke-linejoin": "round",
                            stroke: "currentColor",
                            d: "M1 1h15M1 7h15M1 13h15",
                            "stroke-linecap": "round",
                        }
                    }
                }
                div {
                    id: "navbar-multi-level",
                    class: "{menu_class} w-full md:block md:w-auto",
                    ul { class: "flex flex-col font-medium p-4 md:p-0 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:space-x-8 rtl:space-x-reverse md:flex-row md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700",
                        MenuItem { route: Route::Home {}, title: "Home" }
                        MenuItem { route: Route::Blog { id: 1 }, title: "Blog" }
                        if let Some(user) = user().as_ref() {
                            if user.is_admin {
                                MenuItem { route: Route::UserList {}, title: "Users" }
                            }
                            MenuItem { route: Route::Logout {}, title: "Logout" }
                        } else {
                            MenuItem { route: Route::Login {}, title: "Login" }
                        }
                    }
                }
            }
        }

        if let Err(err) = user_load_error.0() {
            div { class: "alert alert-error", {err.to_string()} }
        }

        Outlet::<Route> {}
    }
}
