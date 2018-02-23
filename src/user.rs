use uuid::Uuid;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use validator::Validate;
use bcrypt::{DEFAULT_COST, hash, verify};

use errors::*;
use schema::users;

#[derive(Debug, PartialEq, Identifiable, Queryable, Insertable, Validate)]
#[primary_key(uuid)]
#[table_name="users"]
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

graphql_object!(User: () |&self| {
    field uuid() -> &Uuid {
        &self.uuid
    }

    field login() -> &str {
        &self.login
    }

    field email() -> &str {
        &self.email
    }

    field created() -> &Option<NaiveDateTime> {
        &self.created
    }

    field updated() -> &Option<NaiveDateTime> {
        &self.updated
    }
});

// TODO remove
use postgres::rows::Row;
impl<'a> From<Row<'a>> for User {
    fn from(row: Row) -> Self {
        User {
            uuid: row.get("uuid"),
            login: row.get("login"),
            email: row.get("email"),
            password: row.get("password"),
            created: row.get("created"),
            updated: row.get("updated"),
        }
    }
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

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool> {
    Ok(verify(password, hashed_password)?)
}
