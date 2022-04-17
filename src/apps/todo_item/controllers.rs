use crate::apps::new_item::{SchemaTodoItem, ResultResponse, TodoItem};
use crate::apps::ti_logic;
use crate::errors::AppError;

use crate::utils::*;
use crate::AppState;
use actix_web::{get, post, put, web, HttpResponse, Responder};
use slog::o;

#[get("/todos/{list_id}/items{_:/?}")]
pub async fn get_items(
    state: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> Result<impl Responder, AppError> {
    let log = state.logger.new(o!("handler" => "get_items"));

    let conn = get_db_conn(&state.pool, &state.logger)?;

    let result = ti_logic::get_items(&conn, path.0);

    result
        .map(|items| HttpResponse::Ok().json(items))
        .map_err(log_error(log))
}

#[post("/todos/{list_id}/items{_:/?}")]
pub async fn create_item(
    state: web::Data<AppState>,
    data: web::Json<SchemaTodoItem>,
) -> Result<impl Responder, AppError> {
    let log = state.logger.new(o!("handler" => "create_item"));

    let conn = get_db_conn(&state.pool, &state.logger)?;

    let result = ti_logic::create_item(&conn, data.list_id, &data.title);

    result
        .map(|item| HttpResponse::Ok().json(item))
        .map_err(log_error(log))
}

#[put("/todos/{list_id}/items{_:/?}")]
pub async fn check_todo_item(
    state: web::Data<AppState>,
    data: web::Json<TodoItem>,
) -> Result<impl Responder, AppError> {
    let log = state.logger.new(o!("handler" => "check_todo"));

    let conn = get_db_conn(&state.pool, &state.logger)?;

    let result = ti_logic::check_todo_item(&conn, data.id, data.list_id);

    result
        .map(|res| HttpResponse::Ok().json(ResultResponse { success: res }))
        .map_err(log_error(log))
}
