use crate::apps::new_user::{SchemaNewUser, SchemaUser};
use crate::apps::u_logic;
use crate::middlewares::User;

use super::crypto::{Encode, Validate};
use crate::errors::AppError;
use crate::utils::*;
use crate::AppState;
use actix_web::{get, post, web, HttpResponse, Responder};
use slog::o;

#[post("/users/login")]
pub async fn login(
    state: web::Data<AppState>,
    data: web::Json<SchemaUser>,
) -> Result<HttpResponse, AppError> {
    let log = state.logger.new(o!("handler" => "user login"));

    let conn = get_db_conn(&state.pool, &state.logger).map_err(log_error(log))?;

    let user = u_logic::get_user_by_name(&conn, &data.name).map_err(AppError::db_not_found)?;
    state
        .crypto
        .validate_password(data.plain_password.clone(), user.password.clone())
        .map_err(AppError::unauthorized)?;

    let token = state.crypto.encode_jwt(user.id).unwrap();
    Ok(HttpResponse::Ok()
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .json(user))
}
#[post("/users/register")]
pub async fn register(
    state: web::Data<AppState>,
    data: web::Json<SchemaNewUser>,
) -> Result<HttpResponse, AppError> {
    let log = state.logger.new(o!("handler" => "user register"));

    let conn = get_db_conn(&state.pool, &state.logger).map_err(log_error(log))?;

    let hash = state.crypto.hash_password(data.password.clone());

    let user =
        u_logic::create_user(&conn, &data.name, &hash, &data.email).map_err(AppError::db_unique)?;

    let token = state.crypto.encode_jwt(user.id).unwrap();

    Ok(HttpResponse::Ok()
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .json(user))
}

#[get("/users/information")]
pub async fn information(
    state: web::Data<AppState>,
    user: User,
) -> Result<impl Responder, AppError> {
    let log = state.logger.new(o!("handler" => "user information"));

    let conn = get_db_conn(&state.pool, &state.logger).map_err(log_error(log))?;

    let user = u_logic::get_user_by_id(&conn, user.user_id).map_err(AppError::db_not_found)?;
    Ok(HttpResponse::Ok().json(user))
}
