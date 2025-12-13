use macros::make_repo;
use sea_orm::DbErr;

use crate::models::Page;

make_repo!(User, (
    async fn create(
        &self,
        _email: String,
        _pass_hash: String,
        _first_name: String,
        _last_name: String,
    ) -> Result<entity::users::Model, DbErr> {
        todo!();
    }

    async fn list(&self, _limit: u64, _offset: u64) -> Result<Page<entity::users::Model>, DbErr> {
        todo!();
    }

    async fn get(&self, _id: i32) -> Result<Option<entity::users::Model>, DbErr> {
        todo!();
    }

    async fn get_by_email(&self, _email: String) -> Result<Option<entity::users::Model>, DbErr> {
        todo!();
    }

    async fn delete(&self, _id: i32) -> Result<bool, DbErr> {
        todo!();
    }
));
