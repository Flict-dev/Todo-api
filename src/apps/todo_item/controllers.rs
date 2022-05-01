use crate::apps::new_item::{ResultResponse, SchemaTodoItem, TodoItem};
use crate::apps::ti_logic;
use crate::errors::AppError;
use crate::middlewares::User;

use crate::utils::*;
use crate::AppState;
use actix_web::{get, post, put, web, HttpResponse, Responder};
use slog::o;

#[get("/todos/{list_id}/items{_:/?}")]
pub async fn get_items(
    state: web::Data<AppState>,
    path: web::Path<(i32,)>,
    _user: User,
) -> Result<impl Responder, AppError> {
    let log = state.logger.new(o!("handler" => "get_items"));

    let conn = get_db_conn(&state.pool, &state.logger).map_err(log_error(log))?;

    let result = ti_logic::get_items(&conn, path.0);

    result
        .map(|items| HttpResponse::Ok().json(items))
        .map_err(AppError::db_not_found)
}

#[post("/todos/{list_id}/items{_:/?}")]
pub async fn create_item(
    state: web::Data<AppState>,
    path: web::Path<(i32,)>,
    data: web::Json<SchemaTodoItem>,
    _user: User,
) -> Result<impl Responder, AppError> {
    let log = state.logger.new(o!("handler" => "create_item"));

    let conn = get_db_conn(&state.pool, &state.logger).map_err(log_error(log))?;

    let result = ti_logic::create_item(&conn, path.0, &data.title);

    result
        .map(|item| HttpResponse::Ok().json(item))
        .map_err(AppError::db_not_found)
}

#[put("/todos/{list_id}/items{_:/?}")]
pub async fn check_todo_item(
    state: web::Data<AppState>,
    path: web::Path<(i32,)>,
    data: web::Json<TodoItem>,
    _user: User,
) -> Result<impl Responder, AppError> {
    let log = state.logger.new(o!("handler" => "check_todo"));

    let conn = get_db_conn(&state.pool, &state.logger).map_err(log_error(log))?;

    let result = ti_logic::check_todo_item(&conn, data.id, path.0);

    result
        .map(|res| HttpResponse::Ok().json(ResultResponse { success: res }))
        .map_err(AppError::db_not_found)
}
