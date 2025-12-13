use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct PurchaseOrderModel {
    pub id: i32,
    pub reference: String,
    pub date: String,
    pub status: String,
    #[serde(rename = "supplierId")]
    pub supplier_id: i32,
    #[serde(rename = "sourceStockId")]
    pub source_stock_id: i32,
    #[serde(rename = "targetStockId")]
    pub target_stock_id: i32,
}

impl From<&entity::purchase_orders::Model> for PurchaseOrderModel {
    fn from(entity: &entity::purchase_orders::Model) -> Self {
        let status = match entity.status {
            entity::sea_orm_active_enums::PurchaseOrderStatus::Draft => "draft",
            entity::sea_orm_active_enums::PurchaseOrderStatus::Finished => "finished",
        };

        Self {
            id: entity.id,
            reference: entity.reference.clone(),
            date: entity.date.to_string(),
            status: status.to_string(),
            supplier_id: entity.supplier_id,
            source_stock_id: entity.source_stock_id,
            target_stock_id: entity.target_stock_id,
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct CreatePurchaseOrderInput {
    pub reference: String,
    pub date: String,
    pub status: String,
    #[serde(rename = "supplierId")]
    pub supplier_id: i32,
    #[serde(rename = "sourceStockId")]
    pub source_stock_id: i32,
    #[serde(rename = "targetStockId")]
    pub target_stock_id: i32,
}

#[derive(Deserialize, Validate)]
pub struct ListPurchaseOrdersInput {
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u64>,
    #[validate(range(min = 0))]
    pub offset: Option<u64>,
}
