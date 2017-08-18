use graphql::query::Query;
use users::User;
use files::{File, files_resolver};
use feeds;
use feeds::Feed;
use users_feeds::{unreaded_feeds, users_feeds_resolver, feeds_by_reaction_resolver};
use bookmarks::{bookmarks_resolver, Bookmark};
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
        bookmarks_resolver(executor.context().connection.clone(), limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0), &self.user)
            .map_err(|e| e.description().to_string())
    }

    field files(&executor, limit: Option<i32> as "Limit", offset: Option<i32> as "Offset") -> Result<Vec<File>, String> {
        files_resolver(executor.context().connection.clone(), limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0), &self.user)
            .map_err(|e| e.description().to_string())
    }

    field feeds(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset"
    ) -> Result<Vec<Feed>, String> {
        feeds::find_resolver(executor.context().connection.clone(), limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0))
            .map_err(|e| e.description().to_string())
    }

    field my_feeds(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset"
    ) -> Result<Vec<Feed>, String> {
        users_feeds_resolver(executor.context().connection.clone(), limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0), &self.user)
            .map_err(|e| e.description().to_string())
    }

    field my_sources(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset",
    ) -> Result<Vec<Source>, String> {
        users_sources_resolver(executor.context().connection.clone(), limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0), &self.user)
            .map_err(|e| e.description().to_string())
    }

    field unfollowed_sources(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset",
    ) -> Result<Vec<Source>, String> {
        unfollowed_sources_resolver(executor.context().connection.clone(), limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0), &self.user)
            .map_err(|e| e.description().to_string())
    }

    field unreaded_feeds(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset"
    ) -> Result<Vec<Feed>, String> {
        unreaded_feeds(executor.context().connection.clone(), limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0), &self.user)
            .map_err(|e| e.description().to_string())
    }

    field feeds_by_reaction(
        &executor,
        reaction: String as "reaction",
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset",
    ) -> Result<Vec<Feed>, String> {
        feeds_by_reaction_resolver(executor.context().connection.clone(), &reaction, limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0), &self.user)
            .map_err(|e| {
                println!("{:?}", e);
                e.description().to_string()
            })
    }
});
