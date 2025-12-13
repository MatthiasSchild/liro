use actix_web::{HttpResponse, HttpResponseBuilder, http::StatusCode};
use serde_json::json;

pub const MESSAGE_ACCOUNT_NOT_FOUND: &str = "Ledger account could not be found";
pub const MESSAGE_DATABASE_UNREACHABLE: &str = "The database is currently unreachable";
pub const MESSAGE_INTERNAL_SERVER_ERROR: &str = "Internal server error";
pub const MESSAGE_INVALID_ACCOUNT_TYPE: &str = "Invalid account type. Must be asset, liability, income or expense";
pub const MESSAGE_VARIANT_NOT_FOUND: &str = "Variant not found";

pub const CODE_ACCOUNT_NOT_FOUND: &str = "ACCOUNT_NOT_FOUND";
pub const CODE_DATABASE_UNREACHABLE: &str = "DATABASE_UNREACHABLE";
pub const CODE_INTERNAL_SERVER_ERROR: &str = "INTERNAL_SERVER_ERROR";
pub const CODE_INVALID_ACCOUNT_TYPE: &str = "INVALID_ACCOUNT_TYPE";
pub const CODE_VARIANT_NOT_FOUND: &str = "VARIANT_NOT_FOUND";

pub const STATUS_ACCOUNT_NOT_FOUND: StatusCode = StatusCode::NOT_FOUND;
pub const STATUS_DATABASE_UNREACHABLE: StatusCode = StatusCode::SERVICE_UNAVAILABLE;
pub const STATUS_INTERNAL_SERVER_ERROR: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
pub const STATUS_INVALID_ACCOUNT_TYPE: StatusCode = StatusCode::BAD_REQUEST;
pub const STATUS_VARIANT_NOT_FOUND: StatusCode = StatusCode::NOT_FOUND;

pub enum ApiErrors {
    AccountNotFound,
    DatabaseUnreachable,
    InternalServerError,
    InvalidAccountType,
    VariantNotFound,
}

impl ApiErrors {
    pub fn status(&self) -> StatusCode {
        match self {
            ApiErrors::AccountNotFound => STATUS_ACCOUNT_NOT_FOUND,
            ApiErrors::DatabaseUnreachable => STATUS_DATABASE_UNREACHABLE,
            ApiErrors::InternalServerError => STATUS_INTERNAL_SERVER_ERROR,
            ApiErrors::InvalidAccountType => STATUS_INVALID_ACCOUNT_TYPE,
            ApiErrors::VariantNotFound => STATUS_VARIANT_NOT_FOUND,
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            ApiErrors::AccountNotFound => MESSAGE_ACCOUNT_NOT_FOUND,
            ApiErrors::DatabaseUnreachable => MESSAGE_DATABASE_UNREACHABLE,
            ApiErrors::InternalServerError => MESSAGE_INTERNAL_SERVER_ERROR,
            ApiErrors::InvalidAccountType => MESSAGE_INVALID_ACCOUNT_TYPE,
            ApiErrors::VariantNotFound => MESSAGE_VARIANT_NOT_FOUND,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            ApiErrors::AccountNotFound => CODE_ACCOUNT_NOT_FOUND,
            ApiErrors::DatabaseUnreachable => CODE_DATABASE_UNREACHABLE,
            ApiErrors::InternalServerError => CODE_INTERNAL_SERVER_ERROR,
            ApiErrors::InvalidAccountType => CODE_INVALID_ACCOUNT_TYPE,
            ApiErrors::VariantNotFound => CODE_VARIANT_NOT_FOUND,
        }
    }

    pub fn json(&self) -> serde_json::Value {
        json!({
            "error": self.message(),
            "errorCode": self.code(),
        })
    }
}

impl Into<HttpResponse> for ApiErrors {
    fn into(self) -> HttpResponse {
        HttpResponseBuilder::new(self.status()).json(self.json())
    }
}
