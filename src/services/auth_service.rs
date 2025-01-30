use crate::models::user::User;
use crate::repository::user_repository::{find_user_by_email, save_user};
use crate::security::jwt::generate_jwt_token;
use crate::security::password::{hash_password, verify_password};
use actix_web::HttpResponse;
use bcrypt::verify;
use std::error::Error;

pub async fn register_service(user: User) -> Result<(), Box<dyn Error>> {
    let hashed_password = hash_password(&user.password).await?;
    save_user(User {
        password: hashed_password,
        ..user
    })
    .await?;
    Ok(())
}

pub async fn login_service(user: User) -> Result<String, Box<dyn Error>> {
    let db_user = find_user_by_email(&user.email).await?;
    if verify_password(&user.password, &db_user.password).await? {
        let token = generate_jwt_token(&db_user).await?;
        Ok(token)
    } else {
        Err("Invalid credentials".into())
    }
}
