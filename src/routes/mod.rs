pub mod user_routes;
pub mod post_routes;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
  user_routes::config(cfg);
  post_routes::config(cfg);
}