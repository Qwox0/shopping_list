use super::openfoodsfacts::OpenFoodFactsProduct;
use crate::{
    barcode_scanner::{Barcode, OptionBarcode},
    error::Result,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct VariantImpl<ID> {
    pub id: ID,
    pub name: String,

    //pub shop_id: Option<i64>,
    pub shop: String,
    pub barcode: OptionBarcode,
    pub brands: String,
    pub img_url: Option<String>,
    pub thumb_url: Option<String>,
    pub packaging: String,
    pub quantity: String,
}

pub type Variant = VariantImpl<i64>;

impl Variant {
    #[cfg(feature = "ssr")]
    pub async fn for_item(
        item_id: i64,
        conn: impl sqlx::Executor<'_, Database = crate::db::DBType>,
    ) -> Result<Vec<Self>> {
        Ok(sqlx::query_as!(
            Variant,
            "SELECT id, name, shop, barcode, brands, img_url, thumb_url, packaging, quantity FROM \
             item_variant WHERE variant_of = ?",
            item_id
        )
        .fetch_all(conn)
        .await?)
    }
}

pub type NewVariant = VariantImpl<()>;

impl Default for NewVariant {
    fn default() -> Self {
        Self {
            id: (),
            name: "".to_string(),
            //shop_id: None,
            shop: "".to_string(),
            barcode: OptionBarcode::none(),
            brands: "".to_string(),
            img_url: None,
            thumb_url: None,
            packaging: "".to_string(),
            quantity: "".to_string(),
        }
    }
}

impl NewVariant {
    pub async fn from_barcode(barcode: Barcode) -> Result<Self> {
        OpenFoodFactsProduct::request_with_barcode(barcode).await.map(|data| Self {
            name: data.product_name,
            barcode: OptionBarcode::some(barcode),
            img_url: Some(data.image_url),
            thumb_url: Some(data.image_thumb_url),
            brands: data.brands,
            packaging: data.packaging,
            quantity: data.quantity,
            ..Self::default()
        })
    }

    pub fn with_id(self, id: i64) -> Variant {
        Variant { id, ..self }
    }

    #[cfg(feature = "ssr")]
    pub async fn insert(
        self,
        item_id: i64,
        conn: impl sqlx::Executor<'_, Database = crate::db::DBType>,
    ) -> Result<Variant> {
        let id = sqlx::query!(
            r#"INSERT INTO item_variant(variant_of, name, shop, barcode, brands, img_url, thumb_url, packaging, quantity)
            VALUES ( ?, ?, ?, ?, ?, ?, ?, ?, ? )"#,
            item_id,
            self.name,
            self.shop,
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

pub struct PendingVariant(pub NewVariant);

impl From<NewVariant> for PendingVariant {
    fn from(value: NewVariant) -> Self {
        PendingVariant(value)
    }
}
