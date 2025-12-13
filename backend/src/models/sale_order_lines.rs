use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct SaleOrderLineModel {
    pub id: i32,
    pub description: String,
    pub quantity: f64,
    pub price: i32,
    pub tax: i32,
    #[serde(rename = "orderId")]
    pub order_id: i32,
    #[serde(rename = "variantId")]
    pub variant_id: Option<i32>,
    #[serde(rename = "moveId")]
    pub move_id: Option<i32>,
}

impl From<&entity::sale_order_lines::Model> for SaleOrderLineModel {
    fn from(entity: &entity::sale_order_lines::Model) -> Self {
        Self {
            id: entity.id,
            description: entity.description.clone(),
            quantity: entity.quantity,
            price: entity.price,
            tax: entity.tax,
            order_id: entity.order_id,
            variant_id: entity.variant_id,
            move_id: entity.move_id,
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct CreateSaleOrderLineInput {
    pub description: String,
    pub quantity: f64,
    pub price: i32,
    pub tax: i32,
    #[serde(rename = "orderId")]
    pub order_id: i32,
    #[serde(rename = "variantId")]
    pub variant_id: Option<i32>,
}

#[derive(Deserialize, Validate)]
pub struct ListSaleOrderLinesInput {
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u64>,
    #[validate(range(min = 0))]
    pub offset: Option<u64>,
}
