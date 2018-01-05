use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::prelude::*;
use postgres::rows::Row;
use postgres::types::ToSql;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use diesel::PgConnection;

use errors::*;
use users::User;
use pg::{PgInsertable, PgDatabase};

use diesel;
use diesel::prelude::*;
use schema::diesel_bookmarks;

#[derive(Debug, GraphQLObject, Deserialize, Queryable, Insertable)]
#[table_name="diesel_bookmarks"]
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
pub fn diesel_find_bookmarks(connection: &PgConnection) -> Result<Vec<Bookmark>> {
    use schema::diesel_bookmarks::dsl::*;
    Ok(diesel_bookmarks.filter(url.eq("lol"))
        .limit(5)
        .load::<Bookmark>(&*connection)?)
}
pub fn diesel_insert_bookmark(connection: &PgConnection, bookmark: &Bookmark) -> Result<Bookmark> {
    Ok(diesel::insert_into(diesel_bookmarks::table)
        .values(bookmark)
        .get_result(&*connection)?)
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
}

impl<'a> From<Row<'a>> for Bookmark {
    fn from(row: Row) -> Self {
        Bookmark {
            uuid: row.get("uuid"),
            url: row.get("url"),
            title: row.get("title"),
            description: row.get("description"),
            path: row.get("path"),
            created: row.get("created"),
            updated: row.get("updated"),
            user_uuid: row.get("user_uuid"),
        }
    }
}

impl PgInsertable for Bookmark {
    fn insert_query(&self) -> String {
        r#"
            INSERT INTO bookmarks (uuid, title, url, description, path, created, updated, user_uuid)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8);
        "#.to_owned()
    }

    fn insert_params(&self) -> Box<[&ToSql]> {
        Box::new([&self.uuid, &self.title, &self.url, &self.description, &self.path, &self.created, &self.updated, &self.user_uuid])
    }
}

fn is_bookmark_exist(pg: &PgDatabase, url: &str, user: &User) -> Result<bool> {
    let query = "SELECT COUNT(*) AS exist FROM bookmarks WHERE url = $1 AND user_uuid = $2::uuid;";
    Ok(pg.exist(query, &[&url, &user.uuid])?)
}

pub fn add_bookmark_resolver(pool: Pool<PostgresConnectionManager>, bookmark: Bookmark, user: &User) -> Result<Bookmark> {
    let pg = PgDatabase::from_pool(pool)?;
    if !is_bookmark_exist(&pg, &bookmark.url, user)? {
        pg.insert(&bookmark)?;
        Ok(bookmark)
    } else {
        Err(ErrorKind::AlreadyExist.into())
    }
}

fn find_bookmarks(pg: &PgDatabase, limit: i32, offset: i32, user: &User) -> Result<Vec<Bookmark>> {
    let query = "SELECT * FROM bookmarks WHERE user_uuid = $1::uuid LIMIT $2::int OFFSET $3::int;";
    Ok(pg.find(query, &[&user.uuid, &limit, &offset])?)
}

pub fn bookmarks_resolver(pool: Pool<PostgresConnectionManager>, limit: i32, offset: i32, user: &User) -> Result<Vec<Bookmark>> {
    let pg = PgDatabase::from_pool(pool)?;
    Ok(find_bookmarks(&pg, limit, offset, user)?)
}
