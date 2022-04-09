use crate::db;
use crate::req_models::{CreateList, CreateListItem, ListItem, ResultResponse, Status};
use actix_web::{web, Responder};
use deadpool_postgres::{Client, Pool};
use std::io::ErrorKind::Other;

pub async fn status() -> impl Responder {
    actix_web::HttpResponse::Ok().json(Status {
        status: String::from("Ok"),
    })
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");

    let result = db::get_todos(&client).await;

    match result {
        Ok(todos) => actix_web::HttpResponse::Ok().json(todos),
        Err(_) => actix_web::HttpResponse::InternalServerError().into(),
    }
}

pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");

    let result = db::get_items(&client, path.0).await;

    match result {
        Ok(items) => actix_web::HttpResponse::Ok().json(items),
        Err(_) => actix_web::HttpResponse::InternalServerError().into(),
    }
}

pub async fn create_list(db_pool: web::Data<Pool>, list: web::Json<CreateList>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");

    let result = db::create_list(&client, list.title.clone()).await;

    match result {
        Ok(list) => actix_web::HttpResponse::Ok().json(list),
        Err(_) => actix_web::HttpResponse::InternalServerError().into(),
    }
}

pub async fn create_item(
    db_pool: web::Data<Pool>,
    data: web::Json<CreateListItem>,
) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");

    let result = db::create_item(&client, data.list_id, data.title.clone()).await;

    match result {
        Ok(item) => actix_web::HttpResponse::Ok().json(item),
        Err(_) => actix_web::HttpResponse::InternalServerError().into(),
    }
}

pub async fn check_todo(db_pool: web::Data<Pool>, data: web::Json<ListItem>) -> impl Responder {
    let client = db_pool.get().await.expect("Error connecting to database");

    let result = db::check_todo(&client, data.id, data.list_id).await;

    match result {
        Ok(()) => actix_web::HttpResponse::Ok().json(ResultResponse { success: true }),
        Err(err) if err.kind() == Other => {
            actix_web::HttpResponse::Ok().json(ResultResponse { success: false })
        }
        Err(_) => actix_web::HttpResponse::InternalServerError().into(),
    }
}
