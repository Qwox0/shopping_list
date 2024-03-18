use crate::error::{Error, Result};
use leptos::{provide_context, use_context, ServerFnError};
use sqlx::{pool::PoolConnection, Connection, Pool, Sqlite, SqliteConnection};

const DB_URL: &str = dotenvy_macro::dotenv!("DATABASE_URL");
type DBType = Sqlite;

#[derive(Debug, Clone)]
pub struct DB {
    pool: Pool<DBType>,
}

impl DB {
    pub fn new() -> Result<DB> {
        let pool = Pool::connect_lazy(DB_URL)?;
        Ok(DB { pool })
    }

    pub async fn connection_from_context() -> Result<PoolConnection<Sqlite>> {
        Ok(use_context::<Self>()
            .ok_or(Error::missing_ctx::<Self>())?
            .pool
            .acquire()
            .await?)
    }
}

pub async fn db() -> core::result::Result<SqliteConnection, ServerFnError> {
    Ok(SqliteConnection::connect(DB_URL).await?)
}
