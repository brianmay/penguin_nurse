use std::sync::Arc;

use chrono::NaiveDate;
use dioxus::prelude::*;

use components::{consumables, consumptions, navbar::Navbar, poos, timeline, users, wees};
use models::{ConsumableId, ConsumptionId, PooId, User, UserId, WeeId};
use views::{
    ConsumableDetail, ConsumableList, ConsumptionDetail, Home, Login, Logout, PooDetail,
    TimelineList, UserDetail, UserList, WeeDetail, get_user,
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
    #[route("/:date?:dialog")]
    TimelineList { date: NaiveDate, dialog: timeline::DialogReference},
    #[route("/users?:dialog")]
    UserList { dialog: users::ListDialogReference },
    #[route("/users/:user_id?:dialog")]
    UserDetail { user_id: UserId, dialog: users::DetailsDialogReference },
    #[route("/wees/:wee_id?:dialog")]
    WeeDetail { wee_id: WeeId, dialog: wees::DialogReference },
    #[route("/poos/:poo_id?:dialog")]
    PooDetail { poo_id: PooId, dialog: poos::DialogReference },
    #[route("/consumptions/:consumption_id?:dialog")]
    ConsumptionDetail { consumption_id: ConsumptionId, dialog: consumptions::DialogReference},
    #[route("/consumables?:dialog")]
    ConsumableList {dialog: consumables::ListDialogReference },
    #[route("/consumables/:consumable_id?:dialog")]
    ConsumableDetail { consumable_id: ConsumableId, dialog: consumables::DetailsDialogReference },
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
    let _zbar = asset!("/assets/zbar.wasm");

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: MEDICAL_SVG }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        document::Script {
            type: "module",
            src: asset!("/assets/bundle.js", JsAssetOptions::new().with_minify(false)),
        }


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
