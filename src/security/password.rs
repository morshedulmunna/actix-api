use async_trait::async_trait;
use bcrypt::{hash, verify};

pub async fn hash_password(password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let hashed_password = hash(password, 10)?;
    Ok(hashed_password)
}

pub async fn verify_password(
    stored_hash: &str,
    password: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let result = verify(password, stored_hash)?;
    Ok(result)
}
