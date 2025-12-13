use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize)]
pub struct LedgerAccountModel {
    pub id: i32,
    #[serde(rename = "type")]
    pub account_type: String,
    pub name: String,
}

impl From<&entity::ledger_accounts::Model> for LedgerAccountModel {
    fn from(entity: &entity::ledger_accounts::Model) -> Self {
        Self {
            id: entity.id,
            account_type: entity.account_type.clone(),
            name: entity.name.clone(),
        }
    }
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateLedgerAccountInput {
    #[serde(rename = "type")]
    #[validate(length(min = 1))]
    pub account_type: String,
    #[validate(length(min = 1))]
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct ListLedgerAccountsInput {
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u64>,
    #[validate(range(min = 0))]
    pub offset: Option<u64>,
}
