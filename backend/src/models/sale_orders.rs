use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct SaleOrderModel {
    pub id: i32,
    pub reference: String,
    pub date: String,
    pub status: String,
    #[serde(rename = "customerId")]
    pub customer_id: i32,
    #[serde(rename = "sourceStockId")]
    pub source_stock_id: i32,
    #[serde(rename = "targetStockId")]
    pub target_stock_id: i32,
}

impl From<&entity::sale_orders::Model> for SaleOrderModel {
    fn from(entity: &entity::sale_orders::Model) -> Self {
        let status = match entity.status {
            entity::sea_orm_active_enums::SaleOrderStatus::Draft => "draft",
            entity::sea_orm_active_enums::SaleOrderStatus::Quotation => "quotation",
            entity::sea_orm_active_enums::SaleOrderStatus::Order => "order",
            entity::sea_orm_active_enums::SaleOrderStatus::Finished => "finished",
        };

        Self {
            id: entity.id,
            reference: entity.reference.clone(),
            date: entity.date.to_string(),
            status: status.to_string(),
            customer_id: entity.customer_id,
            source_stock_id: entity.source_stock_id,
            target_stock_id: entity.target_stock_id,
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct CreateSaleOrderInput {
    pub reference: String,
    pub date: String,
    pub status: String,
    #[serde(rename = "customerId")]
    pub customer_id: i32,
    #[serde(rename = "sourceStockId")]
    pub source_stock_id: i32,
    #[serde(rename = "targetStockId")]
    pub target_stock_id: i32,
}

#[derive(Deserialize, Validate)]
pub struct ListSaleOrdersInput {
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u64>,
    #[validate(range(min = 0))]
    pub offset: Option<u64>,
}
