use entity::sea_orm_active_enums::SaleInvoiceStatus;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct SaleInvoiceModel {
    pub id: i32,
    pub reference: String,
    pub date: String,
    pub status: String,
    #[serde(rename = "orderId")]
    pub order_id: i32,
    #[serde(rename = "customerId")]
    pub customer_id: i32,
}

impl From<&entity::sale_invoices::Model> for SaleInvoiceModel {
    fn from(entity: &entity::sale_invoices::Model) -> Self {
        let status = match entity.status {
            SaleInvoiceStatus::Draft => "draft",
            SaleInvoiceStatus::Proforma => "proforma",
            SaleInvoiceStatus::Invoice => "invoice",
            SaleInvoiceStatus::Finished => "finished",
        };

        Self {
            id: entity.id,
            reference: entity.reference.clone(),
            date: entity.date.to_string(),
            status: status.to_string(),
            order_id: entity.order_id,
            customer_id: entity.customer_id,
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct CreateSaleInvoiceInput {
    pub reference: String,
    pub date: String,
    #[serde(rename = "orderId")]
    pub order_id: i32,
    #[serde(rename = "customerId")]
    pub customer_id: i32,
}

#[derive(Deserialize, Validate)]
pub struct ListSaleInvoiceInput {
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u64>,
    #[validate(range(min = 0))]
    pub offset: Option<u64>,
}
