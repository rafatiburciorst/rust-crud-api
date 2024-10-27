use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
  pub id: i32,
  pub name: String,
  pub email: String,
  pub created_at: Option<DateTime<Utc>>,
}