use r2d2_postgres::PostgresConnectionManager;
use r2d2::PooledConnection;
use postgres::error::{DUPLICATE_TABLE};
use postgres::rows::Row;

use errors::*;

#[derive(Debug)]
pub struct Evolution {
    pub id: String,
    pub up: String,
    pub down: String,
}

impl Evolution {
    pub fn new(id: &str, up: &str, down: &str) -> Self {
        Evolution {
            id: id.to_owned(),
            up: up.to_owned(),
            down: down.to_owned(),
        }
    }
}

impl<'a> From<Row<'a>> for Evolution {
    fn from(row: Row) -> Self {
        Evolution {
            id: row.get("id"),
            up: row.get("up"),
            down: row.get("down"),
        }
    }
}

pub fn sync(connection: PooledConnection<PostgresConnectionManager>, news: Vec<Evolution>) -> Result<()> {
    create_evolution_table(&connection)?;
    let current = find_evolutions(&connection)?;
    let non_applyed = find_non_applyed_evolutions(current, news);
    apply_evolutions(&connection, non_applyed)?;
    Ok(())
}

fn create_evolution_table(connection: &PooledConnection<PostgresConnectionManager>) -> Result<u64> {
    let create_table_query = r#"
        CREATE TABLE evolutions (
            id TEXT PRIMARY KEY,
            up TEXT NOT NULL,
            down TEXT NOT NULL
        );
    "#;
    match connection.execute(create_table_query, &[]) {
        Ok(i) => Ok(i),
        Err(ref e) if e.code() == Some(&DUPLICATE_TABLE) => {
            Ok(0)
        },
        Err(e) => Err(e.into())
    }
}

fn find_evolutions(connection: &PooledConnection<PostgresConnectionManager>) -> Result<Vec<Evolution>> {
    let query = "SELECT * FROM evolutions;";
    let rows = connection.query(query, &[])?;
    Ok(rows.iter().map(|row| row.into()).collect())
}

fn find_non_applyed_evolutions(news: Vec<Evolution>, current: Vec<Evolution>) -> Vec<Evolution> {
    current.into_iter().filter(|c| news.iter().find(|n| n.id == c.id).is_none()).collect()
}

fn apply_evolutions(connection: &PooledConnection<PostgresConnectionManager>, evolutions: Vec<Evolution>) -> Result<()> {
    for evolution in evolutions {
        connection.batch_execute(&evolution.up)?;
        insert_evolution(connection, &evolution)?;
    }
    Ok(())
}

fn insert_evolution(connection: &PooledConnection<PostgresConnectionManager>, evolution: &Evolution) -> Result<u64> {
    let query = "INSERT INTO evolutions (id, up, down) VALUES ($1, $2, $3);";
    Ok(connection.execute(query, &[&evolution.id, &evolution.up, &evolution.down])?)
}
