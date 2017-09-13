use std::error::Error;
use std::path::Path;

use juniper::{FieldError, FieldResult};

use graphql::query::Query;
use users::User;
use bookmarks::bookmarks::{Bookmark, add_bookmark_resolver};
use cloud::files::File;
use sources::Source;
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

graphql_object!(AuthMutation: Query as "AuthMutation" |&self| {
    description: "AuthMutation"

    field me() -> &User as "User" {
        &self.user
    }

    field create_directory(
        &executor,
        name: String as "name",
        path: String as "path",
    ) -> FieldResult<File> {
        let connection = executor.context().connection.clone().get().map_err(|e| FieldError::from(&e.description().to_string()))?;
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
    ) -> FieldResult<Bookmark> as "Bookmark" {
        let bookmark = Bookmark::new(url, title, description, path, self.user.uuid);
        add_bookmark_resolver(executor.context().connection.clone(), bookmark, &self.user)
            .map_err(|e| FieldError::from(&e.description().to_string()))
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
