use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::prelude::*;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use schema::bookmarks;

use errors::*;
use users::User;

#[derive(Debug, GraphQLObject, Deserialize, Queryable, Insertable)]
#[table_name="bookmarks"]
pub struct Bookmark {
    pub uuid: Uuid,
    pub url: String,
    pub title: String,
    pub description: Option<String>,
    pub path: Option<String>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub user_uuid: Uuid,
}

impl Bookmark {
    pub fn new(url: String, title: String, description: Option<String>, path: String, user_uuid: Uuid) -> Self {
        Bookmark {
            uuid: Uuid::new_v4(),
            url,
            title,
            description,
            path: Some(path),
            created: Utc::now().naive_utc(),
            updated: Utc::now().naive_utc(),
            user_uuid
        }
    }

    pub fn insert(connection: &PgConnection, bookmark: &Bookmark) -> Result<Bookmark> {
        Ok(diesel::insert_into(bookmarks::table)
            .values(bookmark)
            .get_result(&*connection)?)
    }

    pub fn is_exist(connection: &PgConnection, searched_url: &str, user: &User) -> Result<bool> {
        use schema::bookmarks::dsl::*;
        use diesel::dsl::*;
        use diesel::select;
        Ok(select(exists(bookmarks.filter(url.eq(searched_url)).filter(user_uuid.eq(&user.uuid)))).get_result(connection)?)
    }

    pub fn find(connection: &PgConnection, limit: i64, offset: i64, user: &User) -> Result<Vec<Bookmark>> {
        use schema::bookmarks::dsl::*;
        Ok(bookmarks.filter(user_uuid.eq(&user.uuid))
            .limit(limit)
            .offset(offset)
            .load::<Bookmark>(&*connection)?)
    }
}

pub fn add_bookmark_resolver(pool: &Pool<ConnectionManager<PgConnection>>, bookmark: Bookmark, user: &User) -> Result<Bookmark> {
    let connection = pool.get()?;
    if !Bookmark::is_exist(&connection, &bookmark.url, user)? {
        let inserted_bookmark = Bookmark::insert(&connection, &bookmark)?;
        Ok(inserted_bookmark)
    } else {
        Err(ErrorKind::AlreadyExist.into())
    }
}

pub fn bookmarks_resolver(pool: &Pool<ConnectionManager<PgConnection>>, limit: i64, offset: i64, user: &User) -> Result<Vec<Bookmark>> {
    let connection = pool.get()?;
    Ok(Bookmark::find(&connection, limit, offset, user)?)
}
