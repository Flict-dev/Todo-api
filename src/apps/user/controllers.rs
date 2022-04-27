use crate::apps::new_user::{SchemaNewUser, SchemaUser};
use crate::apps::u_logic;
use crate::apps::u_schema::response::UserResponse;

use crate::middlewares::User;

use super::crypto::{Encode, Validate};
use crate::errors::AppError;
use crate::utils::*;
use crate::AppState;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use slog::{info, o};

#[post("/users/login")]
pub async fn login(
    state: web::Data<AppState>,
    data: web::Json<SchemaUser>,
) -> Result<impl Responder, AppError> {
    let log = state.logger.new(o!("handler" => "user login"));

    let conn = get_db_conn(&state.pool, &state.logger)?;

    let user = u_logic::get_user_by_name(&conn, &data.name).map_err(log_error(log))?;

    if state
        .crypto
        .validate_password(data.plain_password.clone(), user.password.clone())
    {
        let token = state.crypto.encode_jwt(user.id).unwrap();
        Ok(HttpResponse::Ok().json(UserResponse { user, token }))
    } else {
        Err(AppError::unauthorized("Invalid password or name"))
    }
}

#[post("/users/register")]
pub async fn register(
    state: web::Data<AppState>,
    data: web::Json<SchemaNewUser>,
) -> Result<impl Responder, AppError> {
    let log = state.logger.new(o!("handler" => "user register"));

    let conn = get_db_conn(&state.pool, &state.logger)?;

    let hash = state.crypto.hash_password(data.password.clone());

    let user =
        u_logic::create_user(&conn, &data.name, &hash, &data.email).map_err(log_error(log))?;

    let token = state.crypto.encode_jwt(user.id).unwrap(); // create error handler!

    Ok(HttpResponse::Ok().json(UserResponse { user, token }))
}

#[get("/users/information")]
pub async fn information(
    _req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let log = state.logger.new(o!("handler" => "user information"));

    let conn = get_db_conn(&state.pool, &state.logger)?;

    let auth = _req.headers().get("Authorization");
    match auth {
        Some(auth) => {
            let token = auth.to_str().unwrap().split(" ").collect::<Vec<&str>>()[1].to_string();
            match state.crypto.validate_jwt(token.to_string()) {
                Ok(token) => {
                    if let Some(claims) = token {
                        let user = u_logic::get_user_by_id(&conn, claims.user_id)
                            .map_err(log_error(log))?;
                        return Ok(HttpResponse::Ok().json(user));
                    } else {
                        return Err(AppError::unauthorized("Outdated token"));
                    }
                }
                Err(_) => Err(AppError::unauthorized("Invalid token")),
            }
        }
        None => Err(AppError::unauthorized("There is no authorization header")),
    }
}

#[get("/users/test")]
pub async fn test(state: web::Data<AppState>, user: User) -> Result<impl Responder, AppError> {
    let log = state.logger.new(o!("handler" => "test middleware"));
    info!(log, "Test user - {} !!", user.user_id);
    Ok(HttpResponse::Ok())
}
