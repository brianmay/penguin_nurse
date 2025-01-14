use std::sync::Arc;

use chrono::NaiveDate;
use dioxus::prelude::*;

use components::navbar::Navbar;
use models::{User, UserId};
use views::{get_user, ConsumableList, Home, Login, Logout, TimelineList, UserDetail, UserList};

mod components;
mod dt;
mod forms;
mod functions;
mod models;
mod version;
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
    Home {  },
    #[route("/:date")]
    TimelineList { date: NaiveDate },
    #[route("/users")]
    UserList {},
    #[route("/users/:user_id")]
    UserDetail { user_id: UserId },
    #[route("/consumables")]
    ConsumableList {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

const MEDICAL_SVG: Asset = asset!("/assets/medical.svg");
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

#[derive(Debug, Clone)]
struct UserLoadError(Signal<Result<(), ServerFnError>>);

#[component]
fn App() -> Element {
    let mut user: Signal<Arc<Option<User>>> = use_signal(|| Arc::new(None));
    let mut result: Signal<Result<(), ServerFnError>> = use_signal(|| Ok(()));

    use_context_provider(|| user);
    use_context_provider(|| UserLoadError(result));

    use_future(move || async move {
        let data = get_user().await;

        let the_error = match &data {
            Ok(Some(_user)) => Ok(()),
            Ok(None) => Ok(()),
            Err(err) => Err(err.clone()),
        };

        let the_user = match data {
            Ok(Some(user)) => Some(user),
            Ok(None) => None,
            Err(_err) => None,
        };

        result.set(the_error);
        user.set(Arc::new(the_user));
    });

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: MEDICAL_SVG }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    let segments = segments.join("/");
    rsx! {
        div {
            main { role: "main", class: "container",
                h1 { "404 Not Found" }
                p { "The page you are looking for does not exist." }
                p { "Segments: {segments}" }
                p { "Please ask a friendly penguin for help." }
            }
        }
    }
}
