use rocket::response::status::Custom;
use rocket::http::Status;

use crate::models::user::User;
use super::postgres_config::DbClient;

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