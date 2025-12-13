use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, ToSchema)]
pub struct ContactModel {
    pub id: i32,
    pub name: String,
    #[serde(rename = "isCustomer")]
    pub is_customer: bool,
    #[serde(rename = "isSupplier")]
    pub is_supplier: bool,
    #[serde(rename = "customerAccountId")]
    pub customer_account_id: Option<i32>,
    #[serde(rename = "supplierAccountId")]
    pub supplier_account_id: Option<i32>,
    pub street1: String,
    pub street2: String,
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    pub city: String,
    pub country: String,
}

impl From<&entity::contacts::Model> for ContactModel {
    fn from(entity: &entity::contacts::Model) -> Self {
        Self {
            id: entity.id,
            name: entity.name.clone(),
            is_customer: entity.is_customer,
            is_supplier: entity.is_supplier,
            customer_account_id: entity.customer_account_id,
            supplier_account_id: entity.supplier_account_id,
            street1: entity.street1.clone(),
            street2: entity.street2.clone(),
            postal_code: entity.postal_code.clone(),
            city: entity.city.clone(),
            country: entity.country.clone(),
        }
    }
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateContactInput {
    #[validate(length(min = 1, max = 128))]
    pub name: String,
    #[validate(range(min = 1))]
    #[serde(rename = "accountId")]
    pub account_id: i32,
    #[validate(length(max = 128))]
    pub street1: String,
    #[validate(length(max = 128))]
    pub street2: String,
    #[validate(length(max = 128))]
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    #[validate(length(max = 128))]
    pub city: String,
    #[validate(length(max = 128))]
    pub country: String,
}

#[derive(Deserialize, Validate)]
pub struct ListContactsInput {
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u64>,
    #[validate(range(min = 0))]
    pub offset: Option<u64>,
}
