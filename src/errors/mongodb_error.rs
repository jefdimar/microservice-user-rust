use thiserror::Error;
use mongodb::error::Error as MongoError;
use rocket::http::Status;
use rocket::response::status::Custom;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum MongoDbError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] MongoError),
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid ID format")]
    InvalidId,
    #[error("Insertion failed")]
    InsertionFailed,
    #[error("Update failed")]
    UpdateFailed,
    #[error("Deletion failed")]
    DeletionFailed,
}

impl MongoDbError {
    #[allow(dead_code)]
    pub fn to_rocket_error(&self) -> Custom<String> {
        match self {
            MongoDbError::DatabaseError(_) => Custom(Status::InternalServerError, self.to_string()),
            MongoDbError::UserNotFound => Custom(Status::NotFound, self.to_string()),
            MongoDbError::InvalidId => Custom(Status::BadRequest, self.to_string()),
            MongoDbError::InsertionFailed => Custom(Status::InternalServerError, self.to_string()),
            MongoDbError::UpdateFailed => Custom(Status::InternalServerError, self.to_string()),
            MongoDbError::DeletionFailed => Custom(Status::InternalServerError, self.to_string()),
        }
    }}
