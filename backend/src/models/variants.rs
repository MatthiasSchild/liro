use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize)]
pub struct VariantModel {
    pub id: i32,
    pub name: String,
    #[serde(rename = "salePrice")]
    pub sale_price: i32,
    #[serde(rename = "purchasePrice")]
    pub purchase_price: i32,
    #[serde(rename = "productId")]
    pub product_id: i32,
}

impl From<&entity::variants::Model> for VariantModel {
    fn from(entity: &entity::variants::Model) -> Self {
        Self {
            id: entity.id,
            name: entity.name.clone(),
            sale_price: entity.sale_price,
            purchase_price: entity.purchase_price,
            product_id: entity.product_id,
        }
    }
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct VariantsPath {
    #[validate(range(min = 1))]
    pub id: i32,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateVariantInput {
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    #[serde(rename = "salePrice")]
    #[validate(range(min = 0))]
    pub sale_price: i32,
    #[serde(rename = "purchasePrice")]
    #[validate(range(min = 0))]
    pub purchase_price: i32,
    #[serde(rename = "productId")]
    #[validate(range(min = 1))]
    pub product_id: i32,
}

#[derive(Deserialize, Validate)]
pub struct ListVariantsInput {
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u64>,
    #[validate(range(min = 0))]
    pub offset: Option<u64>,
}
