use crate::db;
use crate::models::Status;
use actix_web::{web, Responder};
use deadpool_postgres::{Client, Pool};

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
