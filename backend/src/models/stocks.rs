use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize)]
pub struct StockModel {
    pub id: i32,
    pub name: String,
    #[serde(rename = "own")]
    pub is_own: bool,
    #[serde(rename = "owner")]
    pub owner_id: Option<i32>,
}

impl From<&entity::stocks::Model> for StockModel {
    fn from(entity: &entity::stocks::Model) -> Self {
        Self {
            id: entity.id,
            name: entity.name.to_string(),
            is_own: entity.is_own,
            owner_id: entity.owner_id,
        }
    }
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateStockInput {
    #[validate(length(min = 1))]
    pub name: String,
}
