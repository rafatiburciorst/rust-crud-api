use std::env;

use anyhow::Ok;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use anyhow::Result;

pub async fn init_db_pool() -> Result<PgPool> {
  dotenv::dotenv().ok();

  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE URL WAS NOT DEFINED");

  let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await?;

  Ok(pool)
}