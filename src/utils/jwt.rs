use serde::{Deserialize, Serialize};
use std::env;

use jsonwebtoken::{decode, encode, errors::Error, DecodingKey, EncodingKey, Header, Validation};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
}

pub fn generate_token(user_id: i32) -> String {
    let secret_key = env::var("JWT_SECRET").expect("JWT SECRET KEY MUST BE SET");

    let expires = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24)) // on real case you must change the exp time
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expires,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .expect("JWT generated")
}
pub fn validate_token(token: &str) -> Result<Claims, Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
