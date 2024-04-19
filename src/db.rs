use crate::error::{Error, Result};
use leptos::{use_context, ServerFnError};
use sqlx::{pool::PoolConnection, Connection, Pool, Sqlite, SqliteConnection, Transaction};

const DB_URL: &str = dotenvy_macro::dotenv!("DATABASE_URL");
pub type DBType = Sqlite;

#[derive(Debug, Clone)]
pub struct DB {
    pool: Pool<DBType>,
}

lazy_static::lazy_static! {
    pub static ref MY_DB: DB = DB::new().expect("could connect to DB");
}

impl DB {
    pub fn new() -> Result<DB> {
        let pool = Pool::connect_lazy(DB_URL)?;
        Ok(DB { pool })
    }

    pub fn as_pool(&self) -> &Pool<DBType> {
        &self.pool
    }

    pub async fn connection(&self) -> Result<PoolConnection<DBType>> {
        Ok(self.pool.acquire().await?)
    }

    pub async fn begin_transaction(&self) -> Result<Transaction<'static, DBType>> {
        Ok(self.pool.begin().await?)
    }

    pub async fn connection_from_context() -> Result<PoolConnection<DBType>> {
        Ok(use_context::<Self>().ok_or(Error::missing_ctx::<Self>())?.connection().await?)
    }
}
