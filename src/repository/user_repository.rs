use crate::entities::user_entity::{User, DbUser};
use crate::controllers::user_controller::UserSchema;
use anyhow::Result;
use sqlx::PgPool;

#[derive(Debug)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        UserRepository { pool }
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as!(User, "SELECT id, name, email, created_at FROM users")
            .fetch_all(&self.pool)
            .await?;
        Ok(users)
    }

    pub async fn create_user(&self, form: UserSchema) -> Result<DbUser> {
        let user = sqlx::query_as!(
            DbUser,
            "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id, name, email, created_at, password",
            form.name,
            form.email,
            form.password
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }
}
