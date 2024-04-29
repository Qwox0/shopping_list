use crate::{
    barcode_scanner::Barcode,
    error::{Error, Result},
};
use leptos::logging;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct OpenFoodFactsProduct {
    pub product_name: String,
    // product_name_de: Option<String>,
    pub brands: String,
    pub quantity: String,
    // product_quantity: Option<String>,
    pub image_url: String,
    pub image_thumb_url: String,
    pub packaging: String,
    // nutrient_levels: Option<Nutriments>,
    pub nutriments: Nutriments,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Nutriments {
    carbohydrates: f32,
    carbohydrates_100g: f32,
    carbohydrates_unit: String,
    carbohydrates_value: f32,
    energy: u32,
    #[serde(rename = "energy-kcal")]
    energy_kcal: u32,
    #[serde(rename = "energy-kcal_100g")]
    energy_kcal_100g: u32,
    #[serde(rename = "energy-kcal_unit")]
    energy_kcal_unit: String,
    #[serde(rename = "energy-kcal_value")]
    energy_kcal_value: u32,
    #[serde(rename = "energy-kcal_value_computed")]
    energy_kcal_value_computed: f32,
    energy_100g: u32,
    energy_unit: String,
    energy_value: u32,
    // fat: u32,
    // fat_100g: u32,
    // fat_unit: String,
    // fat_value: u32,
    // fiber: f32,
    // fiber_100g: f32,
    // fiber_unit: String,
    // fiber_value: f32,
    // #[serde(rename = "nutrition-score-fr")]
    // nutrition_score_fr: u32,
    // #[serde(rename = "nutrition-score-fr_100g")]
    // nutrition_score_fr_100g: u32,
    proteins: f32,
    proteins_100g: f32,
    proteins_unit: String,
    proteins_value: f32,
    salt: f32,
    salt_100g: f32,
    salt_unit: String,
    salt_value: f32,
    // #[serde(rename = "saturated-fat")]
    // saturated_fat: f32,
    // #[serde(rename = "saturated-fat_100g")]
    // saturated_fat_100g: f32,
    // #[serde(rename = "saturated-fat_unit")]
    // saturated_fat_unit: String,
    // #[serde(rename = "saturated-fat_value")]
    // saturated_fat_value: f32,
    sodium: f32,
    sodium_100g: f32,
    sodium_unit: String,
    sodium_value: f32,
    sugars: f32,
    sugars_100g: f32,
    sugars_unit: String,
    sugars_value: f32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct NutrientLevels {
    fat: String,
    salt: String,
    #[serde(rename = "saturated-fat")]
    saturated_fat: String,
    sugars: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct OpenFoodFactsResponse {
    code: String,
    product: Option<serde_json::Value>,
    status: u8,
    status_verbose: String,
}

impl Display for OpenFoodFactsResponse {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let is_alternate = f.alternate();
        let mut f = f.debug_struct("OpenFoodFactsResponse");
        let f = f
            .field("code", &format_args!("{}", self.code))
            .field("status", &self.status)
            .field("status_verbose", &self.status);
        match &self.product {
            Some(product) if is_alternate => f.field("product", &format_args!("{:#}", product)),
            Some(product) => f.field("product", &format_args!("{}", product)),
            None => f,
        }
        .finish()
    }
}

/// this functions returns an [`OpenFoodFactsResponse`]. If you want to parse
/// the response use [`OpenFoodFactsProduct::request_with_barcode`].
pub async fn request_with_barcode(barcode: Barcode) -> Result<serde_json::Value> {
    const OK_STATUS: u8 = 1;

    let url =
        format!("https://world.openfoodfacts.org/api/v0/product/{}.json", barcode);
    let res = reqwest::get(url).await?.json::<OpenFoodFactsResponse>().await?;

    match res {
        OpenFoodFactsResponse { product: Some(p), status: OK_STATUS, .. } => Ok(p),
        _ => {
            logging::error!("Error with OpenFoodFacts: {}", res.status_verbose);
            Err(Error::DidntFindProduct)
        },
    }
}

impl OpenFoodFactsProduct {
    pub async fn request_with_barcode(barcode: Barcode) -> Result<Self> {
        serde_json::from_value(request_with_barcode(barcode).await?).map_err(Into::into)
    }
}
