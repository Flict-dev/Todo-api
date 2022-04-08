use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u32,
}

#[derive(Deserialize)]
pub struct ToDoConfig {
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config,
}

impl ToDoConfig {
    pub fn new() -> Result<ToDoConfig, ConfigError> {
        let config = Config::builder()
            .add_source(config::Environment::default())
            .build()?;
        let todo_config: ToDoConfig = config.try_deserialize().expect("Fail to deserialize");
        Ok(todo_config)
    }
}
