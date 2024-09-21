use rocket::serde::json::Json;
use rocket::http::Status;
use rocket_okapi::openapi;
use log::{info, error};

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
    info!("Adding new user: {:?}", user);
    match execute_query(
        conn,
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        &[&user.name, &user.email]
    ).await {
        Ok(_) => {
            info!("User added successfully");
            get_users(conn).await
        }
        Err(e) => {
            error!("Failed to add user: {:?}", e);
            Err(e.into())
        }
    }
}

#[openapi]
#[get("/users")]
pub async fn get_users(conn: &DbClient) -> Result<Json<Vec<User>>, AppError> {
    info!("Fetching all users");
    match get_users_from_db(conn).await {
        Ok(users) => {
            info!("Successfully fetched {} users", users.len());
            Ok(Json(users))
        }
        Err(e) => {
            error!("Failed to fetch users: {:?}", e);
            Err(e.into())
        }
    }
}

#[openapi]
#[put("/users/<id>", data = "<user>")]
pub async fn update_user(
    conn: &DbClient,
    id: i32,
    user: Json<User>
) -> Result<Json<Vec<User>>, AppError> {
    info!("Updating user with id: {}", id);
    match execute_query(
        conn,
        "UPDATE users SET name = $1, email = $2 WHERE id = $3",
        &[&user.name, &user.email, &id]
    ).await {
        Ok(_) => {
            info!("User updated successfully");
            get_users(conn).await
        }
        Err(e) => {
            error!("Failed to update user: {:?}", e);
            Err(e.into())
        }
    }
}

#[openapi]
#[delete("/users/<id>")]
pub async fn delete_user(conn: &DbClient, id: i32) -> Result<Status, AppError> {
    info!("Deleting user with id: {}", id);
    match execute_query(conn, "DELETE FROM users WHERE id = $1", &[&id]).await {
        Ok(_) => {
            info!("User deleted successfully");
            Ok(Status::NoContent)
        }
        Err(e) => {
            error!("Failed to delete user: {:?}", e);
            Err(e.into())
        }
    }
}