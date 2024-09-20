use rocket::Route;

use crate::handlers::{user_handler, mongo_user_handler};

pub fn user_routes() -> Vec<Route> {
    routes![
        user_handler::add_user,
        user_handler::get_users,
        user_handler::update_user,
        user_handler::delete_user
    ]
}

pub fn user_mongo_routes() -> Vec<Route> {
    routes![
        mongo_user_handler::adding_user,
        mongo_user_handler::getting_users,
        mongo_user_handler::updating_user,
        mongo_user_handler::deleting_user
    ]
}