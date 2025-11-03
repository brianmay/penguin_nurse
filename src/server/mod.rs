use dioxus::prelude::*;

pub mod auth;
// pub mod context;
pub mod database;
mod handlers;
mod oidc;
mod session_store;

use axum::{Extension, routing::get};
use handlers::{dioxus_handler, health_check};
use time::Duration;
use tower_sessions::session_store::ExpiredDeletion;
use tower_sessions::{Expiry, SessionManagerLayer, cookie::SameSite};

pub use oidc::middleware::ClientState as OidcClientState;

// The entry point for the server
#[cfg(feature = "server")]
pub fn init(app: fn() -> Element) {
    use axum_login::AuthManagerLayerBuilder;
    use oidc::middleware::add_oidc_middleware;
    use tap::Pipe;

    dioxus::serve(move || async move {
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
                .with_always_save(true)
        };

        let (auth_layer, auth_manager) = {
            // Auth service.
            //
            // This combines the session layer with our backend to establish the auth
            // service which will provide the auth session as a request extension.

            use std::sync::Arc;

            use axum_login::AuthManager;
            let backend = auth::Backend::new(database.clone());

            let layer = AuthManagerLayerBuilder::new(backend.clone(), session_layer).build();
            let manager = Arc::new(AuthManager::new((), backend, "axum-login.data"));

            (layer, manager)
        };

        let cfg = ServeConfig::new();

        axum::Router::new()
            // You can add a dioxus application to the router with the `serve_dioxus_application` method
            // This will add a fallback route to the router that will serve your component and server functions
            // .serve_static_assets()
            .serve_dioxus_application(cfg, app)
            .route("/_health", get(health_check))
            .route("/_dioxus", get(dioxus_handler))
            .pipe(add_oidc_middleware)
            .layer(axum::middleware::from_fn(auth::session_middleware))
            .layer(auth_layer)
            .layer(Extension(database))
            .layer(Extension(auth_manager))
            .pipe(Ok)
    });
}
