use macros::make_repo;
use sea_orm::DbErr;

use crate::models::Page;

make_repo!(LedgerTransaction, (
    async fn create(
        &self,
        _debit_account: i32,
        _credit_account: i32,
        _date: String,
        _amount: i32,
    ) -> Result<entity::ledger_transactions::Model, DbErr> {
        todo!();
    }

    async fn list(
        &self,
        _ledger_account_id: i32,
        _limit: u64,
        _offset: u64,
    ) -> Result<Page<entity::ledger_transactions::Model>, DbErr> {
        todo!();
    }

    async fn get(&self, _id: i32) -> Result<Option<entity::ledger_transactions::Model>, DbErr> {
        todo!();
    }

    async fn delete(&self, _id: i32) -> Result<bool, DbErr> {
        todo!();
    }
));
