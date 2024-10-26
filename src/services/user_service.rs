use std::sync::Arc;
use crate::{
  entities::user_entity::User, 
  middlewares::errors::CustomError, 
  repository::user_repository::UserRepository,
  controllers::user_controller::UserSchema
};
use anyhow::Result;

#[derive(Clone)]
pub struct UserService {
  repository: Arc<UserRepository>
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
      UserService { repository: Arc::new(repository) }
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, CustomError> {
      self.repository.get_all_users().await.map_err(|_| CustomError::InternalServerError)
    }

    pub async fn create(&self, form: UserSchema) -> Result<User, CustomError> {
      self.repository.create_user(form).await.map_err(|_| CustomError::InternalServerError)
    }
}