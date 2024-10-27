use crate::{errors_handler::errors::CustomError, services::post_service::PostService};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostSchema {
    pub title: String,
    pub text: String,
    pub author_id: i32,
}

pub async fn find_all(service: web::Data<PostService>) -> Result<impl Responder, CustomError> {
    let posts = service.find_all().await?;
    Ok(HttpResponse::Ok().json(posts))
}

pub async fn create(
    service: web::Data<PostService>,
    data: web::Json<PostSchema>,
) -> Result<impl Responder, CustomError> {
    service.create(data.into_inner()).await?;
    Ok(HttpResponse::Created())
}

pub async fn find_by_id(
    service: web::Data<PostService>,
    id: web::Path<i32>,
) -> Result<impl Responder, CustomError> {
    let post = service.find_by_id(*id).await?;
    Ok(HttpResponse::Ok().json(post))
}
