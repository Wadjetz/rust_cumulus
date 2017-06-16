use query::Query;

#[derive(Debug)]
pub struct Mutation;

graphql_object!(Mutation: Query as "Mutation" |&self| {
    description: "Mutation"

    field toto(
        token: String as "Auth token"
    ) -> String as "path" {
        token
    }
});
