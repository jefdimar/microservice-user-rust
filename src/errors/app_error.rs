use thiserror::Error;
use mongodb::error::Error as MongoError;
use tokio_postgres::Error as PostgresError;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket_okapi::response::OpenApiResponderInner;
use rocket::serde::json::Json;
use serde_json::json;
use schemars::JsonSchema;
use rocket::response::status::Custom;
use log::{error, warn};

#[derive(Error, Debug, JsonSchema)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl OpenApiResponderInner for AppError {
    fn responses(gen: &mut rocket_okapi::gen::OpenApiGenerator) -> rocket_okapi::Result<rocket_okapi::okapi::openapi3::Responses> {
        use rocket_okapi::okapi::openapi3::{Response, RefOr};

        let mut responses = rocket_okapi::okapi::openapi3::Responses::default();
        let schema = gen.json_schema::<AppError>();
        let response = Response {
            description: "Error".to_string(),
            content: rocket_okapi::okapi::map! {
                "application/json".to_string() => rocket_okapi::okapi::openapi3::MediaType {
                    schema: Some(schema),
                    ..Default::default()
                }
            },
            ..Default::default()
        };

        responses.responses.insert("4XX".to_string(), RefOr::Object(response.clone()));
        responses.responses.insert("5XX".to_string(), RefOr::Object(response));

        Ok(responses)
    }
}

impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let (status, message) = match self {
            AppError::DatabaseError(msg) => {
                error!("Database error: {}", msg);
                (Status::InternalServerError, msg)
            },
            AppError::NotFound(msg) => {
                warn!("Not found: {}", msg);
                (Status::NotFound, msg)
            },
            AppError::BadRequest(msg) => {
                warn!("Bad request: {}", msg);
                (Status::BadRequest, msg)
            },
            AppError::Unauthorized(msg) => {
                warn!("Unauthorized: {}", msg);
                (Status::Unauthorized, msg)
            },
            AppError::Forbidden(msg) => {
                warn!("Forbidden: {}", msg);
                (Status::Forbidden, msg)
            },
            AppError::InternalServerError(msg) => {
                error!("Internal server error: {}", msg);
                (Status::InternalServerError, msg)
            },
        };

        let body = Json(json!({
            "error": status.to_string(),
            "message": message
        }));

        Response::build()
            .status(status)
            .header(rocket::http::ContentType::JSON)
            .sized_body(body.to_string().len(), std::io::Cursor::new(body.to_string()))
            .ok()
    }
}

impl From<MongoError> for AppError {
    fn from(error: MongoError) -> Self {
        error!("MongoDB error: {}", error);
        AppError::DatabaseError(error.to_string())
    }
}

impl From<PostgresError> for AppError {
    fn from(error: PostgresError) -> Self {
        error!("PostgreSQL error: {}", error);
        AppError::DatabaseError(error.to_string())
    }
}

impl From<Custom<String>> for AppError {
    fn from(custom: Custom<String>) -> Self {
        match custom.1.as_str() {
            "Not Found" => {
                warn!("Not Found: {}", custom.1);
                AppError::NotFound(custom.1)
            },
            "Bad Request" => {
                warn!("Bad Request: {}", custom.1);
                AppError::BadRequest(custom.1)
            },
            "Unauthorized" => {
                warn!("Unauthorized: {}", custom.1);
                AppError::Unauthorized(custom.1)
            },
            "Forbidden" => {
                warn!("Forbidden: {}", custom.1);
                AppError::Forbidden(custom.1)
            },
            _ => {
                error!("Internal Server Error: {}", custom.1);
                AppError::InternalServerError(custom.1)
            },
        }
    }
}
