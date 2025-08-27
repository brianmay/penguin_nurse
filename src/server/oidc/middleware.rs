use std::{collections::HashMap, env, sync::Arc};

use arc_swap::ArcSwap;
use axum::{
    Extension,
    extract::{Query, Request},
    http::StatusCode,
    middleware::{Next, from_fn},
    response::{IntoResponse, Redirect, Response},
};
use tap::Pipe;
use thiserror::Error;
use url::Url;

use crate::server::{
    auth::{AuthError, AuthSession},
    database::connection::DatabasePool,
    oidc::{Client, Config},
};

#[derive(Debug, Error)]
enum Error {
    #[error("OIDC client not initialized")]
    NotInitialized,
    #[error(transparent)]
    Oidc(#[from] super::Error),
    #[error(transparent)]
    AxumLogin(#[from] AuthError),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

pub type ClientState = Arc<ArcSwap<Option<Client>>>;

pub fn add_oidc_middleware<S: Clone + Send + Sync + 'static>(
    app: axum::Router<S>,
) -> axum::Router<S> {
    if env::var("OIDC_DISCOVERY_URL").is_err() {
        return app;
    }

    let base_url = env::var("BASE_URL").expect("BASE_URL must be set");
    let issuer = env::var("OIDC_DISCOVERY_URL").expect("OIDC_DISCOVERY_URL must be set");
    let client_id = env::var("OIDC_CLIENT_ID").expect("OIDC_CLIENT_ID must be set");
    let client_secret = env::var("OIDC_CLIENT_SECRET").expect("OIDC_CLIENT_SECRET must be set");
    let auth_scope = env::var("OIDC_AUTH_SCOPE").expect("OIDC_AUTH_SCOPE must be set");

    let redirect_uri = Url::parse(&base_url)
        .expect("BASE_URL must be a valid URL")
        .join("/openid_connect_redirect_uri")
        .expect("failed to join redirect_uri");

    let oidc_config = Config {
        issuer,
        client_id,
        client_secret,
        redirect_uri: redirect_uri.to_string(),
        scopes: auth_scope,
    };

    let oidc_client = Arc::new(ArcSwap::new(Arc::new(None)));

    {
        let client = oidc_client.clone();
        tokio::spawn(async move {
            loop {
                tracing::info!("refreshing oidc client");
                let new_client = Client::new(&oidc_config).await;
                match new_client {
                    Ok(new_client) => {
                        client.store(Arc::new(Some(new_client)));
                        tokio::time::sleep(tokio::time::Duration::from_secs(10 * 60)).await;
                    }
                    Err(e) => {
                        tracing::error!("failed to refresh oidc client: {}", e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                    }
                }
            }
        });
    }

    let layer = from_fn(auth);

    app.layer(layer).layer(Extension(oidc_client))
}

// #[axum::debug_middleware]
async fn auth(
    Extension(oidc_client): Extension<ClientState>,
    Extension(database_pool): Extension<DatabasePool>,
    Query(params): Query<HashMap<String, String>>,
    mut session: Extension<AuthSession>,
    request: Request,
    next: Next,
) -> Result<Response, Error> {
    if request.uri().path() == "/openid_connect_redirect_uri" {
        let code = params.get("code").cloned().unwrap_or_default();

        let state = params
            .get("state")
            .cloned()
            .unwrap_or_else(|| "/".to_string());

        let oidc_client = oidc_client.load();
        let Some(oidc_client) = oidc_client.as_ref() else {
            return Err(Error::NotInitialized);
        };

        let user = oidc_client.login(&code, &database_pool).await?;
        session.login(&user).await?;

        return Redirect::to(&state).into_response().pipe(Ok);
    }

    next.run(request).await.pipe(Ok)
}
