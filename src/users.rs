use uuid::Uuid;
use bcrypt::{DEFAULT_COST, hash, verify, BcryptError};

use graphql::query::Query;

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

    #[allow(dead_code)]
    pub fn new_secure(login: String, email: String, password: String) -> Result<User, BcryptError> {
        hash_password(&password).map(|hashed_password| User {
            uuid: Uuid::new_v4(),
            login,
            email,
            password: hashed_password,
        })
    }
}

pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, BcryptError> {
  verify(password, hashed_password)
}

graphql_object!(User: Query as "User" |&self| {
    description: "User"

    field uuid() -> String as "uuid" {
        self.uuid.hyphenated().to_string()
    }

    field email() -> &String as "email" {
        &self.email
    }

    field login() -> &String as "login" {
        &self.login
    }
});

