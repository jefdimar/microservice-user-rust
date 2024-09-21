use rocket::serde::json::Json;
use rocket::{ State, http::Status };
use mongodb::Database;
use rocket_okapi::openapi;

use crate::models::user::User;
use crate::services::user_service;
use crate::errors::app_error::AppError;

#[openapi]
#[post("/v2/users", data = "<user>")]
pub async fn adding_user(db: &State<Database>, user: Json<User>) -> Result<Json<User>, AppError> {
    user_service::add_user(db, user.into_inner()).await.map(Json)
}

#[openapi]
#[get("/v2/users")]
pub async fn getting_users(db: &State<Database>) -> Result<Json<Vec<User>>, AppError> {
    user_service::get_users(db).await.map(Json)
}

#[openapi]
#[put("/v2/users/<id>", data = "<user>")]
pub async fn updating_user(db: &State<Database>, id: String, user: Json<User>) -> Result<Json<User>, AppError> {
    user_service::update_user(db, id, user.into_inner()).await.map(Json)
}

#[openapi]
#[delete("/v2/users/<id>")]
pub async fn deleting_user(db: &State<Database>, id: String) -> Result<Status, AppError> {
    user_service::delete_user(db, id).await.map(|_| Status::Ok)
}