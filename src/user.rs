use uuid::Uuid;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use validator::Validate;
use bcrypt::{DEFAULT_COST, hash};

use errors::*;

#[derive(Debug, PartialEq, GraphQLObject, Validate)]
pub struct User {
    pub uuid: Uuid,
    #[validate(length(min = "1"))]
    pub login: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "6"))]
    pub password: String,
    pub created: Option<NaiveDateTime>,
    pub updated: Option<NaiveDateTime>,
}

impl User {
    pub fn new_secure(login: String, email: String, password: String) -> Result<User> {
        let hashed_password = hash_password(&password)?;
        let user = User {
            uuid: Uuid::new_v4(),
            login,
            email,
            password: hashed_password,
            created: Some(Utc::now().naive_utc()),
            updated: Some(Utc::now().naive_utc()),
        };
        Ok(user)
    }
}

pub fn hash_password(password: &str) -> Result<String> {
    Ok(hash(password, DEFAULT_COST)?)
}
