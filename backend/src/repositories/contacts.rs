use macros::make_repo;
use sea_orm::DbErr;

use crate::models::Page;

make_repo!(Contact, (
    async fn create_customer(
        &self,
        _name: String,
        _account_id: i32,
        _street1: String,
        _street2: String,
        _postal_code: String,
        _city: String,
        _county: String,
    ) -> Result<entity::contacts::Model, DbErr> {
        todo!()
    }

    async fn create_supplier(
        &self,
        _name: String,
        _account_id: i32,
        _street1: String,
        _street2: String,
        _postal_code: String,
        _city: String,
        _county: String,
    ) -> Result<entity::contacts::Model, DbErr> {
        todo!()
    }

    async fn list_customers(
        &self,
        _limit: u64,
        _offset: u64,
    ) -> Result<Page<entity::contacts::Model>, DbErr> {
        todo!()
    }

    async fn list_suppliers(
        &self,
        _limit: u64,
        _offset: u64,
    ) -> Result<Page<entity::contacts::Model>, DbErr> {
        todo!()
    }

    async fn get(&self, _id: i32) -> Result<Option<entity::contacts::Model>, DbErr> {
        todo!()
    }

    async fn delete(&self, _id: i32) -> Result<bool, DbErr> {
        todo!()
    }
));
