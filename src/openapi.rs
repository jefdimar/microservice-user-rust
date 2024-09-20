use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::SwaggerUIConfig;
use rocket::Route;

use crate::handlers::{user_handler, mongo_user_handler};

pub fn openapi_routes() -> Vec<Route> {
    openapi_get_routes![
        user_handler::add_user,
        user_handler::get_users,
        user_handler::update_user,
        user_handler::delete_user,
        mongo_user_handler::adding_user,
        mongo_user_handler::getting_users,
        mongo_user_handler::updating_user,
        mongo_user_handler::deleting_user
    ]
}

pub fn swagger_ui() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/openapi.json".to_string(),
        ..Default::default()
    }
}