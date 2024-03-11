use crate::{barcode::Barcode, image::Image};
use leptos::{
    component,
    leptos_dom::logging::{console_error, console_log},
    view, IntoView,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    pub(crate) db_id: usize,
    barcode: Barcode,
    name: String,
    img_url: String,
    thumb_url: String,
    count: usize,
}

#[component]
pub fn ItemView(item: Item) -> impl IntoView {
    view! {
        <li class="item">
            <input type="checkbox"/>
            <Image thumb_url=item.thumb_url full_url=item.img_url/>
            <span>{ item.name }</span>
        </li>
    }
}

impl Item {
    pub async fn from_openfoodsfacts(barcode: Barcode) -> Result<Item, Error> {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct OpenFoodFactsResponse {
            code: String,
            product: serde_json::Value,
            status: u8,
            status_verbose: String,
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct OpenFoodFactsProduct {
            product_name: String,
            product_name_de: Option<String>,
            image_url: String,
            image_thumb_url: String,
        }
        const OK_STATUS: u8 = 1;

        let url =
            format!("https://world.openfoodfacts.org/api/v0/product/{}.json", barcode.get_digits());
        let res = reqwest::get(url).await?.json::<OpenFoodFactsResponse>().await?;

        if res.status != OK_STATUS {
            console_error(&format!("Error with OpenFoodFacts: {}", res.status_verbose));
            return Err(Error::DidntFindProduct);
        }
        let product: OpenFoodFactsProduct = serde_json::from_value(res.product)?;

        Ok(Item {
            db_id: 0,
            barcode,
            count: 1,
            name: product.product_name,
            img_url: product.image_url,
            thumb_url: product.image_thumb_url,
        })
    }

    pub async fn from_barcode(barcode: Barcode) -> Result<Item, Error> {
        Item::from_openfoodsfacts(barcode).await
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error("didn't find product")]
    DidntFindProduct,

    #[error("missing \"{}\" field on product", .0)]
    MissingProductField(&'static str),
}
