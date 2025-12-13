use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct PurchaseInvoiceModel {
    pub id: i32,
    pub reference: String,
    pub date: String,
    pub status: String,
    #[serde(rename = "orderId")]
    pub order_id: i32,
    #[serde(rename = "supplierId")]
    pub supplier_id: i32,
}

impl From<&entity::purchase_invoices::Model> for PurchaseInvoiceModel {
    fn from(entity: &entity::purchase_invoices::Model) -> Self {
        let status = match entity.status {
            entity::sea_orm_active_enums::PurchaseInvoiceStatus::Draft => "draft",
            entity::sea_orm_active_enums::PurchaseInvoiceStatus::Finished => "finished",
        };

        Self {
            id: entity.id,
            reference: entity.reference.clone(),
            date: entity.date.to_string(),
            status: status.to_string(),
            order_id: entity.order_id,
            supplier_id: entity.supplier_id,
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct CreatePurchaseInvoiceInput {
    pub reference: String,
    pub date: String,
    #[serde(rename = "orderId")]
    pub order_id: i32,
    #[serde(rename = "supplierId")]
    pub supplier_id: i32,
}

#[derive(Deserialize, Validate)]
pub struct ListPurchaseInvoiceInput {
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u64>,
    #[validate(range(min = 0))]
    pub offset: Option<u64>,
}
