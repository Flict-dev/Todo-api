use crate::apps::u_models::{NewUser, User};
use crate::errors::AppError;
use crate::schema::users::dsl::*;
use crate::Connection;
use diesel::prelude::*;

pub fn get_user(conn: &Connection, user_id: i32) -> Result<User, AppError> {
    let user = users
        .find(user_id)
        .get_result::<User>(conn)
        .map_err(AppError::db_error)?;

    Ok(user)
}

pub fn create_user<'a>(
    conn: &Connection,
    user_name: &'a str,
    user_password: &'a str,
    user_email: &'a str,
) -> Result<User, AppError> {
    let new_user = NewUser {
        name: user_name,
        password: user_password,
        email: user_email,
    };

    let new_user = diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
        .map_err(AppError::db_error)?;

    Ok(new_user)
}

pub fn update_user<'a>(
    conn: &Connection,
    user_id: i32,
    user_email: &'a str,
) -> Result<User, AppError> {
    let up_user = diesel::update(users)
        .filter(id.eq(user_id))
        .set(email.eq(user_email))
        .get_result(conn)
        .map_err(AppError::db_error)?;

    Ok(up_user)
}
