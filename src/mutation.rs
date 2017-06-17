use uuid::Uuid;
use query::Query;
use user::{User, hash_password};
use user_repository;
use token;

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
        let connection = executor.context().connection.clone().get().expect("Error connection pool");
        let uuid = Uuid::new_v4();
        let hashed_password = hash_password(&password).unwrap(); // TODO
        let user = User::new(uuid, login, email.clone(), hashed_password);
        user_repository::insert(&connection, &user).and_then(|_| {
            token::create_token(uuid, email)
        }).map_err(|e| {
            e.description().to_string()
        })
    }

});
