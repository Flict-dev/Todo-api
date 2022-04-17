#[cfg(test)]
mod todo_item_tests {
    use crate::apps::td_controllers;
    use crate::apps::td_model::TodoList;
    use crate::apps::ti_controllers;
    use crate::apps::ti_model::TodoItem;

    use crate::config::ToDoConfig;
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
    async fn get_item() {
        let app = test::init_service(
            App::new()
                .app_data(Data::new(APP_STATE.clone()))
                .service(ti_controllers::get_items)
                .service(td_controllers::create_todo),
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
                .service(ti_controllers::get_items)
                .service(td_controllers::create_todo)
                .service(ti_controllers::create_item),
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
    #[actix_web::test]
    async fn update_item() {
        let app = test::init_service(
            App::new()
                .app_data(Data::new(APP_STATE.clone()))
                .service(ti_controllers::get_items)
                .service(td_controllers::create_todo)
                .service(ti_controllers::create_item)
                .service(ti_controllers::check_todo_item),
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
        );
        let content = json!({"list_id": created_item.list_id, "id": created_item.id});

        let req = test::TestRequest::put()
            .uri(&format!("/todos/{}/items", created_list.id))
            .insert_header(ContentType::json())
            .set_payload(content.to_string())
            .to_request();

        let res = test::call_service(&app, req).await;
        assert!(
            res.status().is_success(),
            "Fail to update item"
        )
    }
}
