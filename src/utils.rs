use crate::errors::AppError;
use crate::{Connection, DbPool};
use slog::{crit, error, o, Logger};

pub fn get_db_conn(poll: &DbPool, logger: &Logger) -> Result<Connection, AppError> {
    let log = logger.new(o!("handler" => "get_db_client"));

    poll.get().map_err(|err| {
        let sublog = log.new(o!("cause" => err.to_string()));
        crit!(sublog, "Error with connection to database");
        AppError::db_not_found(err.to_string())
    })
}

pub fn log_error(log: Logger) -> Box<dyn Fn(AppError) -> AppError> {
    Box::new(move |error| {
        let sublog = log.new(o!("cause" => error.cause.clone()));
        error!(sublog, "{}", error.message());
        error
    })
}
