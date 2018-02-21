use r2d2::Pool;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2::PooledConnection;
use postgres::rows::Row;
use postgres::error::{UNIQUE_VIOLATION};
use postgres::types::ToSql;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use std::ops::Deref;

use config;
use errors::*;

pub struct DbConn(pub PooledConnection<PostgresConnectionManager>);

pub fn create_db_pool(app_config: &config::Config) -> Pool<PostgresConnectionManager> {
    let database_url = app_config.database_url.clone();
    let manager = PostgresConnectionManager::new(database_url, TlsMode::None).expect("Create PostgresConnectionManager error");
    Pool::new(manager).expect("Failed to create pool")
}

pub trait Insertable {
    fn insert_query(&self) -> String;
    fn insert_params(&self) -> Box<[&ToSql]>;
}

pub struct PgDatabase {
    connection: PooledConnection<PostgresConnectionManager>,
}

impl PgDatabase {
    pub fn new(connection: PooledConnection<PostgresConnectionManager>) -> Self {
        PgDatabase { connection }
    }

    pub fn from_pool(pool: Pool<PostgresConnectionManager>) -> Result<PgDatabase> {
        let connection = pool.get()?;
        Ok(PgDatabase::new(connection))
    }

    pub fn insert<E>(&self, entity: &E) -> Result<u64> where E: Insertable {
        match self.connection.execute(&entity.insert_query(), &entity.insert_params()) {
            Ok(0) => Err(ErrorKind::NotInserted.into()),
            Ok(i) => Ok(i),
            Err(ref e) if e.code() == Some(&UNIQUE_VIOLATION) => Err(ErrorKind::AlreadyExist.into()),
            Err(e) => {
                println!("INSERT Error -> {:?}", e);
                Err(e.into())
            },
        }
    }

    pub fn update<'a>(&self, query: &str, params: &[&'a ToSql]) -> Result<u64> {
        Ok(self.connection.execute(query, params)?)
    }

    pub fn exist<'a>(&self, query: &str, params: &[&'a ToSql]) -> Result<bool> {
        let rows = self.connection.query(query, params)?;
        Ok(rows.iter().fold(false, |_, row| {
            let exist: i64 = row.get("exist");
            exist > 0
        }))
    }

    pub fn total<'a>(&self, query: &str, params: &[&'a ToSql]) -> Result<i64> {
        let rows = self.connection.query(query, params)?;
        let total = rows.iter().map(|row| row.get("total")).next().unwrap_or(0);
        Ok(total)
    }

    pub fn find<'a, E>(&self, query: &str, params: &[&'a ToSql]) -> Result<Vec<E>> where E: for<'b> From<Row<'b>> {
        let rows = self.connection.query(query, params)?;
        Ok(rows.iter().map(|row| row.into()).collect())
    }

    pub fn find_one<'a, E>(&self, query: &str, params: &[&'a ToSql]) -> Result<Option<E>> where E: for<'b> From<Row<'b>> {
        let rows = self.connection.query(query, params)?;
        let mut items: Vec<E> = rows.iter().map(|row| row.into()).collect();
        Ok(items.pop())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool<PostgresConnectionManager>>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}


impl Deref for DbConn {
    type Target = PooledConnection<PostgresConnectionManager>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DbConn> for PgDatabase {
    fn from(conn: DbConn) -> PgDatabase {
        PgDatabase::new(conn.0)
    }
}
