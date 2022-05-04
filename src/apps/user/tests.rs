#[cfg(test)]
mod user_tests {
    use crate::apps::u_controllers;
    use crate::apps::u_models::User;

    use crate::config::ToDoConfig;
    use crate::AppState;
    use actix_web::{http::header::ContentType, test, web, web::Data, App};
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
    async fn user_work() {
        let app = test::init_service(
            App::new().app_data(Data::new(APP_STATE.clone())).service(
                web::scope("/user")
                    .service(u_controllers::register)
                    .service(u_controllers::information)
                    .service(u_controllers::login),
            ),
        )
        .await;

        let content = json!({"name": "Test User", "password": "test", "email": "test@gmail.com"});

        let req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .set_payload(content.to_string())
            .uri("/user/register")
            .to_request();

        let res = test::call_service(&app, req).await;

        assert!(
            res.status().is_success(),
            "Response of creating user doesn't successful"
        );

        let token = res
            .headers()
            .get("Authorization")
            .unwrap()
            .to_str()
            .unwrap()
            .split(" ")
            .collect::<Vec<&str>>()[1];

        let req = test::TestRequest::get()
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .uri("/user/information")
            .to_request();
        let res = test::call_service(&app, req).await;

        let serv_user: Result<User, serde_json::Error> =
            serde_json::from_slice(&test::read_body(res).await);

        assert!(serv_user.is_ok(), "Body of getting user doesn't successful");

        let serv_user = serv_user.unwrap();

        assert_eq!(serv_user.name, "Test User", "User doesn't match");

        let login_content = json!({"name": "Test User", "plain_password": "test"});
        let req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .set_payload(login_content.to_string())
            .uri("/user/login")
            .to_request();

        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success(), "User login doesn't work");

        let serv_user: Result<User, serde_json::Error> =
            serde_json::from_slice(&test::read_body(res).await);

        assert!(serv_user.is_ok(), "Body of login user doesn't successful ");

        let serv_user = serv_user.unwrap();

        assert_eq!(serv_user.name, "Test User", "User doesn't match");
    }
}
