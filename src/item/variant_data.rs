use crate::{barcode_scanner::OptionBarcode, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ItemVariantImpl<ID> {
    pub id: ID,
    pub name: String,

    pub shop_id: Option<i64>, // TODO
    pub barcode: OptionBarcode,
    pub brands: Option<String>,
    pub img_url: Option<String>,
    pub thumb_url: Option<String>,
    pub quantity: Option<String>,
}

pub type ItemVariant = ItemVariantImpl<i64>;
pub type NewItemVariant = ItemVariantImpl<()>;

impl Default for NewItemVariant {
    fn default() -> Self {
        Self {
            id: (),
            name: "".to_string(),
            shop_id: None,
            barcode: OptionBarcode::none(),
            brands: None,
            img_url: None,
            thumb_url: None,
            quantity: None,
        }
    }
}

impl ItemVariant {
    #[cfg(feature = "ssr")]
    pub async fn for_item(
        item_id: i64,
        conn: impl sqlx::Executor<'_, Database = crate::db::DBType>,
    ) -> Result<Vec<Self>> {
        Ok(sqlx::query_as!(
            ItemVariant,
            "SELECT id, name, shop_id, barcode, brands, img_url, thumb_url, quantity FROM \
             item_variant WHERE variant_of = ?",
            item_id
        )
        .fetch_all(conn)
        .await?)
    }
}

impl NewItemVariant {
    #[cfg(feature = "ssr")]
    pub async fn insert(
        self,
        item_id: i64,
        conn: impl sqlx::Executor<'_, Database = crate::db::DBType>,
    ) -> Result<ItemVariant> {
        let id = sqlx::query!(
            r#"INSERT INTO item_variant(variant_of, name, shop_id, barcode, brands, img_url, thumb_url, quantity)
            VALUES ( ?, ?, ?, ?, ?, ?, ?, ? )"#,
            item_id,
            self.name,
            self.shop_id,
            self.barcode,
            self.brands,
            self.img_url,
            self.thumb_url,
            self.quantity
        )
        .execute(conn)
        .await?
        .last_insert_rowid();
        Ok(ItemVariant { id, ..self })
    }
}
