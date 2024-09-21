use mongodb::{Client, Database};
use std::env;
use log::{info, error};
use crate::errors::app_error::AppError;

pub async fn mongo_connect() -> Result<Database, AppError> {
    info!("Attempting to connect to MongoDB");
    let mongo_uri = env::var("MONGODB_URI")
        .map_err(|e| {
            error!("Failed to read MONGODB_URI from environment: {}", e);
            AppError::DatabaseError(format!("MongoDB URI not set: {}", e))
        })?;

    info!("Establishing connection to MongoDB");
    let client = Client::with_uri_str(&mongo_uri).await
        .map_err(|e| {
            error!("Failed to create MongoDB client: {}", e);
            AppError::DatabaseError(format!("MongoDB connection error: {}", e))
        })?;

    info!("Connection to MongoDB established successfully");
    let database = client.database("mydatabase");
    info!("MongoDB database 'mydatabase' selected");

    Ok(database)
}