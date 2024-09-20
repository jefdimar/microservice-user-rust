use mongodb::{Database, bson::doc};
use crate::rocket::futures::TryStreamExt;
use crate::models::user::User;
use crate::errors::mongodb_error::MongoDbError;

pub async fn add_user(db: &Database, user: User) -> Result<User, MongoDbError> {
    let collection = db.collection::<User>("users");
    let result = collection.insert_one(user, None).await?;
    
    let inserted_id = result.inserted_id.as_object_id()
        .ok_or(MongoDbError::InsertionFailed)?;
    let new_user = collection.find_one(doc! { "_id": inserted_id }, None).await?
        .ok_or(MongoDbError::InsertionFailed)?;

    Ok(new_user)
}

pub async fn get_users(db: &Database) -> Result<Vec<User>, MongoDbError> {
    let collection = db.collection::<User>("users");
    let mut cursor = collection.find(None, None).await?;

    let mut users = Vec::new();
    while let Some(user) = cursor.try_next().await? {
        users.push(user);
    }

    Ok(users)
}

pub async fn update_user(db: &Database, id: String, user: User) -> Result<User, MongoDbError> {
    let collection = db.collection::<User>("users");
    let object_id = mongodb::bson::oid::ObjectId::parse_str(&id)
        .map_err(|_| MongoDbError::InvalidId)?;

    let update = doc! {
        "$set": {
            "name": &user.name,
            "email": &user.email,
        }
    };

    let result = collection.update_one(doc! { "_id": object_id }, update, None).await?;

    if result.modified_count == 0 {
        return Err(MongoDbError::UserNotFound);
    }

    let updated_user = collection.find_one(doc! { "_id": object_id }, None).await?
        .ok_or(MongoDbError::UpdateFailed)?;

    Ok(updated_user)
}

pub async fn delete_user(db: &Database, id: String) -> Result<(), MongoDbError> {
    let collection = db.collection::<User>("users");
    let object_id = mongodb::bson::oid::ObjectId::parse_str(&id)
        .map_err(|_| MongoDbError::InvalidId)?;

    let result = collection.delete_one(doc! { "_id": object_id }, None).await?;

    if result.deleted_count == 0 {
        return Err(MongoDbError::UserNotFound);
    }

    Ok(())
}
