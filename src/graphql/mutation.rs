use std::error::Error;
use juniper::{FieldError, FieldResult};

use graphql::query::Query;
use graphql::auth_mutation::AuthMutation;
use mindstream::sources::{Source, add_rss_source_resolver};
use users::{User, signup_resolver, auth_resolver};

#[derive(Debug)]
pub struct Mutation;

graphql_object!(Mutation: Query as "Mutation" |&self| {
    description: "Mutation"

    field auth(
        &executor,
        token: String as "Auth token"
    ) -> FieldResult<AuthMutation> as "Auth" {
        auth_resolver(&executor.context().diesel_pool, token)
            .map_err(|e| FieldError::from(&e.description().to_string()))
    }

    field signup(
        &executor,
        login: String as "Login",
        email: String as "Email",
        password: String as "Password"
    ) -> FieldResult<String> as "Token" {
        let user = User::new_secure(login, email, password)?;
        signup_resolver(&executor.context().diesel_pool, user).map_err(|e| FieldError::from(&e.description().to_string()))
    }

    field add_rss_source(
        &executor,
        xml_url: String as "xml_url",
    ) -> FieldResult<Source> {
        add_rss_source_resolver(executor.context().connection.clone(), &xml_url)
            .map_err(|e| FieldError::from(&e.description().to_string()))
    }
});
