use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct PurchaseInvoiceLineModel {
    pub id: i32,
    pub description: String,
    pub quantity: f64,
    pub price: i32,
    pub tax: i32,
    #[serde(rename = "invoiceId")]
    pub invoice_id: i32,
    #[serde(rename = "variantId")]
    pub variant_id: Option<i32>,
    #[serde(rename = "transactionId")]
    pub transaction_id: Option<i32>,
    #[serde(rename = "taxTransactionId")]
    pub tax_transaction_id: Option<i32>,
}

impl From<&entity::purchase_invoice_lines::Model> for PurchaseInvoiceLineModel {
    fn from(entity: &entity::purchase_invoice_lines::Model) -> Self {
        Self {
            id: entity.id,
            description: entity.description.clone(),
            quantity: entity.quantity,
            price: entity.price,
            tax: entity.tax,
            invoice_id: entity.invoice_id,
            variant_id: entity.variant_id,
            transaction_id: entity.transaction_id,
            tax_transaction_id: entity.tax_transaction_id,
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct CreatePurchaseInvoiceLineInput {
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
pub struct ListPurchaseInvoiceLinesInput {
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u64>,
    #[validate(range(min = 0))]
    pub offset: Option<u64>,
}
