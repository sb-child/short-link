use axum::http::StatusCode;
use sea_orm::DbErr;
use thiserror::Error;
use tokio::task::JoinError;

use super::{RequestError, ToError};

#[derive(Error, Debug)]
pub enum ChallengeError {
    #[error("database error")]
    DatabaseError(#[from] DbErr),
    #[error("task error")]
    TaskError(#[from] JoinError),
    #[error("generator error")]
    GeneratorError(),
    #[error("invalid payload")]
    InvalidPayload(),
    #[error("challenge failed")]
    ChallengeFailed(),
    #[error("invalid token")]
    InvalidToken(),
}

impl ToError for ChallengeError {
    fn to_error(&self) -> RequestError {
        match self {
            ChallengeError::DatabaseError(x) => RequestError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                cause: Some(x.to_string()),
                error: "database error".to_string(),
                error_message: "数据库错误".to_string(),
            },
            ChallengeError::TaskError(x) => RequestError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                cause: Some(x.to_string()),
                error: "task error".to_string(),
                error_message: "任务失败".to_string(),
            },
            ChallengeError::GeneratorError() => RequestError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                cause: None,
                error: "generator error".to_string(),
                error_message: "生成失败".to_string(),
            },
            ChallengeError::InvalidPayload() => RequestError {
                status_code: StatusCode::FORBIDDEN,
                cause: None,
                error: "invalid payload".to_string(),
                error_message: "验证数据无效".to_string(),
            },
            ChallengeError::InvalidToken() => RequestError {
                status_code: StatusCode::FORBIDDEN,
                cause: None,
                error: "invalid token".to_string(),
                error_message: "令牌无效".to_string(),
            },
            ChallengeError::ChallengeFailed() => RequestError {
                status_code: StatusCode::FORBIDDEN,
                cause: None,
                error: "challenge failed".to_string(),
                error_message: "挑战失败".to_string(),
            },
        }
    }
}
