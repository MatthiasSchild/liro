use macros::make_repo;
use sea_orm::DbErr;

use crate::models::Page;

make_repo!(Stock, (
    async fn create_own(&self, _name: String) -> Result<entity::stocks::Model, DbErr> {
        todo!();
    }
    async fn create_for_contact(
        &self,
        _name: String,
        _contact_id: i32,
    ) -> Result<entity::stocks::Model, DbErr> {
        todo!();
    }

    async fn list_own(
        &self,
        _limit: u64,
        _offset: u64,
    ) -> Result<Page<entity::stocks::Model>, DbErr> {
        todo!();
    }

    async fn list_for_contact(
        &self,
        _contact_id: i32,
        _limit: u64,
        _offset: u64,
    ) -> Result<Page<entity::stocks::Model>, DbErr> {
        todo!();
    }

    async fn get(&self, _id: i32) -> Result<Option<entity::stocks::Model>, DbErr> {
        todo!();
    }

    async fn delete(&self, _id: i32) -> Result<bool, DbErr> {
        todo!();
    }
));
