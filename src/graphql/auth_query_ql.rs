use graphql::query::Query;
use models::user::User;
use models::file::File;
use feeds;
use feeds::Feed;
use users_feeds::{unreaded_feeds, users_feeds_resolver};
use models::bookmark::Bookmark;
use repositories::{bookmark_repository, file_repository};
use sources::Source;
use users_sources::{unfollowed_sources_resolver, users_sources_resolver};

use std::error::Error;

#[derive(Debug)]
pub struct AuthQuery {
    pub user: User
}

impl AuthQuery {
    pub fn new(user: User) -> Self {
        AuthQuery { user }
    }
}

const DEFAULT_LIMIT: i32 = 10;

graphql_object!(AuthQuery: Query as "AuthQuery" |&self| {
    description: "AuthQuery"

    field me() -> Option<&User> as "User" {
        Some(&self.user)
    }

    field bookmarks(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset"
    ) -> Result<Vec<Bookmark>, String> {
        let connection = executor.context().connection.clone().get().map_err(|e| e.description().to_string())?;
        bookmark_repository::find(&connection, limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0), &self.user)
                            .map_err(|e| e.description().to_string())
    }

    field files(&executor, limit: Option<i32> as "Limit", offset: Option<i32> as "Offset") -> Result<Vec<File>, String> {
        let connection = executor.context().connection.clone().get().map_err(|e| e.description().to_string())?;
        file_repository::find(&connection, limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0), &self.user)
                            .map_err(|e| e.description().to_string())
    }

    field feeds(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset"
    ) -> Result<Vec<Feed>, String> {
        feeds::find_resolver(executor, limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0)).map_err(|e| e.description().to_string())
    }

    field my_feeds(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset"
    ) -> Result<Vec<Feed>, String> {
        users_feeds_resolver(executor, limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0), &self.user)
            .map_err(|e| e.description().to_string())
    }

    field my_sources(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset",
    ) -> Result<Vec<Source>, String> {
        users_sources_resolver(executor, limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0), &self.user)
            .map_err(|e| e.description().to_string())
    }

    field unfollowed_sources(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset",
    ) -> Result<Vec<Source>, String> {
        unfollowed_sources_resolver(executor, limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0), &self.user)
            .map_err(|e| e.description().to_string())
    }

    field unreaded_feeds(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset"
    ) -> Result<Vec<Feed>, String> {
        unreaded_feeds(executor, limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0), &self.user)
            .map_err(|e| e.description().to_string())
    }
});
