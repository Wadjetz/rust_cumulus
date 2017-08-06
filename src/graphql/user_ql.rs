use graphql::query::Query;
use users::User;

graphql_object!(User: Query as "User" |&self| {
    description: "User"

    field uuid() -> String as "uuid" {
        self.uuid.hyphenated().to_string()
    }

    field email() -> &String as "email" {
        &self.email
    }

    field login() -> &String as "login" {
        &self.login
    }
});
