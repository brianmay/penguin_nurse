use std::sync::Arc;

use dioxus::prelude::*;

use components::Navbar;
use models::User;
use views::{get_user, Blog, Home, Login, Logout};

mod components;
mod models;
mod views;

#[cfg(feature = "server")]
mod server;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/login")]
    Login {},
    #[route("/logout")]
    Logout {},
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    server::init(App).await;
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    let mut user: Signal<Arc<Option<Result<User, ServerFnError>>>> = use_signal(|| Arc::new(None));

    use_context_provider(|| user);

    use_future(move || async move {
        let current_user = get_user().await;

        let current_user = match current_user {
            Ok(Some(user)) => Some(Ok(user)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        };

        user.set(Arc::new(current_user));
    });

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
