mod config;
mod db;
mod handlers;
mod models;

// use actix_web::{web, App, HttpServer, Responder};

use crate::handlers::*;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use deadpool_postgres::Runtime;
use dotenv;
use std::io;
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=trace");
    env_logger::init();
    dotenv::dotenv().ok();

    let config = crate::config::ToDoConfig::new().unwrap();
    let pool = config.pg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .route("/", web::get().to(status))
            .route("/todos{_:/?}", web::get().to(get_todos))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
