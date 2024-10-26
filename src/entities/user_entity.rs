use serde::{Deserialize, Serialize};
use validator::Validate;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Validate)]
pub struct User {
  pub id: i32,
  pub name: String,
  pub email: String
}