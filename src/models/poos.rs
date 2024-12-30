use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Poo {
    pub id: i64,
    pub user_id: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub duration: chrono::Duration,
    pub urgency: i32,
    pub quantity: i32,
    pub bristol: i32,
    pub colour: palette::Hsv,
    pub comments: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NewPoo {
    pub user_id: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub duration: chrono::Duration,
    pub urgency: i32,
    pub quantity: i32,
    pub bristol: i32,
    pub colour: palette::Hsv,
    pub comments: Option<String>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UpdatePoo {
    pub user_id: Option<i64>,
    pub time: Option<chrono::DateTime<chrono::Utc>>,
    pub duration: Option<chrono::Duration>,
    pub urgency: Option<i32>,
    pub quantity: Option<i32>,
    pub bristol: Option<i32>,
    pub colour: Option<palette::Hsv>,
    pub comments: Option<Option<String>>,
}
