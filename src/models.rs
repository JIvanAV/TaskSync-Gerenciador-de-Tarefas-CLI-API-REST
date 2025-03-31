use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewTask {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}
