use std::{thread, time::Duration};
use dotenv::dotenv;
use crate::db::postgres::create_postgres_client;
use crate::db::mongo::mongo_connect;
use mongodb::Database as MongoDatabase;
use tokio_postgres::Client as PostgresClient;
use log::{info, warn, error};

pub struct AppConfig {
    pub postgres_client: PostgresClient,
    pub mongo_db: MongoDatabase,
}

impl AppConfig {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        info!("Initializing AppConfig");
        dotenv().ok();

        let mut retries = 5;
        let mut postgres_client = None;

        while retries > 0 {
            info!("Attempting to connect to PostgreSQL database (Attempt {})", 6 - retries);
            match create_postgres_client().await {
                Ok(client) => {
                    info!("Successfully connected to PostgreSQL database");
                    postgres_client = Some(client);
                    break;
                }
                Err(e) => {
                    warn!("Failed to connect to PostgreSQL. Retrying in 5 seconds... (Attempts left: {})", retries);
                    warn!("Error: {}", e);
                    thread::sleep(Duration::from_secs(5));
                    retries -= 1;
                }
            }
        }

        let postgres_client = postgres_client.ok_or_else(|| {
            error!("Failed to connect to PostgreSQL after multiple attempts");
            "Failed to connect to PostgreSQL after multiple attempts"
        })?;

        info!("Connecting to MongoDB");
        let mongo_db = mongo_connect().await?;
        info!("Successfully connected to MongoDB");

        info!("Creating users table if not exists");
        postgres_client.execute("CREATE TABLE IF NOT EXISTS users (id SERIAL PRIMARY KEY, name TEXT, email TEXT)", &[]).await?;
        info!("Users table created or already exists");

        info!("AppConfig initialization completed successfully");
        Ok(AppConfig {
            postgres_client,
            mongo_db,
        })
    }
}