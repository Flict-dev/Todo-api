use chrono::{Duration, Utc};
use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub exp: usize,
}

impl Claims {
    pub fn new(user_id: i32) -> Claims {
        let _date = Utc::now(); // + Duration::hours(3)
        let exp = _date.timestamp() as usize;
        Claims { user_id, exp }
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
    fn validate_jwt(&self, token: String) -> Result<Option<Claims>, Error>;
    fn validate_password(&self, plain_password: String, hashed_password: String) -> bool;
}

#[derive(Clone)]
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

impl Decode for Crypto {
    fn decode_jwt(&self, token: String) -> Result<Claims, Error> {
        let token = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(self.secret_key.as_ref()),
            &Validation::default(),
        )?;
        Ok(token.claims)
    }
}

impl Validate for Crypto {
    fn validate_jwt(&self, token: String) -> Result<Option<Claims>, Error> {
        let _date = Utc::now();
        let current_exp = _date.timestamp() as usize;
        let claims = self.decode_jwt(token)?;
        if claims.exp > current_exp {
            return Ok(Some(claims));
        }
        return Ok(None);
    }

    fn validate_password(&self, plain_password: String, hashed_password: String) -> bool {
        let hash = &self.hash_password(plain_password);
        if *hash == hashed_password {
            return true;
        }
        return false;
    }
}
