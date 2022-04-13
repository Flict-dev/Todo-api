mod config;
mod db;
mod db_models;
mod errors;
mod handlers;
mod req_models;

use crate::handlers::*;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use deadpool_postgres::Pool;
use dotenv;
use slog::{info, Logger};
use std::io;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
    pub logger: Logger,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    let config = crate::config::ToDoConfig::new().unwrap();
    let state = config.new_state();

    info!(
        state.logger,
        "Starting web server at {}:{}", config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
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

#[cfg(test)]
mod integration_tests;