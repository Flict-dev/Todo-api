use crate::config::ToDoConfig;
use crate::db_models::TodoItem;
use crate::db_models::TodoList;
use crate::handlers;
use crate::AppState;
use actix_web::web::Data;
use actix_web::{http::header::ContentType, test, App};
use lazy_static::lazy_static;
use serde_json::json;

lazy_static! {
    static ref APP_STATE: AppState = {
        dotenv::dotenv().ok();

        let config = ToDoConfig::new().unwrap();
        config.new_state()
    };
}

#[actix_web::test]
async fn get_todos() {
    let app = test::init_service(
        App::new()
            .app_data(Data::new(APP_STATE.clone()))
            .service(handlers::get_todos),
    )
    .await;

    let req = test::TestRequest::get().uri("/todos").to_request();

    let res = test::call_service(&app, req).await;

    assert!(res.status().is_success());
}

#[actix_web::test]
async fn create_todos() {
    let app = test::init_service(
        App::new()
            .app_data(Data::new(APP_STATE.clone()))
            .service(handlers::get_todos)
            .service(handlers::create_list),
    )
    .await;

    let content = json!({"title": "Test todo"});

    let req = test::TestRequest::post()
        .insert_header(ContentType::json())
        .set_payload(content.to_string())
        .uri("/todos")
        .to_request();

    let res = test::call_service(&app, req).await;

    assert!(
        res.status().is_success(),
        "Response of creating todo list doesn't successful"
    );

    let try_created: Result<TodoList, serde_json::Error> =
        serde_json::from_slice(&test::read_body(res).await);

    assert!(
        try_created.is_ok(),
        "Body of creating todo list doesn't successful"
    );

    let created_list = try_created.unwrap();

    let req = test::TestRequest::get().uri("/todos").to_request();
    let res = test::call_service(&app, req).await;

    let todos: Vec<TodoList> = test::read_body_json(res).await;

    let maybe_todo = todos.iter().find(|todo| todo.id == created_list.id);

    assert!(
        maybe_todo.is_some(),
        "Db of creating todo list doesn't successful"
    )
}

#[actix_web::test]
async fn get_items() {
    let app = test::init_service(
        App::new()
            .app_data(Data::new(APP_STATE.clone()))
            .service(handlers::get_items)
            .service(handlers::create_list),
    )
    .await;

    let content = json!({"title": "Test todo"});

    let req = test::TestRequest::post()
        .insert_header(ContentType::json())
        .set_payload(content.to_string())
        .uri("/todos")
        .to_request();

    let res = test::call_service(&app, req).await;

    assert!(
        res.status().is_success(),
        "Response of creating todo list doesn't successful"
    );

    let try_created: Result<TodoList, serde_json::Error> =
        serde_json::from_slice(&test::read_body(res).await);

    assert!(
        try_created.is_ok(),
        "Body of creating todo list doesn't successful"
    );

    let created_list = try_created.unwrap();

    let req = test::TestRequest::get()
        .uri(&format!("/todos/{}/items", created_list.id))
        .to_request();

    let res = test::call_service(&app, req).await;

    assert!(
        res.status().is_success(),
        "Response of getting todo item doesn't successful"
    );
}

#[actix_web::test]
async fn create_item() {
    let app = test::init_service(
        App::new()
            .app_data(Data::new(APP_STATE.clone()))
            .service(handlers::get_items)
            .service(handlers::create_list)
            .service(handlers::create_item),
    )
    .await;

    let content = json!({"title": "Test todo"});

    let req = test::TestRequest::post()
        .insert_header(ContentType::json())
        .set_payload(content.to_string())
        .uri("/todos")
        .to_request();

    let res = test::call_service(&app, req).await;

    assert!(
        res.status().is_success(),
        "Response of creating todo list doesn't successful"
    );

    let try_created: Result<TodoList, serde_json::Error> =
        serde_json::from_slice(&test::read_body(res).await);

    assert!(
        try_created.is_ok(),
        "Body of creating todo list doesn't successful"
    );

    let created_list = try_created.unwrap();

    let item = json!({"title": "test item", "list_id": created_list.id});

    let req = test::TestRequest::post()
        .uri(&format!("/todos/{}/items", created_list.id))
        .insert_header(ContentType::json())
        .set_payload(item.to_string())
        .to_request();

    let res = test::call_service(&app, req).await;

    assert!(
        res.status().is_success(),
        "Response of creating todo item doesn't successful"
    );

    let try_created: Result<TodoItem, serde_json::Error> =
        serde_json::from_slice(&test::read_body(res).await);

    assert!(
        try_created.is_ok(),
        "Body of creating todo item doesn't successful"
    );

    let req = test::TestRequest::get()
        .uri(&format!("/todos/{}/items", created_list.id))
        .to_request();

    let res = test::call_service(&app, req).await;

    assert!(
        res.status().is_success(),
        "Response of getting todo item doesn't successful"
    );

    let created_item = try_created.unwrap();

    let created_res_item: Vec<TodoItem> = test::read_body_json(res).await;

    let maybe_item = created_res_item
        .iter()
        .find(|item| item.id == created_item.id);

    assert!(
        maybe_item.is_some(),
        "Db of creating todo item doesn't successful"
    )
}
