use crate::{
    controllers::post_controller::PostSchema, entities::post_entity::Post,
    errors_handler::errors::CustomError,
};
use anyhow::Result;
use sqlx::PgPool;

#[derive(Clone)]
pub struct PostRepository {
    poll: PgPool,
}

impl PostRepository {
    pub fn new(poll: PgPool) -> PostRepository {
        PostRepository { poll }
    }

    pub async fn find_all(&self) -> Result<Vec<Post>, CustomError> {
        let posts = sqlx::query_as!(Post, "SELECT * FROM posts")
            .fetch_all(&self.poll)
            .await?;
        Ok(posts)
    }

    pub async fn create(&self, data: PostSchema) -> Result<(), CustomError> {
        sqlx::query!(
            "INSERT INTO posts (title, text, author_id) VALUES ($1, $2, $3)",
            data.title,
            data.text,
            data.author_id
        )
        .execute(&self.poll)
        .await?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Post, CustomError> {
        let post = sqlx::query_as!(Post, "SELECT * FROM posts WHERE id = $1", id)
            .fetch_one(&self.poll)
            .await?;
        Ok(post)
    }
}
