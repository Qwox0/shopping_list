use crate::list::item::{Item, ItemSerialized};
use anyhow::{Context, Ok};
use futures::TryStreamExt;
use sqlx::{
    query::{Query, QueryAs},
    sqlite::{SqliteArguments, SqliteRow},
    Connection, FromRow, QueryBuilder, Sqlite, SqliteConnection,
};
use std::marker::PhantomData;

pub trait InDb: Sized + for<'r> FromRow<'r, SqliteRow> + Send + Unpin {
    const COLUMNS_TUPLE: &'static str;
    //fn bind_values<'a>(&self, query: Query<'a, Sqlite, SqliteArguments<'a>>) -> Query<'a, Sqlite, SqliteArguments<'a>>;
    fn bind_values<'a>(
        &self,
        query_builder: &'a mut QueryBuilder<'a, Sqlite>,
    ) -> &mut QueryBuilder<'a, Sqlite>;

    fn query_value_fmt(&self) -> String {
        let value_count = Self::COLUMNS_TUPLE.split(",").count();
        let fmt = (0..value_count)
            .map(|_| "?")
            .collect::<Vec<&str>>()
            .join(", ");
        format!("({})", fmt)
    }

    async fn select_all(table_name: &'static str, connection: &mut SqliteConnection) -> Vec<Self> {
        let mut vec = vec![];
        let mut query_builder = QueryBuilder::new(format!("SELECT * FROM {}", table_name));
        let mut rows = query_builder.build_query_as::<Self>().fetch(connection);
        while let Some(row) = rows.try_next().await.unwrap() {
            vec.push(row);
        }
        vec
    }

    async fn insert(
        &self,
        table_name: &'static str,
        connection: &mut SqliteConnection,
    ) -> sqlx::Result<sqlx::sqlite::SqliteQueryResult> {
        let columns = Self::COLUMNS_TUPLE;
        let values_fmt = self.query_value_fmt();
        let mut query_builder = QueryBuilder::new(format!(
            "INSERT INTO {table_name} {columns} VALUES {values_fmt}"
        ));
        self.bind_values(&mut query_builder)
            .build()
            .execute(connection)
            .await
        /*
         *.expect("successfull db insert")
        let query = format!("INSERT INTO {table_name} {columns} VALUES {values_fmt}");
        self.bind_values(sqlx::query(&query))
        */
    }
}

const ITEMS_DB_URL: &str = "sqlite:./data/ShoppingList.db";
const ITEMS_TABLE_NAME: &str = "items";

pub struct ItemsDbTable<T>
where
    T: for<'a> FromRow<'a, SqliteRow> + InDb + Send + Unpin + std::fmt::Debug,
{
    db_url: &'static str,
    table_name: &'static str,
    connection: SqliteConnection,
    row_type: PhantomData<T>,
}

impl<T> ItemsDbTable<T>
where
    T: for<'a> FromRow<'a, SqliteRow> + InDb + Send + Unpin + std::fmt::Debug,
{
    pub async fn new() -> Result<Self, anyhow::Error> {
        Ok(ItemsDbTable {
            db_url: ITEMS_DB_URL,
            table_name: ITEMS_TABLE_NAME,
            connection: SqliteConnection::connect(ITEMS_DB_URL)
                .await
                .map_err(anyhow::Error::from)
                .context("Failed to connect to db")?,
            row_type: PhantomData,
        })
    }

    pub async fn get_all(&mut self) -> anyhow::Result<Vec<T>> {
        let mut items = Vec::new();
        let query = format!("SELECT * FROM {}", self.table_name);
        let mut rows = sqlx::query_as::<_, T>(&query).fetch(&mut self.connection);
        while let Some(row) = rows.try_next().await.unwrap() {
            items.push(row);
        }
        Ok(items)
    }

    pub async fn add_row(&mut self, row: T) -> anyhow::Result<()> {
        row.insert(self.table_name, &mut self.connection).await?;
        Ok(())
    }

    pub async fn add(&mut self, iter: impl Iterator<Item = T>) -> anyhow::Result<()> {
        for row in iter {
            self.add_row(row).await?
        }
        Ok(())
    }
}

pub async fn connect_db(url: &'static str) -> Result<SqliteConnection, anyhow::Error> {
    SqliteConnection::connect(url)
        .await
        .map_err(anyhow::Error::from)
        .context("Failed to connect to db")
}

pub async fn test_db() {
    let mut table: ItemsDbTable<ItemSerialized> = ItemsDbTable::new().await.expect("got db");
    /*
    table.add_row(ItemSerialized::new(
        "TestA",
        5,
        crate::list::item::ItemState::Needed,
    )).await.expect("added row");
    */
    let items = table.get_all().await.expect("got items");
    println!("items: {:?}", items);
}
