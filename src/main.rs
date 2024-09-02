use actix_web::{web, App, HttpServer, HttpResponse};
use dotenv::dotenv;
use env_logger::Env;
use diesel::prelude::*;
use actix_web::middleware::Logger;

mod api;
mod db;
mod handlers;
mod models;
mod services;
mod i18n;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(api::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

