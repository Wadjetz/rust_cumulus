use uuid::Uuid;
use query::Query;
use user::{User, hash_password, verify_password};
use user_repository;
use token;

use errors::ErrorKind;
use std::error::Error;

#[derive(Debug)]
pub struct Mutation;

graphql_object!(Mutation: Query as "Mutation" |&self| {
    description: "Mutation"

    field signup(
        &executor,
        login: String as "Login",
        email: String as "Email",
        password: String as "Password"
    ) -> Result<String, String> as "Token" {
        let connection = executor.context().connection.clone().get().map_err(|e| e.description().to_string())?;
        let uuid = Uuid::new_v4();
        let hashed_password = hash_password(&password).unwrap(); // TODO
        let user = User::new(uuid, login, email.clone(), hashed_password);
        user_repository::insert(&connection, &user).and_then(|_| {
            token::create_token(uuid, email)
        }).map_err(|e| e.description().to_string())
    }
    
    field login(
        &executor,
        email: String as "Email",
        password: String as "Password"
    ) -> Result<String, String> as "Token" {
        let connection = executor.context().connection.clone().get().map_err(|e| e.description().to_string())?;
        user_repository::find_by_email(&connection, &email)
            .and_then(|user| {
                match verify_password(&password, &user.password) {
                    Ok(true) => token::create_token(user.uuid, email),
                    Ok(false) => Err(ErrorKind::WrongCredentials.into()),
                    Err(e) => Err(ErrorKind::WrongCredentials.into()),
                }
            })
            .map_err(|e| e.description().to_string())
    }
    
});
