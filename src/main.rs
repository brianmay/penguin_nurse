use std::sync::Arc;

use chrono::NaiveDate;
use dioxus::prelude::*;

use components::navbar::Navbar;
use models::{ConsumableId, ConsumptionId, PooId, User, UserId, WeeId};
use views::{
    get_user, ConsumableDetail, ConsumableList, ConsumptionDetail, Home, Login, Logout, PooDetail,
    TimelineList, UserDetail, UserList, WeeDetail,
};

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
    #[route("/wees/:wee_id")]
    WeeDetail { wee_id: WeeId },
    #[route("/poos/:poo_id")]
    PooDetail { poo_id: PooId },
    #[route("/consumptions/:consumption_id")]
    ConsumptionDetail { consumption_id: ConsumptionId },
    #[route("/consumables/:consumable_id")]
    ConsumableDetail { consumable_id: ConsumableId },
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

fn reload_user() {
    let mut user_resource: Resource<Result<Option<Arc<User>>, ServerFnError>> = use_context();
    user_resource.restart();
}

fn use_user() -> Result<Option<Arc<User>>, ServerFnError> {
    let user_resource: Resource<Result<Option<Arc<User>>, ServerFnError>> = use_context();
    let user_result: &Option<Result<Option<Arc<User>>, ServerFnError>> = &user_resource.read();
    let user_result = user_result.as_ref().map_or_else(
        || Err(ServerFnError::ServerError("Mo user resource".to_string())),
        |x| x.clone(),
    );
    user_result
}

#[component]
fn App() -> Element {
    let user_resource = use_server_future(move || async move {
        let data = get_user().await;
        data.map(|x| x.map(Arc::new))
    })?;

    use_context_provider(|| user_resource);

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
