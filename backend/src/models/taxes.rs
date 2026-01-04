use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize)]
pub struct TaxModel {
    pub id: i32,
    pub name: String,
    #[serde(rename = "nameShort")]
    pub name_short: String,
    pub rate: f64,
    #[serde(rename = "accountId")]
    pub account_id: i32,
}

impl From<&entity::taxes::Model> for TaxModel {
    fn from(entity: &entity::taxes::Model) -> Self {
        Self {
            id: entity.id,
            name: entity.name.clone(),
            name_short: entity.name_short.clone(),
            rate: entity.rate,
            account_id: entity.account_id,
        }
    }
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateTaxInput {
    #[validate(length(min = 1, max = 32))]
    pub name: String,
    #[serde(rename = "nameShort")]
    #[validate(length(min = 1, max = 32))]
    pub name_short: String,
    #[validate(range(min = 0f64))]
    pub rate: f64,
    #[serde(rename = "account")]
    pub account_id: i32,
}

#[derive(Deserialize, Validate)]
pub struct ListTaxesInput {
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u64>,
    #[validate(range(min = 0))]
    pub offset: Option<u64>,
}
