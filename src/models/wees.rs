use serde::{Deserialize, Serialize};

use super::common::MaybeString;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Wee {
    pub id: i64,
    pub user_id: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub duration: chrono::Duration,
    pub urgency: i32,
    pub mls: i32,
    pub colour: palette::Hsv,
    pub comments: MaybeString,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NewWee {
    pub user_id: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub duration: chrono::Duration,
    pub urgency: i32,
    pub mls: i32,
    pub colour: palette::Hsv,
    pub comments: MaybeString,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UpdateWee {
    pub user_id: Option<i64>,
    pub time: Option<chrono::DateTime<chrono::Utc>>,
    pub duration: Option<chrono::Duration>,
    pub urgency: Option<i32>,
    pub mls: Option<i32>,
    pub colour: Option<palette::Hsv>,
    pub comments: Option<MaybeString>,
}
