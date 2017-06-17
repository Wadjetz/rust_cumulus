use uuid::Uuid;
use bcrypt::{DEFAULT_COST, hash, verify, BcryptError};

#[derive(Debug)]
pub struct User {
    pub uuid: Uuid,
    pub login: String,
    pub email: String,
    pub password: String,
    //pub created: String,
    //pub updated: String,
    //pub last_connection: String,
    //pub active: bool,
}

impl User {
    pub fn new(uuid: Uuid, login: String, email: String, password: String) -> Self {
        User {
            uuid, login, email, password,
        }
    }
}

pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, BcryptError> {
  verify(password, hashed_password)
}
