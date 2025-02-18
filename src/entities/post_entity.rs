use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Post {
  pub id: i32,
  pub title: String,
  pub text: String,
  pub author_id: i32,
  pub password: String,
  pub created_at: Option<DateTime<Utc>>,
}