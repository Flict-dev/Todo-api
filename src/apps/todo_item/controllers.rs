use crate::apps::new_item::{ResultResponse, SchemaTodoItem, TodoItem};
use crate::apps::ti_logic;
use crate::errors::AppError;
use crate::middlewares::User;

use crate::utils::*;
use crate::AppState;
use actix_web::HttpResponse;
use paperclip::actix::{
    api_v2_operation, delete, get, post, put,
    web::{self, ServiceConfig},
};
use slog::o;

#[api_v2_operation]
#[get("/{list_id}/items")]
pub async fn get_item(
    state: web::Data<AppState>,
    path: web::Path<(i32,)>,
    _user: User,
) -> Result<HttpResponse, AppError> {
    let log = state.logger.new(o!("handler" => "get_items"));

    let conn = get_db_conn(&state.pool, &state.logger).map_err(log_error(log))?;

    let result = ti_logic::get_items(&conn, path.0);

    result
        .map(|items| HttpResponse::Ok().json(items))
        .map_err(AppError::db_not_found)
}

#[api_v2_operation]
#[post("/{list_id}/items")]
pub async fn create_item(
    state: web::Data<AppState>,
    path: web::Path<(i32,)>,
    data: web::Json<SchemaTodoItem>,
    _user: User,
) -> Result<HttpResponse, AppError> {
    let log = state.logger.new(o!("handler" => "create_item"));

    let conn = get_db_conn(&state.pool, &state.logger).map_err(log_error(log))?;

    let result = ti_logic::create_item(&conn, path.0, &data.title);

    result
        .map(|item| HttpResponse::Ok().json(item))
        .map_err(AppError::db_not_found)
}

#[api_v2_operation]
#[put("/{list_id}/items")]
pub async fn check_todo_item(
    state: web::Data<AppState>,
    path: web::Path<(i32,)>,
    data: web::Json<TodoItem>,
    _user: User,
) -> Result<HttpResponse, AppError> {
    let log = state.logger.new(o!("handler" => "check_todo"));

    let conn = get_db_conn(&state.pool, &state.logger).map_err(log_error(log))?;

    let result = ti_logic::check_todo_item(&conn, data.id, path.0);

    result
        .map(|res| HttpResponse::Ok().json(ResultResponse { success: res }))
        .map_err(AppError::db_not_found)
}

#[api_v2_operation]
#[delete("/{list_id}/items")]
pub async fn delete_item(
    state: web::Data<AppState>,
    path: web::Path<(i32,)>,
    data: web::Json<TodoItem>,
    _user: User,
) -> Result<HttpResponse, AppError> {
    let log = state.logger.new(o!("handler" => "delete item"));

    let conn = get_db_conn(&state.pool, &state.logger).map_err(log_error(log))?;

    let result = ti_logic::delete_item(&conn, data.id, path.0).map_err(AppError::db_not_found)?;
    Ok(HttpResponse::Ok().json(result))
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(get_item);
    cfg.service(create_item);
    cfg.service(check_todo_item);
    cfg.service(delete_item);
}
