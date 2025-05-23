use dioxus::prelude::*;

pub mod auth;
pub mod database;
mod handlers;
mod oidc;
mod session_store;

use axum::{routing::get, Extension};
use handlers::{dioxus_handler, health_check};
use time::Duration;
use tower_sessions::{cookie::SameSite, ExpiredDeletion, Expiry, SessionManagerLayer};

pub use oidc::middleware::ClientState as OidcClientState;

// The entry point for the server
#[cfg(feature = "server")]
pub async fn init(app: fn() -> Element) {
    use axum_login::AuthManagerLayerBuilder;
    use oidc::middleware::add_oidc_middleware;
    use tap::Pipe;

    tracing_subscriber::fmt::init();

    let database = database::connection::init().await;

    let session_layer = {
        let session_store = session_store::PostgresStore::new(database.clone());

        tokio::task::spawn(
            session_store
                .clone()
                .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
        );

        SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(Duration::days(7)))
            .with_same_site(SameSite::Lax)
    };

    let auth_layer = {
        // Auth service.
        //
        // This combines the session layer with our backend to establish the auth
        // service which will provide the auth session as a request extension.
        let backend = auth::Backend::new(database.clone());
        AuthManagerLayerBuilder::new(backend, session_layer).build()
    };

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
        .pipe(add_oidc_middleware)
        .layer(auth_layer)
        .layer(Extension(database));

    // Finally, we can launch the server
    let router = router.into_make_service();
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
