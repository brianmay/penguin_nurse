use std::str::FromStr;

use derive_enum_all_values::AllValues;

use crate::models::MaybeSet;

use serde::{Deserialize, Serialize};

// Types from database::models that frontend requires. This excludes secrets such as the users password.

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, AllValues)]
pub enum Sex {
    Male,
    Female,
}

impl Sex {
    pub fn as_id(&self) -> &'static str {
        match self {
            Self::Male => "male",
            Self::Female => "female",
        }
    }

    pub fn as_title(&self) -> &'static str {
        match self {
            Self::Male => "Male",
            Self::Female => "Female",
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UserId(i64);

impl UserId {
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
    pub oidc_id: Option<String>,
    pub email: String,
    pub is_admin: bool,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub sex: Option<Sex>,
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
    pub sex: Option<Sex>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChangeUser {
    pub username: MaybeSet<String>,
    // pub password: MaybeSet<String>,
    pub full_name: MaybeSet<String>,
    pub oidc_id: MaybeSet<Option<String>>,
    pub email: MaybeSet<String>,
    pub is_admin: MaybeSet<bool>,
    pub date_of_birth: MaybeSet<Option<chrono::NaiveDate>>,
    pub sex: MaybeSet<Option<Sex>>,
}
