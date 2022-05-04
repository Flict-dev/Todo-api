use crate::apps::new_todo::SchemaTodo;
use crate::apps::td_logic;
use crate::apps::td_models::TodoList;
use crate::middlewares::User;

use crate::errors::AppError;
use crate::utils::*;
use crate::AppState;
use actix_web::HttpResponse;
use paperclip::actix::{
    api_v2_operation, get, post,
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
    list: web::Json<SchemaTodo>,
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

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(get_todo);
    cfg.service(create_todo);
}
