use crate::controllers::user_controller::UserSchema;
use crate::entities::user_entity::User;
use crate::errors_handler::errors::CustomError;
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

    pub async fn create_user(&self, form: UserSchema) -> Result<(), CustomError> {
        sqlx::query!(
            "INSERT INTO users (name, email, password) VALUES ($1, $2, $3)",
            form.name,
            form.email,
            form.password
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_by_email(&self, email: String) -> Result<User, CustomError> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, name, email, created_at FROM users WHERE email = $1",
            email
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }
}
