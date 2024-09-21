#[macro_use]
extern crate rocket;

mod models;
mod handlers;
mod db;
mod config;
mod routes;
mod services;
mod errors;
mod openapi;

use rocket_okapi::swagger_ui::make_swagger_ui;
use routes::user_routes::{user_routes, user_mongo_routes};
use config::{cors::cors_configuration, app_config::AppConfig};
use openapi::swagger_ui::{openapi_routes, swagger_ui};
use handlers::hello;

#[launch]
async fn rocket() -> _ {
    let app_config = AppConfig::new().await.expect("Failed to initialize application config");

    rocket::build()
        .manage(app_config.postgres_client)
        .manage(app_config.mongo_db)
        .mount("/", routes![hello])
        .mount("/postgres", user_routes())
        .mount("/mongo", user_mongo_routes())
        .mount("/", openapi_routes())
        .mount("/doc", make_swagger_ui(&swagger_ui()))
        .attach(cors_configuration())
}