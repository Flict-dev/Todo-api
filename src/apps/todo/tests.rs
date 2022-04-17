#[cfg(test)]
mod todo_tests {

    use crate::apps::td_controllers;
    use crate::apps::td_model::TodoList;

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
    async fn get_todo() {
        let app = test::init_service(
            App::new()
                .app_data(Data::new(APP_STATE.clone()))
                .service(td_controllers::get_todos),
        )
        .await;

        let req = test::TestRequest::get().uri("/todos").to_request();

        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn create_todo() {
        let app = test::init_service(
            App::new()
                .app_data(Data::new(APP_STATE.clone()))
                .service(td_controllers::get_todos)
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

        let req = test::TestRequest::get().uri("/todos").to_request();
        let res = test::call_service(&app, req).await;

        let todos: Vec<TodoList> = test::read_body_json(res).await;

        let maybe_todo = todos.iter().find(|todo| todo.id == created_list.id);

        assert!(
            maybe_todo.is_some(),
            "Db of creating todo list doesn't successful"
        )
    }
}
