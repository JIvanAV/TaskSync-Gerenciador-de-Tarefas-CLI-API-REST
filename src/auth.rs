use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData};
use serde::{Serialize, Deserialize};
use std::env;
use crate::models::Task;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt(user_id: &str) -> String {
    let expiration = chrono::Utc::now().timestamp() as usize + 3600;
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    let secret = env::var("JWT_SECRET").unwrap_or("mysecret".to_string());
    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    let token = encode(&Header::default(), &claims, &encoding_key).unwrap();

    token
}

pub fn validate_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").unwrap_or("mysecret".to_string());
    let decoding_key = DecodingKey::from_secret(secret.as_ref());

    decode::<Claims>(token, &decoding_key, &Validation::default())
}
