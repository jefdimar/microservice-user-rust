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
            AppError::DatabaseError(msg) => (Status::InternalServerError, msg),
            AppError::NotFound(msg) => (Status::NotFound, msg),
            AppError::BadRequest(msg) => (Status::BadRequest, msg),
            AppError::Unauthorized(msg) => (Status::Unauthorized, msg),
            AppError::Forbidden(msg) => (Status::Forbidden, msg),
            AppError::InternalServerError(msg) => (Status::InternalServerError, msg),
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
        AppError::DatabaseError(error.to_string())
    }
}

impl From<PostgresError> for AppError {
    fn from(error: PostgresError) -> Self {
        AppError::DatabaseError(error.to_string())
    }
}

impl From<Custom<String>> for AppError {
    fn from(custom: Custom<String>) -> Self {
        match custom.1.as_str() {
            "Not Found" => AppError::NotFound(custom.1),
            "Bad Request" => AppError::BadRequest(custom.1),
            "Unauthorized" => AppError::Unauthorized(custom.1),
            "Forbidden" => AppError::Forbidden(custom.1),
            _ => AppError::InternalServerError(custom.1),
        }
    }
}
