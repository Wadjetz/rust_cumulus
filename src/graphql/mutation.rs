use graphql::query::Query;
use graphql::auth_mutation::AuthMutation;
use sources::{Source, add_rss_source_resolver};
use repositories::user_repository;
use token;
use users;
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
        users::signup_resolver(executor, login, email, password).map_err(|e| e.description().to_string())
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
