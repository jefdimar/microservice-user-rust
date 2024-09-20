pub mod user_handler;
pub mod mongo_user_handler;

use rocket::get;

#[get("/")]
pub fn hello() -> &'static str {
    "Hello, Rocket!"
}
