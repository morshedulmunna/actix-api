use crate::models::user::User;
use crate::repository::user_repository::{find_user_by_id, update_user};
use std::error::Error;

pub async fn get_user_service(user_id: i32) -> Result<User, Box<dyn Error>> {
    let user = find_user_by_id(user_id).await?;
    Ok(user)
}

pub async fn update_user_service(user_id: i32, updated_user: User) -> Result<User, Box<dyn Error>> {
    let user = update_user(user_id, updated_user).await?;
    Ok(user)
}
