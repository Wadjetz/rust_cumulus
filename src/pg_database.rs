use postgres::rows::Row;
use postgres::error::Error;
use postgres_shared::error::{SqlState};
use r2d2_postgres::PostgresConnectionManager;
use r2d2::PooledConnection;
use postgres_shared::types::ToSql;

use errors::*;

pub trait InsertParams {
    fn insert_params(&self) -> Box<[&ToSql]>;
}

pub trait InsertQuery {
    fn insert_query(&self) -> String;
}

pub struct PgDatabase {
    connection: PooledConnection<PostgresConnectionManager>,
}

impl PgDatabase {
    pub fn new(connection: PooledConnection<PostgresConnectionManager>) -> Self {
        PgDatabase { connection }
    }

    pub fn insert<E>(&self, entity: E) -> Result<u64> where E: InsertQuery + InsertParams {
        self.connection.execute(&entity.insert_query(), &entity.insert_params())
        .map_err(|e| {
            println!("{:?}", e);
            match e {
                Error::Db(ref e) if e.code == SqlState::UniqueViolation => ErrorKind::AlreadyExist.into(),
                e => e.into(),
            }
        })
    }
}
