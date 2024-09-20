use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
}