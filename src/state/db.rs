use std::marker::PhantomData;

use crate::state::item_serialized::ItemSerialized;
use anyhow::{Context, Ok};
use futures::TryStreamExt;
use leptos::ServerFnError;
use sqlx::{
    query::Query,
    sqlite::{SqliteArguments, SqliteRow},
    Connection, FromRow, QueryBuilder, Sqlite, SqliteConnection,
};
use uuid::Uuid;

pub trait InDb: Sized {
    const DB_URL: &'static str;
    const TABLE_NAME: &'static str;
    const HEADERS_TUPLE: &'static str;
    type RowType: for<'r> FromRow<'r, SqliteRow> + Send + Unpin;

    fn bind_values<'a>(
        query: Query<'a, Sqlite, SqliteArguments<'a>>,
        row: Self::RowType,
    ) -> Query<'a, Sqlite, SqliteArguments<'a>>;

    fn query_value_fmt() -> String {
        let value_count = Self::HEADERS_TUPLE.split(",").count();
        let fmt = (0..value_count)
            .map(|_| "?")
            .collect::<Vec<&str>>()
            .join(", ");
        format!("({})", fmt)
    }

    fn get_insert_query_string() -> String {
        format!(
            "INSERT INTO {} {} VALUES {}",
            Self::TABLE_NAME,
            Self::HEADERS_TUPLE,
            Self::query_value_fmt(),
        )
    }

    async fn get_db_connection() -> anyhow::Result<DbConnection<Self>> {
        DbConnection::<Self>::new().await
    }
}

pub struct DbConnection<T> {
    connection: SqliteConnection,
    row_type: PhantomData<T>,
}

impl<T: InDb> DbConnection<T> {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(DbConnection {
            connection: SqliteConnection::connect(T::DB_URL)
                .await
                .map_err(anyhow::Error::from)
                .context("Failed to connect to db")?,
            row_type: PhantomData,
        })
    }

    pub async fn get_all(mut self, cx: leptos::Scope) -> anyhow::Result<Vec<T::RowType>> {
        let mut items = Vec::new();
        let query = format!("SELECT * FROM {}", T::TABLE_NAME);
        let mut rows = sqlx::query_as::<_, T::RowType>(&query).fetch(&mut self.connection);
        while let Some(row) = rows.try_next().await.unwrap() {
            items.push(row);
        }
        Ok(items)
    }

    pub async fn add(mut self, row: T::RowType) -> anyhow::Result<Self> {
        T::bind_values(sqlx::query(&T::get_insert_query_string()), row)
            .execute(&mut self.connection)
            .await?;
        Ok(self)
    }

    pub async fn remove(mut self, id: Uuid) -> anyhow::Result<Self> {
        let query = format!("DELETE FROM {} WHERE id = ?", T::TABLE_NAME);
        sqlx::query(&query)
            .bind(id)
            .execute(&mut self.connection)
            .await
            .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
        Ok(self)
    }
}
