use serde::{Deserialize, Serialize};

// Types from database::models that frontend requires. This excludes secrets such as the users password.

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub full_name: String,
    pub oidc_id: Option<String>,
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
    pub oidc_id: Option<String>,
    pub email: String,
    pub is_admin: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub password: Option<String>,
    pub full_name: Option<String>,
    pub oidc_id: Option<Option<String>>,
    pub email: Option<String>,
    pub is_admin: Option<bool>,
}
