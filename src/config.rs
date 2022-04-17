use config::{Config, ConfigError};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2;
use serde::Deserialize;
use slog::{o, Drain, Logger};
use slog_async;
use slog_term;

use crate::AppState;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u32,
}

#[derive(Deserialize)]
pub struct ToDoConfig {
    pub server: ServerConfig,
    pub database_url: String,
}

impl ToDoConfig {
    pub fn new() -> Result<ToDoConfig, ConfigError> {
        let config = Config::builder()
            .add_source(config::Environment::default())
            .build()?;
        let todo_config: ToDoConfig = config.try_deserialize().expect("Fail to deserialize");
        Ok(todo_config)
    }

    fn configure_logger(&self) -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
        let console_drain = slog_async::Async::new(console_drain).build().fuse();
        slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
    }

    pub fn new_state(&self) -> AppState {
        let logger = self.configure_logger();
        let pool: ConnectionManager<PgConnection> = ConnectionManager::new(&self.database_url);
        let pool = r2d2::Pool::builder()
            .build(pool)
            .expect("Failed to create pool.");
        AppState { pool, logger }
    }
}
