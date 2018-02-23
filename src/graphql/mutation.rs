use std::error::Error;
use juniper::{FieldError, FieldResult};

use config;
use graphql::query::Query;
use graphql::auth_mutation::AuthMutation;
use source::Source;
use sources_resolvers::add_rss_source_resolver;
use user::User;
use users_resolvers;
use errors;

#[derive(Debug)]
pub struct Mutation;

graphql_object!(Mutation: Query as "Mutation" |&self| {
    description: "Mutation"

    field signup(
        &executor,
        login: String as "Login",
        email: String as "Email",
        password: String as "Password"
    ) -> FieldResult<String> as "Token" {
        let user = User::new_secure(login, email, password)?;
        users_resolvers::signup_resolver(&executor.context().diesel_pool.clone(), &config::CONFIG, user)
            .map_err(|e| FieldError::from(&e.description().to_string()))
    }

    field auth(
        &executor,
        token: String as "Auth token"
    ) -> FieldResult<AuthMutation> as "Auth" {
        users_resolvers::auth_resolver(&executor.context().diesel_pool.clone(), &config::CONFIG, token)
            .map_err(|e| errors::ErrorKind::WrongCredentials)
            .map_err(|e| FieldError::from(&e.description().to_string()))
    }

    field add_rss_source(
        &executor,
        xml_url: String as "xml_url",
    ) -> FieldResult<Source> {
        add_rss_source_resolver(executor.context().connection.clone(), &xml_url)
            .map_err(|e| FieldError::from(&e.description().to_string()))
    }
});
