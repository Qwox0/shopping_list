use super::openfoodsfacts::OpenFoodFactsProduct;
use crate::{
    barcode_scanner::{Barcode, OptionBarcode},
    error::Result,
};
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
    pub packaging: Option<String>,
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
            packaging: None,
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
            "SELECT id, name, shop_id, barcode, brands, img_url, thumb_url, packaging, quantity \
             FROM item_variant WHERE variant_of = ?",
            item_id
        )
        .fetch_all(conn)
        .await?)
    }
}

impl NewItemVariant {
    pub async fn from_barcode(barcode: Barcode) -> Result<Self> {
        OpenFoodFactsProduct::request_with_barcode(barcode).await.map(|data| Self {
            name: data.product_name,
            barcode: OptionBarcode::some(barcode),
            img_url: Some(data.image_url),
            thumb_url: Some(data.image_thumb_url),
            brands: Some(data.brands),
            packaging: Some(data.packaging),
            quantity: Some(data.quantity),
            ..Self::default()
        })
    }

    pub fn with_id(self, id: i64) -> ItemVariant {
        ItemVariant { id, ..self }
    }

    #[cfg(feature = "ssr")]
    pub async fn insert(
        self,
        item_id: i64,
        conn: impl sqlx::Executor<'_, Database = crate::db::DBType>,
    ) -> Result<ItemVariant> {
        let id = sqlx::query!(
            r#"INSERT INTO item_variant(variant_of, name, shop_id, barcode, brands, img_url, thumb_url, packaging, quantity)
            VALUES ( ?, ?, ?, ?, ?, ?, ?, ?, ? )"#,
            item_id,
            self.name,
            self.shop_id,
            self.barcode,
            self.brands,
            self.img_url,
            self.thumb_url,
            self.packaging,
            self.quantity
        )
        .execute(conn)
        .await?
        .last_insert_rowid();
        Ok(self.with_id(id))
    }
}
