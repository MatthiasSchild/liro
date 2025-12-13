use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize)]
pub struct UserModel {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

impl From<&entity::users::Model> for UserModel {
    fn from(entity: &entity::users::Model) -> Self {
        Self {
            id: entity.id,
            email: entity.email.clone(),
            first_name: entity.first_name.clone(),
            last_name: entity.last_name.clone(),
        }
    }
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateUserInput {
    #[validate(length(min = 1, max = 64))]
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, max = 64))]
    pub password: String,
    #[serde(rename = "firstName")]
    #[validate(length(min = 1, max = 64))]
    pub first_name: String,
    #[serde(rename = "lastName")]
    #[validate(length(min = 1, max = 64))]
    pub last_name: String,
}

#[derive(Deserialize, Validate)]
pub struct ListUsersInput {
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u64>,
    #[validate(range(min = 0))]
    pub offset: Option<u64>,
}
