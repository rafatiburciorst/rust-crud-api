use crate::{
    entities::login_entity::{Claims, Login},
    errors_handler::errors::CustomError,
    repository::user_repository::UserRepository,
};
use chrono::{Duration, Utc};
use std::env;

use super::user_service::UserService;

#[derive(Debug, Clone)]
pub struct AuthService {
    repository: UserRepository,
    user_service: UserService,
}

impl AuthService {
    pub fn new(repository: UserRepository, user_service: UserService) -> AuthService {
        AuthService {
            repository,
            user_service,
        }
    }

    pub async fn login(&self, data: Login) -> Result<(), CustomError> {
        let user = self.repository.find_by_email(data.email).await?;
        let password = user.password.as_str();
        let verify_password = self.user_service.verify_password(data.password, password)?;

        Ok(())
    }
}
