use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    NotFoundError,
    Unauthorized,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl AppError {
    pub fn message(&self) -> String {
        match &*self {
            AppError {
                message: Some(message),
                cause: _,
                error_type: _,
            } => message.clone(),

            AppError {
                message: None,
                cause: _,
                error_type: AppErrorType::DbError,
            } => String::from("The requested item was not found"),

            _ => String::from("An unexpected error has ocured"),
        }
    }

    pub fn db_error(error: impl ToString) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError,
        }
    }

    pub fn unauthorized() -> AppError {
        AppError {
            message: Some("User unauthorized".to_string()),
            cause: None,
            error_type: AppErrorType::NotFoundError,
        }
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
            AppErrorType::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }
}

#[cfg(test)]
mod tests {

    use super::{AppError, AppErrorType};

    #[test]
    fn test_default_message() {
        let db_err = AppError {
            message: None,
            cause: None,
            error_type: AppErrorType::DbError,
        };

        assert_eq!(
            db_err.message(),
            "The requested item was not found".to_string(),
            "Default message doesn't match"
        )
    }

    #[test]
    fn test_custom_message() {
        let cust_msg = "Custom error message".to_string();

        let db_err = AppError {
            message: Some(cust_msg.clone()),
            cause: None,
            error_type: AppErrorType::DbError,
        };

        assert_eq!(db_err.message(), cust_msg, "Custom message doesn't match")
    }
}
