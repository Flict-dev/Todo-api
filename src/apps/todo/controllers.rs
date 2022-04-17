use crate::apps::new_todo::{SchemaTodo, Status};
use crate::apps::td_logic;
use crate::apps::td_model::TodoList;

use crate::errors::AppError;
use crate::utils::*;
use crate::AppState;
use actix_web::{get, post, web, HttpResponse, Responder};
use slog::o;

#[get("/")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: String::from("Ok"),
    })
}

#[get("/todos{_:/?}")]
pub async fn get_todos(state: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let log = state.logger.new(o!("handler" => "get_todos"));

    let conn = get_db_conn(&state.pool, &state.logger)?;

    let result: Result<Vec<TodoList>, AppError> = td_logic::get_todos(&conn);

    result
        .map(|todos| HttpResponse::Ok().json(todos))
        .map_err(log_error(log))
}

#[post("/todos{_:/?}")]
pub async fn create_todo(
    state: web::Data<AppState>,
    list: web::Json<SchemaTodo>,
) -> Result<impl Responder, AppError> {
    let log = state.logger.new(o!("handler" => "create_list"));

    let conn = get_db_conn(&state.pool, &state.logger)?;

    let result: Result<TodoList, AppError> = td_logic::create_todo(&conn, &list.title);

    result
        .map(|list| HttpResponse::Ok().json(list))
        .map_err(log_error(log))
}
