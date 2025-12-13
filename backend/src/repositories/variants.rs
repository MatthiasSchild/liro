use macros::make_repo;
use sea_orm::DbErr;

use crate::models::Page;

make_repo!(Variant, (
    async fn create(
        &self,
        _product_id: i32,
        _name: String,
        _sale_price: i32,
        _purchase_price: i32,
    ) -> Result<entity::variants::Model, DbErr> {
        todo!();
    }

    async fn list(
        &self,
        _product_id: i32,
        _limit: u64,
        _offset: u64,
    ) -> Result<Page<entity::variants::Model>, DbErr> {
        todo!();
    }

    async fn get(&self, _id: i32) -> Result<Option<entity::variants::Model>, DbErr> {
        todo!();
    }

    async fn delete(&self, _id: i32) -> Result<bool, DbErr> {
        todo!();
    }
));
