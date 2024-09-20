use mongodb::{Client, Database};
use mongodb::options::ClientOptions;
use std::env;

pub async fn mongo_connect() -> mongodb::error::Result<Database> {
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let database_name = env::var("MONGO_DATABASE").expect("MONGO_DATABASE must be set");

    let mut client_options = ClientOptions::parse(mongo_uri).await?;
    client_options.app_name = Some("MyApp".to_string());

    let client = Client::with_options(client_options)?;
    let db = client.database(&database_name);

    println!("Connected to MongoDB!");

    Ok(db)
}