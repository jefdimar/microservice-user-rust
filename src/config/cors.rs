use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use std::env;

pub fn cors_configuration() -> rocket_cors::Cors {
    let allowed_origins = match env::var("ALLOWED_ORIGINS") {
        Ok(origins) => {
            let origins: Vec<&str> = origins.split(',').collect();
            AllowedOrigins::some_exact(&origins)
        }
        Err(_) => AllowedOrigins::all(),
    };

    let allowed_methods: rocket_cors::AllowedMethods = ["GET", "POST", "PUT", "DELETE"]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();
    
    CorsOptions {
        allowed_origins,
        allowed_methods,
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Content-Type",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors()
        .expect("Error while building CORS")
}