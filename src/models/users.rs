use std::str::FromStr;

use super::common::MaybeString;
use serde::{Deserialize, Serialize};

// Types from database::models that frontend requires. This excludes secrets such as the users password.

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UserId(i64);

impl UserId {
    #[cfg(feature = "server")]
    pub fn new(id: i64) -> Self {
        Self(id)
    }
    pub fn as_inner(self) -> i64 {
        self.0
    }
}

impl FromStr for UserId {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub full_name: String,
    pub oidc_id: MaybeString,
    pub email: String,
    pub is_admin: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub full_name: String,
    pub oidc_id: MaybeString,
    pub email: String,
    pub is_admin: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub password: Option<String>,
    pub full_name: Option<String>,
    pub oidc_id: Option<MaybeString>,
    pub email: Option<String>,
    pub is_admin: Option<bool>,
}
