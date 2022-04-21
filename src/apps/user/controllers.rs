use crate::apps::new_user::{SchemaNewUser, SchemaUser};
use crate::apps::u_logic;
use crate::apps::u_schema::response::UserResponse;

use crate::errors::AppError;
use crate::utils::*;
use crate::AppState;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use slog::o;

use super::crypto::{Decode, Encode, Validate};

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
        Ok(HttpResponse::Ok().json(user))
    } else {
        Err(AppError::unauthorized())
    }
}

#[post("/users/register")]
pub async fn register(
    state: web::Data<AppState>,
    data: web::Json<SchemaNewUser>,
) -> Result<impl Responder, AppError> {
    let log = state.logger.new(o!("handler" => "user login"));

    let conn = get_db_conn(&state.pool, &state.logger)?;

    let hash = state.crypto.hash_password(data.password.clone()); // use link here

    let user =
        u_logic::create_user(&conn, &data.name, &data.email, &hash).map_err(log_error(log))?;

    let token = state.crypto.encode_jwt(user.id).unwrap(); // create error handler!

    Ok(HttpResponse::Ok().json(UserResponse { user, token }))
}

#[get("/users/information")]
pub async fn information(
    _req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let log = state.logger.new(o!("handler" => "user login"));

    let conn = get_db_conn(&state.pool, &state.logger)?;

    let auth = _req.headers().get("Authorization");
    match auth {
        Some(auth) => {
            let token = auth
                .to_str()
                .unwrap()
                .split("Bearer")
                .collect::<Vec<&str>>()[1]
                .to_string();
            if state.crypto.validate_jwt(token.to_string()) {
                let decoded_token = state.crypto.decode_jwt(token.clone()).unwrap(); // Maybe join this logic decode and validate
                let user = u_logic::get_user_by_id(&conn, decoded_token.user_id)
                    .map_err(log_error(log))?;
                Ok(HttpResponse::Ok().json(UserResponse { user, token }))
            } else {
                Err(AppError::unauthorized())
            }
        }
        None => Err(AppError::unauthorized()),
    }
}
