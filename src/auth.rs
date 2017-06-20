use query::Query;
use user::User;

#[derive(Debug)]
pub struct Auth {
    pub user: User
}

impl Auth {
    pub fn new(user: User) -> Self {
        Auth { user }
    }
}

graphql_object!(Auth: Query as "Auth" |&self| {
    description: "Auth"

    field me() -> Option<&User> as "User" {
        Some(&self.user)
    }
});
