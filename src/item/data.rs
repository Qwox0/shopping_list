use crate::{
    barcode_scanner::{Barcode, OptionBarcode},
    error::Result,
    item::openfoodsfacts::OpenFoodFactsProduct,
};
use serde::{Deserialize, Serialize};

/*
CREATE TABLE IF NOT EXISTS items (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    amount INTEGER NOT NULL,
    completed BOOLEAN NOT NULL,
);


CREATE TABLE IF NOT EXISTS shops (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
);
 */

/*
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ItemData {
    pub name: String,
    pub amount: i64,
    pub barcode: OptionBarcode,
    pub brands: Option<String>,
    pub img_url: Option<String>,
    pub thumb_url: Option<String>,
    pub quantity: Option<String>,
}

/// TODO?
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Item {
    pub id: i64,
    #[cfg_attr(feature = "ssr", sqlx(flatten))]
    pub data: ItemData,
}
*/

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ItemVariantImpl<ID> {
    pub id: ID,
    pub name: String,
    pub amount: i64,
    shop_id: i64, // TODO

    pub barcode: OptionBarcode,
    pub brands: Option<String>,
    pub img_url: Option<String>,
    pub thumb_url: Option<String>,
    pub quantity: Option<String>,
}

pub type ItemVariant = ItemVariantImpl<i64>;
pub type NewItemVariant = ItemVariantImpl<()>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ItemImpl<ID> {
    pub id: ID,
    pub amount: i64,
    pub completed: bool,
    pub variants: Vec<ItemVariantImpl<ID>>,
}

pub type Item = ItemImpl<i64>;
pub type NewItem = ItemImpl<()>;

impl Default for NewItem {
    fn default() -> Self {
        Self { id: (), amount: 1, completed: false, variants: vec![] }
    }
}

impl Default for NewItemVariant {
    fn default() -> Self {
        Self {
            id: (),
            amount: 1,
            name: "".to_string(),
            shop_id: 0,
            barcode: OptionBarcode::none(),
            brands: None,
            img_url: None,
            thumb_url: None,
            quantity: None,
        }
    }
}

impl NewItem {
    pub async fn from_barcode(barcode: Barcode) -> Result<Self> {
        let OpenFoodFactsProduct {
            product_name, image_url, image_thumb_url, brands, quantity, ..
        } = OpenFoodFactsProduct::request_with_barcode(barcode).await?;

        Ok(Self {
            variants: vec![NewItemVariant {
                name: product_name,
                barcode: OptionBarcode::some(barcode),
                img_url: image_url,
                thumb_url: image_thumb_url,
                brands: Some(brands),
                quantity: Some(quantity),
                ..NewItemVariant::default()
            }],
            ..Self::default()
        })
    }

    pub async fn insert(self) -> Result<Item> {
        todo!()
    }
}

/*
pub async fn add_item_from_barcode(barcode: Barcode) -> Result<(), ServerFnError> {
    pub async fn add_item_from_barcode(barcode: Barcode) -> Result<(), ServerFnError> {
        let ItemData { name, amount, barcode, img_url, thumb_url, .. } =
            ItemData::from_barcode(barcode).await?;

        //let mut conn = crate::db::DB::connection_from_context().await?;
        let mut conn = crate::db::db().await?;

        let _new_id = sqlx::query!(
            r#"
INSERT INTO items(name, amount, barcode, img_url, thumb_url)
VALUES ( ?, ?, ?, ?, ? )"#,
            name,
            amount,
            barcode,
            img_url,
            thumb_url
        )
        .execute(&mut conn)
        .await?
        .last_insert_rowid();

        Ok(())
    }
    add_item_from_barcode(barcode)
        .await
        .inspect_err(|err| eprintln!("ERROR (add_item_from_barcode): {}", err))
}
*/
