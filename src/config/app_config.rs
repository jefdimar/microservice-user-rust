use dotenv::dotenv;
use crate::db::postgres::PostgresConfig;
use crate::db::mongo::mongo_connect;
use mongodb::Database as MongoDatabase;
use tokio_postgres::Client as PostgresClient;

pub struct AppConfig {
    pub postgres_client: PostgresClient,
    pub mongo_db: MongoDatabase,
}

impl AppConfig {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        let postgres_config = PostgresConfig {
            connection_string: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "host=localhost user=postgres password=postgres dbname=project_rocket".to_string()),
        };
        
        let postgres_client = postgres_config.connect().await?;
        let mongo_db = mongo_connect().await?;

        postgres_client.execute("CREATE TABLE IF NOT EXISTS users (id SERIAL PRIMARY KEY, name TEXT, email TEXT)", &[]).await?;

        Ok(AppConfig {
            postgres_client,
            mongo_db,
        })
    }
}
