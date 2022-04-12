use crate::errors::AppError;
use crate::req_models::{CreateList, CreateListItem, ListItem, ResultResponse, Status};
use crate::{db, AppState};
use actix_web::{get, post, put, web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
use slog::{crit, error, o, Logger};

async fn get_db_client(poll: &Pool, logger: &Logger) -> Result<Client, AppError> {
    let log = logger.new(o!("handler" => "get_db_client"));

    poll.get().await.map_err(|err| {
        let sublog = log.new(o!("cause" => err.to_string()));
        crit!(sublog, "Error with connection to database");
        AppError::db_error(err.to_string())
    })
}

pub fn log_error(log: Logger) -> Box<dyn Fn(AppError) -> AppError> {
    Box::new(move |error| {
        let sublog = log.new(o!("cause" => error.cause.clone()));
        error!(sublog, "{}", error.message());
        error
    })
}

#[get("/")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: String::from("Ok"),
    })
}

#[get("/todos{_:/?}")]
pub async fn get_todos(state: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let log = state.logger.new(o!("handler" => "get_todos"));

    let client: Client = get_db_client(&state.pool, &state.logger).await?;

    let result = db::get_todos(&client).await;

    result
        .map(|todos| HttpResponse::Ok().json(todos))
        .map_err(log_error(log))
}

#[get("/todos/{list_id}/items{_:/?}")]
pub async fn get_items(state: web::Data<AppState>, path: web::Path<(i32,)>) -> impl Responder {
    let log = state.logger.new(o!("handler" => "get_items"));

    let client: Client = get_db_client(&state.pool, &state.logger).await?;

    let result = db::get_items(&client, path.0).await;

    result
        .map(|items| HttpResponse::Ok().json(items))
        .map_err(log_error(log))
}

#[post("/todos{_:/?}")]
pub async fn create_list(
    state: web::Data<AppState>,
    list: web::Json<CreateList>,
) -> impl Responder {
    let log = state.logger.new(o!("handler" => "create_list"));

    let client: Client = get_db_client(&state.pool, &state.logger).await?;

    let result = db::create_list(&client, list.title.clone()).await;

    result
        .map(|list| HttpResponse::Ok().json(list))
        .map_err(log_error(log))
}

#[post("/todos/{list_id}/items{_:/?}")]
pub async fn create_item(
    state: web::Data<AppState>,
    data: web::Json<CreateListItem>,
) -> impl Responder {
    let log = state.logger.new(o!("handler" => "create_item"));

    let client: Client = get_db_client(&state.pool, &state.logger).await?;

    let result = db::create_item(&client, data.list_id, data.title.clone()).await;

    result
        .map(|item| HttpResponse::Ok().json(item))
        .map_err(log_error(log))
}

#[put("/todos/{list_id}/items{_:/?}")]
pub async fn check_todo(state: web::Data<AppState>, data: web::Json<ListItem>) -> impl Responder {
    let log = state.logger.new(o!("handler" => "check_todo"));

    let client: Client = get_db_client(&state.pool, &state.logger).await?;

    let result = db::check_todo(&client, data.id, data.list_id).await;

    result
        .map(|res| HttpResponse::Ok().json(ResultResponse { success: res }))
        .map_err(log_error(log))
}
