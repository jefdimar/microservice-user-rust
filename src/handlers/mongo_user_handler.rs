use rocket::serde::json::Json;
use rocket::{ State, http::Status };
use mongodb::Database;
use rocket_okapi::openapi;
use log::{info, error};

use crate::models::user::UserMongo;
use crate::services::user_service;
use crate::errors::app_error::AppError;

#[openapi]
#[post("/v2/users", data = "<user>")]
pub async fn adding_user(db: &State<Database>, user: Json<UserMongo>) -> Result<Json<UserMongo>, AppError> {
    info!("Adding new user: {:?}", user);
    match user_service::add_user(db, user.into_inner()).await {
        Ok(added_user) => {
            info!("User added successfully: {:?}", added_user);
            Ok(Json(added_user))
        }
        Err(e) => {
            error!("Failed to add user: {:?}", e);
            Err(e)
        }
    }
}

#[openapi]
#[get("/v2/users")]
pub async fn getting_users(db: &State<Database>) -> Result<Json<Vec<UserMongo>>, AppError> {
    info!("Fetching all users");
    match user_service::get_users(db).await {
        Ok(users) => {
            info!("Successfully fetched {} users", users.len());
            Ok(Json(users))
        }
        Err(e) => {
            error!("Failed to fetch users: {:?}", e);
            Err(e)
        }
    }
}

#[openapi]
#[put("/v2/users/<id>", data = "<user>")]
pub async fn updating_user(db: &State<Database>, id: String, user: Json<UserMongo>) -> Result<Json<UserMongo>, AppError> {
    info!("Updating user with id: {}", id);
    match user_service::update_user(db, id, user.into_inner()).await {
        Ok(updated_user) => {
            info!("User updated successfully: {:?}", updated_user);
            Ok(Json(updated_user))
        }
        Err(e) => {
            error!("Failed to update user: {:?}", e);
            Err(e)
        }
    }
}

#[openapi]
#[delete("/v2/users/<id>")]
pub async fn deleting_user(db: &State<Database>, id: String) -> Result<Status, AppError> {
    info!("Deleting user with id: {}", id);
    match user_service::delete_user(db, id).await {
        Ok(_) => {
            info!("User deleted successfully");
            Ok(Status::Ok)
        }
        Err(e) => {
            error!("Failed to delete user: {:?}", e);
            Err(e)
        }
    }
}