use crate::apps::new_todo::{SchemaNewTodo, SchemaTodo};
use crate::apps::td_logic;
use crate::apps::td_models::TodoList;
use crate::middlewares::User;

use crate::errors::AppError;
use crate::utils::*;
use crate::AppState;
use actix_web::HttpResponse;
use paperclip::actix::{
    api_v2_operation, delete, get, post,
    web::{self, ServiceConfig},
};
use slog::o;

#[api_v2_operation]
#[get("/")]
pub async fn get_todo(state: web::Data<AppState>, user: User) -> Result<HttpResponse, AppError> {
    let log = state.logger.new(o!("handler" => "get_todos"));

    let conn = get_db_conn(&state.pool, &state.logger)?;

    let result: Result<Vec<TodoList>, AppError> = td_logic::get_todos(&conn, user.user_id);

    result
        .map(|todos| HttpResponse::Ok().json(todos))
        .map_err(log_error(log))
}

#[api_v2_operation]
#[post("/")]
pub async fn create_todo(
    state: web::Data<AppState>,
    list: web::Json<SchemaNewTodo>,
    user: User,
) -> Result<HttpResponse, AppError> {
    let log = state.logger.new(o!("handler" => "create_list"));

    let conn = get_db_conn(&state.pool, &state.logger)?;

    let result: Result<TodoList, AppError> =
        td_logic::create_todo(&conn, &list.title, user.user_id);

    result
        .map(|list| HttpResponse::Ok().json(list))
        .map_err(log_error(log))
}

#[api_v2_operation]
#[delete("/")]
pub async fn delete_todo(
    state: web::Data<AppState>,
    data: web::Json<SchemaTodo>,
    _user: User,
) -> Result<HttpResponse, AppError> {
    let log = state.logger.new(o!("handler" => "delete todo"));

    let conn = get_db_conn(&state.pool, &state.logger).map_err(log_error(log))?;

    let result = td_logic::delete_todo(&conn, data.todo_id).map_err(AppError::db_not_found)?;

    Ok(HttpResponse::Ok().json(result))
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(get_todo);
    cfg.service(create_todo);
    cfg.service(delete_todo);
}
