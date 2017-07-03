use graphql::query::Query;
use models::user::User;
use models::bookmark::Bookmark;
use models::file::File;
use repositories::bookmark_repository;
use errors;

use std::error::Error;
use std::path::Path;

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

    field create_directory(
        &executor,
        name: String as "name",
        path: String as "path",
    ) -> Result<File, String> {
        let connection = executor.context().connection.clone().get().map_err(|e| e.description().to_string())?;
        let path = Path::new(&path);
        let maybe_parent_path = path.parent();
        let directory = File::new_directory(&name, path.to_str().unwrap_or("/"), self.user.uuid);
        println!("{:?}", directory);
        Ok(directory)
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
