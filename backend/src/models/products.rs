use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize)]
pub struct ProductModel {
    pub id: i32,
    pub name: String,
}

impl From<&entity::products::Model> for ProductModel {
    fn from(entity: &entity::products::Model) -> Self {
        Self {
            id: entity.id,
            name: entity.name.clone(),
        }
    }
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateProductInput {
    #[validate(length(min = 1, max = 32))]
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct ListProductsInput {
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u64>,
    #[validate(range(min = 0))]
    pub offset: Option<u64>,
}
