use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize)]
pub struct LedgerTransactionModel {
    pub id: i32,
    #[serde(rename = "debitAccountId")]
    pub debit_account_id: i32,
    #[serde(rename = "creditAccountId")]
    pub credit_account_id: i32,
    pub date: String,
    pub amount: i32,
}

impl From<&entity::ledger_transactions::Model> for LedgerTransactionModel {
    fn from(entity: &entity::ledger_transactions::Model) -> Self {
        Self {
            id: entity.id,
            debit_account_id: entity.debit_account_id,
            credit_account_id: entity.credit_account_id,
            date: entity.date.format("%Y-%m-%d").to_string(),
            amount: entity.amount,
        }
    }
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateLedgerTransactionInput {
    #[serde(rename = "debitAccountId")]
    pub debit_account_id: i32,
    #[serde(rename = "creditAccountId")]
    pub credit_account_id: i32,
    pub date: String,
    pub amount: i32,
}

#[derive(Deserialize, Validate)]
pub struct ListLedgerTransactionsInput {
    #[serde(rename = "account")]
    pub account_id: i32,
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u64>,
    #[validate(range(min = 0))]
    pub offset: Option<u64>,
}
