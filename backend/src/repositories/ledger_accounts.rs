use entity::prelude::LedgerAccounts;
use macros::make_repo;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DbErr, EntityTrait, PaginatorTrait, QuerySelect,
};

use crate::models::Page;

make_repo!(LedgerAccount, (
    async fn create(
        &self,
        account_type: String,
        name: String,
    ) -> Result<entity::ledger_accounts::Model, DbErr> {
        let entity = entity::ledger_accounts::ActiveModel {
            account_type: Set(account_type),
            name: Set(name),
            ..Default::default()
        };

        match entity.insert(&self.db).await {
            Ok(entity) => Ok(entity),
            Err(err) => Err(err),
        }
    }

    async fn list(
        &self,
        limit: u64,
        offset: u64,
    ) -> Result<Page<entity::ledger_accounts::Model>, DbErr> {
        let query = LedgerAccounts::find()
            .limit(limit)
            .offset(offset)
            .count(&self.db);

        let total = match query.await {
            Ok(total) => total,
            Err(err) => return Err(err),
        };

        let query = LedgerAccounts::find()
            .limit(limit)
            .offset(offset)
            .all(&self.db);

        match query.await {
            Ok(entities) => Ok(Page {
                limit: limit,
                offset: offset,
                total,
                data: entities,
            }),
            Err(err) => Err(err),
        }
    }

    async fn get(&self, _id: i32) -> Result<Option<entity::ledger_accounts::Model>, DbErr> {
        todo!();
    }

    async fn delete(&self, id: i32) -> Result<bool, DbErr> {
        let query = LedgerAccounts::delete_by_id(id).exec(&self.db);
        let found = match query.await {
            Ok(result) => result.rows_affected > 0,
            Err(err) => return Err(err),
        };

        Ok(found)
    }
));
