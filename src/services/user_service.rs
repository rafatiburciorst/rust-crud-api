use crate::{
    controllers::user_controller::UserSchema, entities::user_entity::User,
    errors_handler::errors::CustomError, repository::user_repository::UserRepository,
};
use anyhow::Result;
use std::sync::Arc;
use bcrypt::{hash, DEFAULT_COST};

#[derive(Clone)]
pub struct UserService {
    repository: Arc<UserRepository>,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        UserService {
            repository: Arc::new(repository),
        }
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, CustomError> {
        self.repository
            .get_all_users()
            .await
            .map_err(|_| CustomError::InternalServerError)
    }

    pub async fn create(&self, mut form: UserSchema) -> Result<(), CustomError> {
        match self.find_by_email(form.email.clone()).await {
            Ok(_) => Err(CustomError::UserAlreadyExists),
            Err(_) => {
                let hash_password = self.hash_password(form.password);
                match hash_password {
                    Ok(hash_password) => {
                        form.password = hash_password;
                    }
                    Err(_) => {
                        return Err(CustomError::InternalServerError);
                    }
                }
                
                self.repository.create_user(form).await?;
                Ok(())
            }
        }
    }

    async fn find_by_email(&self, email: String) -> Result<User, CustomError> {
        let user = self.repository.find_by_email(email).await?;
        Ok(user)
    }

    fn hash_password(&self, password: String) -> Result<String, bcrypt::BcryptError> {
        let hash_password = hash(password, DEFAULT_COST)?;
        Ok(hash_password)
    }
}
