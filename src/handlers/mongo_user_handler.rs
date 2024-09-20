use rocket::serde::json::Json;
use rocket::{ State, response::status::Custom, http::Status };
use mongodb::Database;
use rocket_okapi::openapi;

use crate::models::user::User;
use crate::services::user_service;
#[openapi]
#[post("/v2/users", data = "<user>")]
pub async fn adding_user(db: &State<Database>, user: Json<User>) -> Result<Json<User>, Custom<String>> {
    user_service::add_user(db, user.into_inner()).await
        .map(Json)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))
}

#[openapi]
#[get("/v2/users")]
pub async fn getting_users(db: &State<Database>) -> Result<Json<Vec<User>>, Custom<String>> {
    user_service::get_users(db).await
        .map(Json)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))
}

#[openapi]
#[put("/v2/users/<id>", data = "<user>")]
pub async fn updating_user(db: &State<Database>, id: String, user: Json<User>) -> Result<Json<User>, Custom<String>> {
    user_service::update_user(db, id, user.into_inner()).await
        .map(Json)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))
}

#[openapi]
#[delete("/v2/users/<id>")]
pub async fn deleting_user(db: &State<Database>, id: String) -> Result<Status, Custom<String>> {
    user_service::delete_user(db, id).await
        .map(|_| Status::Ok)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))
}