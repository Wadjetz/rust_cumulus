use juniper::{FieldError, FieldResult};

use graphql::query::Query;
use user::User;
use source::Source;
use users_sources;
use users_feeds;

#[derive(Debug)]
pub struct AuthMutation {
    pub user: User
}

impl AuthMutation {
    pub fn new(user: User) -> Self {
        AuthMutation { user }
    }
}

impl From<User> for AuthMutation {
    fn from(user: User) -> Self {
        AuthMutation::new(user)
    }
}

graphql_object!(AuthMutation: Query as "AuthMutation" |&self| {
    description: "AuthMutation"

    field me() -> &User as "User" {
        &self.user
    }

    field fallow_source(
        &executor,
        source_uuid: String as "source_uuid",
    ) -> FieldResult<Source> {
        users_sources::fallow_source_resolver(executor.context().connection.clone(), &source_uuid, &self.user)
            .map_err(|e| FieldError::from(e.to_string()))
    }

    field feed_reaction(
        &executor,
        feed_uuid: String as "feed_uuid",
        reaction: String as "reaction",
    ) -> FieldResult<String> {
        users_feeds::reaction_feed_resolver(executor.context().connection.clone(), &feed_uuid, &reaction, &self.user)
            .map(|_| String::from("ok"))
            .map_err(|e| FieldError::from(e.to_string()))
    }
});
