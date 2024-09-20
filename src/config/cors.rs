use rocket_cors::{CorsOptions, AllowedOrigins};

pub fn cors_configuration() -> rocket_cors::Cors {
    CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .to_cors()
        .expect("Error while building CORS")
}
