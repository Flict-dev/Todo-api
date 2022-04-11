mod config;
mod db;
mod db_models;
mod errors;
mod handlers;
mod req_models;

// use actix_web::{web, App, HttpServer, Responder};

use crate::handlers::*;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use deadpool_postgres::{Pool, Runtime};
use dotenv;
use slog::{info, o, Drain, Logger};
use slog_async;
use slog_term;
use std::io;
use tokio_postgres::NoTls;

fn configure_logger() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
    let console_drain = slog_async::Async::new(console_drain).build().fuse();
    slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
}

pub struct AppState {
    pub pool: Pool,
    pub logger: Logger,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    let config = crate::config::ToDoConfig::new().unwrap();
    let pool = config.pg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    let logger = configure_logger();

    info!(
        logger,
        "Starting web server at {}:{}", config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                pool: pool.clone(),
                logger: logger.clone(),
            }))
            .service(status)
            .service(get_todos)
            .service(create_list)
            .service(get_items)
            .service(create_item)
            .service(check_todo)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
