// use crate::apps::new_user::{SchemaNewUser, SchemaUser};
// use crate::apps::u_logic;
// use crate::apps::u_models::User;

// use crate::apps::user::crypto;

// use crate::errors::AppError;
// use crate::utils::*;
// use crate::AppState;
// use actix_web::{get, post, web, HttpResponse, Responder};
// use slog::o;

// // let user_sec =
// #[post("/users/login")]
// pub async fn login(
//     state: web::Data<AppState>,
//     data: web::Json<SchemaUser>,
// ) -> Result<impl Responder, AppError> {
//     let log = state.logger.new(o!("handler" => "user login"));

//     let conn = get_db_conn(&state.pool, &state.logger)?;

//     let result = u_logic::get_user_by_name(&conn, data.name);

//     // if user.password ==

//     HttpResponse::Ok().json(user)
// }
