use macros::make_repo;
use sea_orm::DbErr;

use crate::models::Page;

make_repo!(Product, (
    async fn create(&self, _name: String) -> Result<entity::products::Model, DbErr> {
        todo!();
    }

    async fn list(
        &self,
        _limit: u64,
        _offset: u64,
    ) -> Result<Page<entity::products::Model>, DbErr> {
        todo!();
    }

    async fn get(&self, _id: i32) -> Result<Option<entity::products::Model>, DbErr> {
        todo!();
    }

    async fn delete(&self, _id: i32) -> Result<bool, DbErr> {
        todo!();
    }
));
