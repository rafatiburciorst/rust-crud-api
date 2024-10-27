use crate::{errors_handler::errors::CustomError, services::user_service::UserService};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct UserSchema {
    #[validate(length(min = 3, message = "name shall be min 3 characters"))]
    pub name: String,
    #[validate(email(message = "shall be a valid email"))]
    pub email: String,
    #[validate(length(min = 6, message = "name shall be min 3 characters"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct EmailSchema {
    #[validate(email(message = "shall be a valid email"))]
    pub email: String
}

pub async fn get_all_users(service: web::Data<UserService>) -> Result<impl Responder, CustomError> {
    let users = service.get_all_users().await?;
    Ok(HttpResponse::Ok().json(users))
}

pub async fn create(
    form: web::Json<UserSchema>,
    service: web::Data<UserService>,
) -> Result<impl Responder, CustomError> {
    service
        .create(form.into_inner())
        .await?;
    Ok(HttpResponse::Created())
}


