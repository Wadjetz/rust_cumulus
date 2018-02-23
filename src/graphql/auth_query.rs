use std::error::Error;

use juniper::{FieldError, FieldResult};

use graphql::query::Query;
use user::User;
use feeds;
use feeds::Feed;
use users_feeds::{unreaded_feeds, users_feeds_resolver, feeds_by_reaction_resolver, unreaded_feeds_by_source_resolver};
use source::Source;
use users_sources::{SourceStat, unfollowed_sources_resolver, users_sources_resolver, total_my_rss_sources_resolver, sources_stats_resolver};

#[derive(Debug)]
pub struct AuthQuery {
    pub user: User
}

impl AuthQuery {
    pub fn new(user: User) -> Self {
        AuthQuery { user }
    }
}

impl From<User> for AuthQuery {
    fn from(user: User) -> Self {
        AuthQuery::new(user)
    }
}

const DEFAULT_LIMIT: i32 = 10;

graphql_object!(AuthQuery: Query as "AuthQuery" |&self| {
    description: "AuthQuery"

    field me() -> Option<&User> as "User" {
        Some(&self.user)
    }

    field feeds(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset"
    ) -> FieldResult<Vec<Feed>> {
        feeds::find_resolver(executor.context().connection.clone(), limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0))
            .map_err(|e| FieldError::from(&e.description().to_string()))
    }

    field my_feeds(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset"
    ) -> FieldResult<Vec<Feed>> {
        users_feeds_resolver(executor.context().connection.clone(), limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0), &self.user)
            .map_err(|e| FieldError::from(&e.description().to_string()))
    }

    field my_sources(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset",
    ) -> FieldResult<Vec<Source>> {
        users_sources_resolver(executor.context().connection.clone(), limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0), &self.user)
            .map_err(|e| FieldError::from(&e.description().to_string()))
    }

    field unfollowed_sources(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset",
    ) -> FieldResult<Vec<Source>> {
        unfollowed_sources_resolver(executor.context().connection.clone(), limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0), &self.user)
            .map_err(|e| FieldError::from(&e.description().to_string()))
    }

    field unreaded_feeds(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset"
    ) -> FieldResult<Vec<Feed>> {
        unreaded_feeds(executor.context().connection.clone(), limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0), &self.user)
            .map_err(|e| FieldError::from(&e.description().to_string()))
    }

    field unreaded_feeds_by_source(
        &executor,
        source_uuid: String as "Source Uuid",
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset"
    ) -> FieldResult<Vec<Feed>> {
        unreaded_feeds_by_source_resolver(executor.context().connection.clone(), limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0), &source_uuid, &self.user)
            .map_err(|e| FieldError::from(&e.description().to_string()))
    }

    field feeds_by_reaction(
        &executor,
        reaction: String as "reaction",
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset",
    ) -> FieldResult<Vec<Feed>> {
        feeds_by_reaction_resolver(executor.context().connection.clone(), &reaction, limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0), &self.user)
            .map_err(|e| FieldError::from(&e.description().to_string()))
    }

    field total_my_rss_sources(
        &executor,
    ) -> FieldResult<i32> {
        total_my_rss_sources_resolver(executor.context().connection.clone(), &self.user)
            .map_err(|e| FieldError::from(&e.description().to_string()))
    }

    field sources_stats(
        &executor,
    ) -> FieldResult<Vec<SourceStat>> {
        sources_stats_resolver(executor.context().connection.clone(), &self.user)
            .map_err(|e| FieldError::from(e.to_string()))
    }
});
