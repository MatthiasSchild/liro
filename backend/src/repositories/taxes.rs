use macros::make_repo;
use sea_orm::DbErr;

use crate::models::Page;

make_repo!(Tax, (
    async fn create(
        &self,
        _name: String,
        _name_short: String,
        _rate: f64,
        _account_id: i32,
    ) -> Result<entity::taxes::Model, DbErr> {
        todo!();
    }

    async fn list(&self, _limit: u64, _offset: u64) -> Result<Page<entity::taxes::Model>, DbErr> {
        todo!();
    }

    async fn get(&self, _id: i32) -> Result<Option<entity::taxes::Model>, DbErr> {
        todo!();
    }

    async fn delete(&self, _id: i32) -> Result<bool, DbErr> {
        todo!();
    }
));
