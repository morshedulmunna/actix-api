use crate::models::user::User;
use actix_web::HttpResponse;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

pub async fn generate_jwt_token(user: &User) -> Result<String, HttpResponse> {
    let claims = Claims {
        sub: user.id,
        exp: 10000000000,
    }; // Example expiration
    let secret = "SECRET_KEY";
    let header = Header::new(Algorithm::HS256);

    let token = encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()))
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
    Ok(token)
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: Option<i32>,
    exp: usize,
}
