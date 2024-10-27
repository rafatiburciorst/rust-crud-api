mod controllers;
mod services;
mod routes;
mod repository;
mod entities;
mod config;
mod errors_handler;

use config::database::init_db_pool;
use routes::config as routes_config;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use log::info;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    env_logger::init();

    info!("Starting server");

    let pool = match init_db_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Error on connect to database: {}", e);
            std::process::exit(1);
        }
    };

    let user_reposity = repository::user_repository::UserRepository::new(pool.clone());
    let user_service = services::user_service::UserService::new(user_reposity);
    let post_repository = repository::post_repository::PostRepository::new(pool.clone());
    let post_service = services::post_service::PostService::new(post_repository);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_service.clone()))
            .app_data(web::Data::new(post_service.clone()))
            .configure(routes_config)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}