use crate::controllers::post_controller::{create, find_all, find_by_id};
use actix_web::web;
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .route("", web::get().to(find_all))
            .route("", web::post().to(create))
            .route("/{id}", web::post().to(find_by_id)),
    );
}
