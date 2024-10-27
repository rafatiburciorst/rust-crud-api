use crate::{
    controllers::post_controller::PostSchema, entities::post_entity::Post, errors_handler::errors::CustomError, repository::post_repository::PostRepository
};

#[derive(Clone)]
pub struct PostService {
    respository: PostRepository,
}

impl PostService {
    pub fn new(respository: PostRepository) -> PostService {
        PostService { respository }
    }

    pub async fn find_all(&self) -> Result<Vec<Post>, CustomError> {
        let posts = self.respository.find_all().await?;
        Ok(posts)
    }

    pub async fn create(&self, data: PostSchema) -> Result<(), CustomError> {
        self.respository.create(data).await?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Post, CustomError> {
        let post = self.respository.find_by_id(id).await?;
        Ok(post)
    }
}
