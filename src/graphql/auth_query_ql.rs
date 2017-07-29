use graphql::query::Query;
use models::user::User;
use models::file::File;
use models::feed::Feed;
use models::bookmark::Bookmark;
use models::feed_source::FeedSource;
use repositories::{bookmark_repository, file_repository, feed_repository, users_feeds_sources_repository};

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
        bookmark_repository::find(&connection, limit.unwrap_or(50), offset.unwrap_or(0), &self.user)
                            .map_err(|e| e.description().to_string())
    }

    field files(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset"
    ) -> Result<Vec<File>, String> {
        let connection = executor.context().connection.clone().get().map_err(|e| e.description().to_string())?;
        file_repository::find(&connection, limit.unwrap_or(50), offset.unwrap_or(0), &self.user)
                            .map_err(|e| e.description().to_string())
    }

    field feeds(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset"
    ) -> Result<Vec<Feed>, String> {
        let connection = executor.context().connection.clone().get().map_err(|e| e.description().to_string())?;
        feed_repository::find(&connection, limit.unwrap_or(50), offset.unwrap_or(0), &self.user)
            .map_err(|e| e.description().to_string())
    }

    field feeds_sources(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset"
    ) -> Result<Vec<FeedSource>, String> {
        let connection = executor.context().connection.clone().get().map_err(|e| e.description().to_string())?;
        users_feeds_sources_repository::find_by_user(&connection, limit.unwrap_or(50), offset.unwrap_or(0), &self.user)
            .map_err(|e| e.description().to_string())
    }
});
