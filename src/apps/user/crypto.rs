use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    user_id: i32,
}

impl Claims {
    pub fn new(user_id: i32) -> Claims {
        Claims { user_id }
    }
}

pub trait Encode {
    fn encode_jwt(&self, user_id: i32) -> Result<String, Error>;
    fn hash_password(&self, plain_password: String) -> String;
}

pub trait Decode {
    fn decode_jwt(&self, token: String) -> Result<Claims, Error>;
}

pub trait Validate {
    fn validate_jwt(&self, token: String) -> bool;
    fn validate_password(&self, plain_password: String, hashed_password: String) -> bool;
}

pub struct Crypto {
    secret_key: String,
}

impl Crypto {
    pub fn new(secret_key: String) -> Crypto {
      Crypto { secret_key }
    }
}

impl Encode for Crypto {
    fn encode_jwt(&self, user_id: i32) -> Result<String, Error> {
        let claims = Claims::new(user_id);
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret_key.as_ref()),
        )?;
        Ok(token)
    }

    fn hash_password(&self, plain_password: String) -> String {
        let mut hasher = Sha256::new();
        hasher.update(plain_password.into_bytes());
        format!("{:x}", hasher.finalize())
    }
}
