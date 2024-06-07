use super::{
    server_functions::insert_from_client,
    variant_data::{NewVariant, Variant, VariantImpl},
};
use crate::{barcode_scanner::Barcode, error::Result};
#[cfg(feature = "ssr")]
use crate::{db::DBType, db::DB};
use leptos::{create_server_action, logging, ServerFnErrorErr};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemImpl<ID> {
    pub id: ID,
    pub amount: u64,
    pub completed: bool,
    pub variants: Vec<VariantImpl<ID>>,
}

pub type Item = ItemImpl<i64>;

#[cfg(feature = "ssr")]
impl Item {
    pub async fn select_by_id(id: i64, db: &DB) -> Result<Self> {
        let mut conn = db.connection().await?;
        ItemRow::select_by_id(id, conn.as_mut())
            .await?
            .fetch_variants(conn.as_mut())
            .await
    }

    pub async fn select_all(db: &DB) -> Result<Vec<Self>> {
        let mut conn = db.connection().await?;
        let rows = sqlx::query_as!(ItemRow, "SELECT * FROM item").fetch_all(conn.as_mut()).await?;
        let mut items = Vec::with_capacity(rows.len());
        for row in rows {
            items.push(row.fetch_variants(conn.as_mut()).await?)
        }
        Ok(items)
    }

    /// Returns whether rows where affected or not
    pub async fn remove(id: i64, db: &DB) -> Result<bool> {
        let mut tx = db.begin_transaction().await?;
        let affected = sqlx::query!("DELETE FROM item WHERE id = ?", id)
            .execute(tx.as_mut())
            .await?
            .rows_affected()
            > 0;

        sqlx::query!("DELETE FROM item_variant WHERE variant_of = ?", id)
            .execute(tx.as_mut())
            .await?;

        tx.commit().await?;
        Ok(affected)
    }
}

pub type NewItem = ItemImpl<()>;

impl Default for NewItem {
    fn default() -> Self {
        Self { id: (), amount: 1, completed: false, variants: vec![] }
    }
}

impl NewItem {
    pub fn with_default_variant() -> Self {
        Self { variants: vec![NewVariant::default()], ..Self::default() }
    }

    pub async fn from_barcode(barcode: Barcode) -> Result<Self> {
        Ok(Self { variants: vec![NewVariant::from_barcode(barcode).await?], ..Self::default() })
    }

    #[cfg(feature = "ssr")]
    pub async fn insert(self, db: &DB) -> Result<Item> {
        logging::log!("insert item: {:?}", self);
        let mut tx = db.begin_transaction().await?;

        let amount = self.amount as i64;
        let id = sqlx::query!(
            "INSERT INTO item(amount, completed) VALUES ( ?, ? )",
            amount,
            self.completed
        )
        .execute(tx.as_mut())
        .await?
        .last_insert_rowid();

        let mut variants = Vec::with_capacity(self.variants.len());
        for v in self.variants {
            variants.push(v.insert(id, tx.as_mut()).await?);
        }

        tx.commit().await?;
        Ok(Item { id, variants, ..self })
    }

    #[cfg(not(feature = "ssr"))]
    pub async fn insert_from_client(self) -> Result<Item> {
        let ids = insert_from_client(self.clone()).await.map_err(ServerFnErrorErr::from)?;
        Ok(Item {
            id: ids.item_id,
            variants: self
                .variants
                .into_iter()
                .zip(ids.variant_ids.iter().copied())
                .map(|(variant, id)| variant.with_id(id))
                .collect(),
            ..self
        })
    }
}

pub struct PendingItem(pub NewItem);

impl From<NewItem> for PendingItem {
    fn from(value: NewItem) -> Self {
        PendingItem(value)
    }
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ItemRow {
    pub id: i64,
    pub amount: i64,
    pub completed: bool,
}

#[cfg(feature = "ssr")]
impl ItemRow {
    pub async fn select_by_id(
        id: i64,
        conn: impl sqlx::Executor<'_, Database = DBType>,
    ) -> Result<Self> {
        Ok(sqlx::query_as!(ItemRow, "SELECT * FROM item WHERE id = ?", id)
            .fetch_one(conn)
            .await?)
    }

    pub async fn fetch_variants(
        self,
        conn: impl sqlx::Executor<'_, Database = DBType>,
    ) -> Result<Item> {
        let Self { id, amount, completed } = self;
        let variants = Variant::for_item(id, conn).await?;
        Ok(Item { id, amount: saturating_as(amount), completed, variants })
    }
}

fn saturating_as(int: i64) -> u64 {
    int.max(0) as u64
}
