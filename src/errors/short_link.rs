use axum::http::StatusCode;
use sea_orm::DbErr;
use thiserror::Error;

use super::{RequestError, ToError};

#[derive(Error, Debug)]
pub enum ShortLinkError {
    #[error("database error")]
    DatabaseError(#[from] DbErr),
    #[error("short link not exist")]
    Invalid(),
    #[error("invalid token")]
    InvalidToken(),
    #[error("short link exists")]
    Exists(),
    #[error("field required")]
    FieldRequired(),
}

impl ToError for ShortLinkError {
    fn to_error(&self) -> RequestError {
        match self {
            ShortLinkError::DatabaseError(x) => RequestError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                cause: Some(x.to_string()),
                error: "database error".to_string(),
                error_message: "数据库错误".to_string(),
            },
            ShortLinkError::Invalid() => RequestError {
                status_code: StatusCode::NOT_ACCEPTABLE,
                cause: None,
                error: "short link not exist".to_string(),
                error_message: "找不到短链接".to_string(),
            },
            ShortLinkError::InvalidToken() => RequestError {
                status_code: StatusCode::FORBIDDEN,
                cause: None,
                error: "invalid token".to_string(),
                error_message: "令牌无效".to_string(),
            },
            ShortLinkError::Exists() => RequestError {
                status_code: StatusCode::NOT_ACCEPTABLE,
                cause: None,
                error: "short link exists".to_string(),
                error_message: "短链接已存在".to_string(),
            },
            ShortLinkError::FieldRequired() => RequestError {
                status_code: StatusCode::NOT_ACCEPTABLE,
                cause: None,
                error: "field required".to_string(),
                error_message: "缺少必要的字段".to_string(),
            },
        }
    }
}
