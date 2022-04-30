use crate::apps::u_models::{NewUser, User};
use crate::errors::AppError;
use crate::schema::users::dsl::*;
use crate::Connection;
use diesel::prelude::*;

pub fn get_user_by_name(conn: &Connection, user_name: &str) -> Result<User, AppError> {
    let user = users
        .filter(name.eq(user_name))
        .get_result::<User>(conn)
        .map_err(AppError::db_not_found)?;

    Ok(user)
}

pub fn get_user_by_id(conn: &Connection, user_id: i32) -> Result<User, AppError> {
    let user = users
        .find(user_id)
        .get_result::<User>(conn)
        .map_err(AppError::db_not_found)?;

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
        todo_id: None,
    };

    let new_user = diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
        .map_err(AppError::db_not_found)?;
    Ok(new_user)
}
