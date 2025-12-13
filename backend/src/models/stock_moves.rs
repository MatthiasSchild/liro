use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct StockMoveModel {
    pub id: i32,
    pub quantity: f64,
    #[serde(rename = "variantId")]
    pub variant_id: i32,
    #[serde(rename = "sourceStockId")]
    pub source_stock_id: i32,
    #[serde(rename = "targetStockId")]
    pub target_stock_id: i32,
}

impl From<&entity::stock_moves::Model> for StockMoveModel {
    fn from(entity: &entity::stock_moves::Model) -> Self {
        Self {
            id: entity.id,
            quantity: entity.quantity,
            variant_id: entity.variant_id,
            source_stock_id: entity.source_stock_id,
            target_stock_id: entity.target_stock_id,
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct CreateStockMoveInput {
    pub quantity: f64,
    pub variant_id: i32,
    pub source_stock_id: i32,
    pub target_stock_id: i32,
}

#[derive(Deserialize, Validate)]
pub struct ListStockMovesInput {
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u64>,
    #[validate(range(min = 0))]
    pub offset: Option<u64>,
}
