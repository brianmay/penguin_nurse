use std::{any::Any, sync::Arc};

use dioxus::prelude::*;

pub mod database;
mod handlers;

use handlers::dioxus_handler;

// The entry point for the server
#[cfg(feature = "server")]
pub async fn init(app: fn() -> Element) {
    use axum::{routing::get, Extension};
    use handlers::health_check;

    tracing_subscriber::fmt::init();

    let database = database::init().await;

    // Get the address the server should run on. If the CLI is running, the CLI proxies fullstack into the main address
    // and we use the generated address the CLI gives us
    let address = dioxus_cli_config::fullstack_address_or_localhost();

    let cfg = ServeConfigBuilder::default();

    // Set up the axum router
    let router = axum::Router::new()
        // You can add a dioxus application to the router with the `serve_dioxus_application` method
        // This will add a fallback route to the router that will serve your component and server functions
        .serve_dioxus_application(cfg, app)
        .route("/_health", get(health_check))
        .route("/_dioxus", get(dioxus_handler))
        .layer(Extension(database));

    // Finally, we can launch the server
    let router = router.into_make_service();
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
