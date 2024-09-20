use tokio_postgres::{Client, NoTls};
use rocket::State;

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
