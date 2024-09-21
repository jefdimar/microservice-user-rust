use mongodb::{Database, bson::doc};
use crate::rocket::futures::TryStreamExt;
use crate::models::user::User;
use crate::errors::app_error::AppError;
use log::{info, error};

pub async fn add_user(db: &Database, user: User) -> Result<User, AppError> {
    info!("Adding new user: {:?}", user);
    let collection = db.collection::<User>("users");
    let result = collection.insert_one(user, None).await?;
    
    let inserted_id = result.inserted_id.as_object_id()
        .ok_or_else(|| {
            error!("Failed to insert user: Invalid ObjectId");
            AppError::InternalServerError("Failed to insert user".to_string())
        })?;
    
    let new_user = collection.find_one(doc! { "_id": inserted_id }, None).await?
        .ok_or_else(|| {
            error!("Failed to retrieve inserted user");
            AppError::InternalServerError("Failed to retrieve inserted user".to_string())
        })?;

    info!("User added successfully: {:?}", new_user);
    Ok(new_user)
}

pub async fn get_users(db: &Database) -> Result<Vec<User>, AppError> {
    info!("Fetching all users");
    let collection = db.collection::<User>("users");
    let mut cursor = collection.find(None, None).await?;

    let mut users = Vec::new();
    while let Some(user) = cursor.try_next().await? {
        users.push(user);
    }

    info!("Successfully fetched {} users", users.len());
    Ok(users)
}

pub async fn update_user(db: &Database, id: String, user: User) -> Result<User, AppError> {
    info!("Updating user with id: {}", id);
    let collection = db.collection::<User>("users");
    let object_id = mongodb::bson::oid::ObjectId::parse_str(&id)
        .map_err(|_| {
            error!("Invalid ID format: {}", id);
            AppError::BadRequest("Invalid ID format".to_string())
        })?;

    let update = doc! {
        "$set": {
            "name": &user.name,
            "email": &user.email,
        }
    };

    let result = collection.update_one(doc! { "_id": object_id }, update, None).await?;

    if result.modified_count == 0 {
        error!("User not found for update: {}", id);
        return Err(AppError::NotFound("User not found".to_string()));
    }

    let updated_user = collection.find_one(doc! { "_id": object_id }, None).await?
        .ok_or_else(|| {
            error!("Failed to retrieve updated user: {}", id);
            AppError::InternalServerError("Failed to retrieve updated user".to_string())
        })?;

    info!("User updated successfully: {:?}", updated_user);
    Ok(updated_user)
}

pub async fn delete_user(db: &Database, id: String) -> Result<(), AppError> {
    info!("Deleting user with id: {}", id);
    let collection = db.collection::<User>("users");
    let object_id = mongodb::bson::oid::ObjectId::parse_str(&id)
        .map_err(|_| {
            error!("Invalid ID format: {}", id);
            AppError::BadRequest("Invalid ID format".to_string())
        })?;

    let result = collection.delete_one(doc! { "_id": object_id }, None).await?;

    if result.deleted_count == 0 {
        error!("User not found for deletion: {}", id);
        return Err(AppError::NotFound("User not found".to_string()));
    }

    info!("User deleted successfully: {}", id);
    Ok(())
}