use graphql::query::Query;
use graphql::auth_mutation::AuthMutation;
use sources::{Source, add_rss_source_resolver};
use users::{signup_resolver, auth_resolver};
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
        signup_resolver(executor.context().connection.clone(), login, email, password)
            .map_err(|e| e.description().to_string())
    }

    field auth(
        &executor,
        token: String as "Auth token"
    ) -> Result<AuthMutation, String> as "Auth" {
        auth_resolver(executor.context().connection.clone(), token)
            .map_err(|e| e.description().to_string())
    }

    field add_rss_source(
        &executor,
        xml_url: String as "xml_url",
    ) -> Result<Source, String> {
        add_rss_source_resolver(executor.context().connection.clone(), &xml_url)
            .map_err(|e| e.description().to_string())
    }
});
