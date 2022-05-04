#[macro_use]
extern crate diesel;

mod apps;
mod config;
mod errors;
mod middlewares;
mod schema;
mod utils;

use crate::apps::user::crypto::Crypto;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use apps::td_controllers;
use apps::ti_controllers;
use apps::u_controllers;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv;
use paperclip::actix::{web, OpenApiExt};
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
    pub crypto: Crypto,
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
            .wrap_api()
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/todos")
                            .configure(td_controllers::init_routes)
                            .configure(ti_controllers::init_routes),
                    )
                    .service(web::scope("/user").configure(u_controllers::init_routes)),
            )
            .with_json_spec_at("/api/spec/v2")
            .build()
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
