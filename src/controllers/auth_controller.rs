use crate::{
    entities::login_entity::Login, errors_handler::errors::CustomError,
    services::auth_service::AuthService,
};
use actix_web::{web, HttpResponse, Responder};

pub async fn login(
    service: web::Data<AuthService>,
    data: web::Json<Login>,
) -> Result<impl Responder, CustomError> {
  let token = service.login(data.into_inner()).await?;
  Ok(HttpResponse::Ok().json(token))
}
