use query::Query;

#[derive(Debug)]
pub struct Auth;

graphql_object!(Auth: Query as "Auth" |&self| {
    description: "Auth"

    field toto(
        token: String as "Auth token"
    ) -> String as "path" {
        token
    }
});
