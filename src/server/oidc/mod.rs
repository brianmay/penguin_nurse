mod claims;
pub mod middleware;

use openid::{error::ClientError, Discovered, Options};
use thiserror::Error;
use tracing::debug;
use url::Url;

use crate::server::database::models::users::{
    create_user, get_user_by_email, get_user_by_oidc_id, update_user,
};

use super::database::{self, connection::DatabasePool, models::users::User};

#[derive(Debug, Clone)]
pub struct Config {
    pub issuer: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scopes: String,
}

type OpenIdClient = openid::Client<Discovered, claims::StandardClaims>;
type Token = openid::Token<claims::StandardClaims>;

pub struct Client {
    oidc_client: OpenIdClient,
    config: Config,
}

#[derive(Error, Debug)]
pub enum Error {
    // Parse error
    #[error("Parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    // OIDC error
    #[error("OpenID error: {0}")]
    OpenId(#[from] openid::error::Error),

    // OIDC error
    #[error("OpenID Client error: {0}")]
    OpenIdClient(#[from] ClientError),

    // No Token error
    #[error("No token")]
    NoToken,

    // Request token error
    #[error("Request token error: {0}")]
    RequestToken(ClientError),

    // Token decode error
    #[error("Token decode error: {0}")]
    TokenDecode(openid::error::Error),

    // Token validation error
    #[error("Token validation error: {0}")]
    TokenValidation(openid::error::Error),

    // Request user info error
    #[error("Request user info error: {0}")]
    RequestUserInfo(openid::error::Error),

    // User info missing error
    #[error("User info missing {0}")]
    UserInfoMissing(String),

    // Database error
    #[error("Database error: {0}")]
    Database(#[from] database::connection::Error),
}

impl Client {
    pub async fn new(config: &Config) -> Result<Client, Error> {
        let cloned_config = config.clone();

        let client_id = config.client_id.clone();
        let client_secret = config.client_secret.clone();
        let redirect = Some(config.redirect_uri.clone());
        let issuer = Url::parse(&config.issuer)?;

        let client = OpenIdClient::discover(client_id, client_secret, redirect, issuer).await?;

        let client = Client {
            oidc_client: client,
            config: cloned_config,
        };

        Ok(client)
    }

    pub fn get_auth_url(&self, origin_url: &str) -> String {
        let auth_url = self.oidc_client.auth_url(&Options {
            scope: Some(self.config.scopes.to_string()),
            state: Some(origin_url.to_string()),
            ..Default::default()
        });

        auth_url.into()
    }

    async fn login(&self, code: &str, pool: &DatabasePool) -> Result<User, Error> {
        let mut token: Token = self
            .oidc_client
            .request_token(code)
            .await
            .map_err(Error::RequestToken)?
            .into();

        if let Some(id_token) = token.id_token.as_mut() {
            debug!("token: {:?}", id_token);
            self.oidc_client
                .decode_token(id_token)
                .map_err(Error::TokenDecode)?;
            self.oidc_client
                .validate_token(id_token, None, None)
                .map_err(Error::TokenValidation)?;
            debug!("token: {:?}", id_token);
        } else {
            return Err(Error::NoToken);
        }

        let no_groups = vec![];
        let groups = token
            .id_token
            .as_ref()
            .and_then(|id_token| id_token.payload().ok())
            .map_or(&no_groups, |claims| &claims.groups)
            .clone();

        let user_info = self
            .oidc_client
            .request_userinfo(&token)
            .await
            .map_err(Error::RequestUserInfo)?;

        println!("groups: {:#?}", groups);
        println!("user info: {:#?}", user_info);

        let sub = user_info
            .sub
            .ok_or_else(|| Error::UserInfoMissing("sub".into()))?;

        let name = user_info
            .name
            .ok_or_else(|| Error::UserInfoMissing("name".into()))?;

        let email = user_info
            .email
            .ok_or_else(|| Error::UserInfoMissing("email".into()))?;

        let is_admin = groups.contains(&"admin".to_string());

        let mut conn = pool
            .get()
            .await
            .map_err(database::connection::Error::Mobc)?;

        let user = get_user_by_oidc_id(&mut conn, &sub)
            .await
            .map_err(database::connection::Error::Diesel)?;

        let user = if let Some(user) = user {
            Some(user)
        } else {
            get_user_by_email(&mut conn, &email)
                .await
                .map_err(database::connection::Error::Diesel)?
        };

        let user = if let Some(user) = user {
            let updates = database::models::users::UpdateUser {
                full_name: None,
                oidc_id: Some(Some(sub.as_str())),
                email: None,
                is_admin: Some(is_admin),
                username: None,
                password: None,
            };

            update_user(&mut conn, user.id, updates)
                .await
                .map_err(database::connection::Error::Diesel)?
        } else {
            let updates = database::models::users::NewUser {
                full_name: name.as_str(),
                oidc_id: Some(sub.as_str()),
                email: email.as_str(),
                is_admin,
                username: name.as_str(),
                password: "",
            };
            create_user(&mut conn, updates)
                .await
                .map_err(database::connection::Error::Diesel)?
        };

        Ok(user)
    }
}
