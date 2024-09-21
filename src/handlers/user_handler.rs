use rocket::serde::json::Json;
use rocket::http::Status;
use rocket_okapi::openapi;

use crate::models::user::User;
use crate::db::postgres::{get_users_from_db, execute_query};
use crate::db::postgres::DbClient;
use crate::errors::app_error::AppError;

#[openapi]
#[post("/users", data = "<user>")]
pub async fn add_user(
    conn: &DbClient,
    user: Json<User>
) -> Result<Json<Vec<User>>, AppError> {
    execute_query(
        conn,
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        &[&user.name, &user.email]
    ).await?;
    get_users(conn).await
}
#[openapi]
#[get("/users")]
pub async fn get_users(conn: &DbClient) -> Result<Json<Vec<User>>, AppError> {
    get_users_from_db(conn).await.map(Json).map_err(AppError::from)
}
#[openapi]
#[put("/users/<id>", data = "<user>")]
pub async fn update_user(
    conn: &DbClient,
    id: i32,
    user: Json<User>
) -> Result<Json<Vec<User>>, AppError> {
    execute_query(
        conn,
        "UPDATE users SET name = $1, email = $2 WHERE id = $3",
        &[&user.name, &user.email, &id]
    ).await?;
    get_users(conn).await
}

#[openapi]
#[delete("/users/<id>")]
pub async fn delete_user(conn: &DbClient, id: i32) -> Result<Status, AppError> {
    execute_query(conn, "DELETE FROM users WHERE id = $1", &[&id]).await?;
    Ok(Status::NoContent)
}