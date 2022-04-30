use crate::apps::user::crypto::Validate;
use crate::{errors::AppError, AppState};
use actix_web::{dev, web::Data, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};

pub struct User {
    pub user_id: i32,
}

impl FromRequest for User {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let state = req.app_data::<Data<AppState>>().unwrap();
        let auth = req.headers().get("Authorization");
        match auth {
            Some(auth) => {
                let token = auth.to_str().unwrap().split(" ").collect::<Vec<&str>>()[1].to_string();
                match state.crypto.validate_jwt(token.to_string()) {
                    Ok(token) => {
                        if let Some(claims) = token {
                            return ok(User {
                                user_id: claims.user_id,
                            });
                        } else {
                            return err(AppError::unauthorized("Outdated token"));
                        }
                    }
                    Err(_) => err(AppError::unauthorized("Invalid token")),
                }
            }
            None => err(AppError::forbiden("There is no authorization header")),
        }
    }
}
