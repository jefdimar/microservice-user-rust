use rocket::response::status::Custom;
use rocket::http::Status;
use tokio_postgres::{Client, NoTls};
use rocket::State;
use log::{info, error};

use crate::models::user::User;

pub async fn get_users_from_db(client: &DbClient) -> Result<Vec<User>, Custom<String>> {
    info!("Fetching users from PostgreSQL database");
    match client.query("SELECT id, name, email FROM users", &[]).await {
        Ok(rows) => {
            let users = rows
                .iter()
                .map(|row| User { id: Some(row.get(0)), name: row.get(1), email: row.get(2) })
                .collect::<Vec<User>>();
            info!("Successfully fetched {} users from database", users.len());
            Ok(users)
        }
        Err(e) => {
            error!("Failed to fetch users from database: {}", e);
            Err(Custom(Status::InternalServerError, e.to_string()))
        }
    }
}

pub async fn execute_query(
    client: &DbClient,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)]
) -> Result<u64, Custom<String>> {
    info!("Executing PostgreSQL query: {}", query);
    match client.execute(query, params).await {
        Ok(rows_affected) => {
            info!("Query executed successfully. Rows affected: {}", rows_affected);
            Ok(rows_affected)
        }
        Err(e) => {
            error!("Failed to execute query: {}", e);
            Err(Custom(Status::InternalServerError, e.to_string()))
        }
    }
}

pub struct PostgresConfig {
    pub connection_string: String,
}

impl PostgresConfig {
    pub async fn connect(&self) -> Result<Client, tokio_postgres::Error> {
        info!("Attempting to connect to PostgreSQL database");
        let (client, connection) = tokio_postgres::connect(&self.connection_string, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                error!("PostgreSQL connection error: {}", e);
            }
        });

        info!("Successfully connected to PostgreSQL database");
        Ok(client)
    }
}

pub type DbClient = State<Client>;