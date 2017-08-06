use uuid::Uuid;
use graphql::query::Query;
use graphql::auth_mutation::AuthMutation;
use users::{User, hash_password, verify_password};
use sources::{Source, add_rss_source_resolver};
use repositories::user_repository;
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

    field auth(
        &executor,
        token: String as "Auth token"
    ) -> Result<AuthMutation, String> as "Auth" {
      let connection = executor.context().connection.clone().get().expect("Error connection pool");
      token::decode_auth(&token)
            .and_then(|auth_data| user_repository::verify_user(&connection, auth_data))
            .map(AuthMutation::new)
            .map_err(|e| e.description().to_string())
    }

    field add_rss_source(
        &executor,
        xml_url: String as "xml_url",
    ) -> Result<Source, String> {
        add_rss_source_resolver(executor, &xml_url).map_err(|e| e.description().to_string())
    }
});
