#[macro_use]
extern crate rocket;
#[macro_use]
extern crate log;

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
use env_logger::Env;

#[launch]
async fn rocket() -> _ {
    // Initialize the logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Starting application...");

    let app_config = match AppConfig::new().await {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to initialize application config: {}", e);
            panic!("Application startup failed");
        }
    };

    info!("Application config initialized successfully");

    let rocket_instance = rocket::build()
        .manage(app_config.postgres_client)
        .manage(app_config.mongo_db)
        .mount("/", routes![hello])
        .mount("/postgres", user_routes())
        .mount("/mongo", user_mongo_routes())
        .mount("/", openapi_routes())
        .mount("/doc", make_swagger_ui(&swagger_ui()))
        .attach(cors_configuration());

    info!("Rocket instance configured, ready to launch!");

    rocket_instance
}