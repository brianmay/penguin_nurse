use async_trait::async_trait;
use axum::{
    http,
    response::{IntoResponse, Response},
};
use axum_login::{AuthnBackend, UserId};
use password_auth::verify_password;
use serde::Deserialize;
use tap::Pipe;
use tokio::task;

use super::database::{
    self,
    connection::DatabasePool,
    models::users::{get_user_by_id, get_user_by_username, User},
};

// This allows us to extract the authentication fields from forms. We use this
// to authenticate requests with the backend.
#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    // pub next: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Backend {
    db: DatabasePool,
}

impl Backend {
    pub fn new(db: DatabasePool) -> Self {
        Self { db }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Database(#[from] database::connection::Error),

    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let mut conn = self
            .db
            .get()
            .await
            .map_err(database::connection::Error::from)
            .map_err(Error::Database)?;

        let user = get_user_by_username(&mut conn, &creds.username)
            .await
            .map_err(database::connection::Error::from)
            .map_err(Error::Database)?;

        // Verifying the password is blocking and potentially slow, so we'll do so via
        // `spawn_blocking`.
        task::spawn_blocking(|| {
            // We're using password-based authentication--this works by comparing our form
            // input with an argon2 password hash.
            Ok(user.filter(|user| verify_password(creds.password, &user.password).is_ok()))
        })
        .await?
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let mut conn = self
            .db
            .get()
            .await
            .map_err(database::connection::Error::from)
            .map_err(Error::Database)?;

        get_user_by_id(&mut conn, *user_id)
            .await
            .map_err(database::connection::Error::from)
            .map_err(Error::Database)?
            .pipe(Ok)
    }
}

// We use a type alias for convenience.
//
// Note that we've supplied our concrete backend here.
pub type AuthSession = axum_login::AuthSession<Backend>;
pub type AuthError = axum_login::Error<Backend>;

pub struct Session(AuthSession);

impl std::ops::Deref for Session {
    type Target = AuthSession;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Session {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct AuthSessionLayerNotFound;

impl std::fmt::Display for AuthSessionLayerNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthSessionLayer was not found")
    }
}

impl std::error::Error for AuthSessionLayerNotFound {}

impl IntoResponse for AuthSessionLayerNotFound {
    fn into_response(self) -> Response {
        (
            http::status::StatusCode::INTERNAL_SERVER_ERROR,
            "AuthSessionLayer was not found",
        )
            .into_response()
    }
}

#[async_trait]
impl<S: std::marker::Sync + std::marker::Send> axum::extract::FromRequestParts<S> for Session {
    type Rejection = AuthSessionLayerNotFound;

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        AuthSession::from_request_parts(parts, state)
            .await
            .map(Session)
            .map_err(|_| AuthSessionLayerNotFound)
    }
}
