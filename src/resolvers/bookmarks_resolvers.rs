use graphql::query::Query;
use juniper::Executor;

use repositories::bookmark_repository;
use models::bookmark::Bookmark;
use models::user::User;
use errors::*;

pub fn add_bookmark<'a>(executor: &Executor<'a, Query>, bookmark: Bookmark, user: &User) -> Result<Bookmark> {
    let connection = executor.context().connection.clone().get()?;
    let maybe_bookmark = bookmark_repository::find_by_url_and_user(&connection, &bookmark.url, user)?;
    maybe_bookmark.ok_or_else(|| ErrorKind::AlreadyExist)?;
    bookmark_repository::insert(&connection, &bookmark)?;
    Ok(bookmark)
}
