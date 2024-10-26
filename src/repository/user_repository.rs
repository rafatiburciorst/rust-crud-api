use crate::entities::user_entity::User;
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
        let users = sqlx::query_as!(User, "SELECT id, name, email FROM users")
            .fetch_all(&self.pool)
            .await?;
        Ok(users)
    }

    pub async fn create_user(&self, form: UserSchema) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
            form.name,
            form.email
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }
}
