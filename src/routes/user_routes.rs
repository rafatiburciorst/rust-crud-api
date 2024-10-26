use actix_web::web;
use crate::controllers::user_controller::{get_all_users, create};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/users")
      .route("", web::get().to(get_all_users))
      .route("", web::post().to(create))
  );
}