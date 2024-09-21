use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UserMongo {
    #[schemars(skip)]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
}