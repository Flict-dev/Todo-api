#[macro_use]
extern crate diesel;

mod apps;
mod config;
mod errors;
pub mod schema;
mod utils;

use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use apps::todo::controllers::*;
use apps::todo_item::controllers::*;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv;
use r2d2::Pool;
use r2d2::PooledConnection;
use slog::{info, Logger};
use std::io;

type DbPool = Pool<ConnectionManager<PgConnection>>;
type Connection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
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
            .service(
                web::scope("/api")
                    .service(get_todos)
                    .service(create_todo)
                    .service(get_items)
                    .service(create_item)
                    .service(check_todo_item),
            )
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
