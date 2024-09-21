use rocket::response::status::Custom;
use rocket::http::Status;
use tokio_postgres::{Client, NoTls};
use rocket::State;

use crate::models::user::User;

pub async fn get_users_from_db(client: &DbClient) -> Result<Vec<User>, Custom<String>> {
    let users = client
        .query("SELECT id, name, email FROM users", &[]).await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?
        .iter()
        .map(|row| User { id: Some(row.get(0)), name: row.get(1), email: row.get(2) })
        .collect::<Vec<User>>();

    Ok(users)
}

pub async fn execute_query(
    client: &DbClient,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)]
) -> Result<u64, Custom<String>> {
    client
        .execute(query, params).await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))
}


pub struct PostgresConfig {
    pub connection_string: String,
}

impl PostgresConfig {
    pub async fn connect(&self) -> Result<Client, tokio_postgres::Error> {
        let (client, connection) = tokio_postgres::connect(&self.connection_string, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Failed to connect to Postgres: {}", e);
            }
        });

        Ok(client)
    }
}

pub type DbClient = State<Client>;