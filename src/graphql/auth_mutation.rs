use graphql::query::Query;
use models::user::User;
use models::bookmark::Bookmark;
use repositories::bookmark_repository;
use errors;

use std::error::Error;

#[derive(Debug)]
pub struct AuthMutation {
    pub user: User
}

impl AuthMutation {
    pub fn new(user: User) -> Self {
        AuthMutation { user }
    }
}

graphql_object!(AuthMutation: Query as "AuthMutation" |&self| {
    description: "AuthMutation"

    field me() -> Option<&User> as "User" {
        Some(&self.user)
    }

    field add_bookmark(
       &executor,
       url: String as "Url",
       title: String as "Title",
       path: String as "Path",
       description: Option<String> as "Description",
    ) -> Result<Bookmark, String> as "Bookmark" {
        let bookmark = Bookmark::from_request(url.clone(), title, description, path, self.user.uuid);
        let connection = executor.context().connection.clone().get().map_err(|e| e.description().to_string())?;
        bookmark_repository::find_by_url_and_user(&connection, &url, &self.user)
            .and_then(|maybe_bookmark| {
                match maybe_bookmark {
                Some(bookmark) => {
                    bookmark_repository::insert(&connection, &bookmark)
                                        .map(|_| bookmark)
                },
                None => {
                    Err(errors::ErrorKind::AlreadyExist.into())
                }
            }
        })
        .map_err(|e| e.description().to_string())
    }
});
