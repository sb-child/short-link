use axum::extract::{Path, State};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, TransactionTrait};

use crate::{
    config::ServiceConfig,
    errors::{challenge::ChallengeError, ErrorResponse, ToError},
    utils,
};

pub async fn create(
    State((db, sv)): State<(DatabaseConnection, ServiceConfig)>,
) -> Result<String, ErrorResponse> {
    let payload = utils::random_string(64);
    let now = chrono::Utc::now().naive_utc();
    let outdated = now
        .checked_sub_signed(chrono::Duration::try_seconds(sv.challenge_timeout).unwrap())
        .unwrap();
    match db
        .transaction(|ts| {
            let now = now;
            let outdated = outdated;
            let payload = payload.clone();
            Box::pin(async move {
                entity::challenge::Entity::delete_many()
                    .filter(entity::challenge::Column::CreatedAt.lt(outdated))
                    .exec(ts)
                    .await?;
                entity::challenge::Entity::insert(entity::challenge::ActiveModel {
                    payload: sea_orm::ActiveValue::Set(payload),
                    created_at: sea_orm::ActiveValue::Set(now),
                })
                .exec(ts)
                .await?;
                Ok::<(), DbErr>(())
            })
        })
        .await
    {
        Ok(_) => {}
        Err(e) => {
            return match e {
                sea_orm::TransactionError::Connection(e) => {
                    Err(ChallengeError::DatabaseError(e).to_error().to_response())
                }
                sea_orm::TransactionError::Transaction(_e) => {
                    Err(ChallengeError::GeneratorError().to_error().to_response())
                }
            }
        }
    };
    Ok(payload)
}

pub async fn verify(
    State((db, sv)): State<(DatabaseConnection, ServiceConfig)>,
    Path((payload, hash)): Path<(String, String)>,
) -> Result<String, ErrorResponse> {
    let now = chrono::Utc::now().naive_utc();
    let challenge_outdated = now
        .checked_sub_signed(chrono::Duration::try_seconds(sv.challenge_timeout).unwrap())
        .unwrap();
    let token_outdated = now
        .checked_sub_signed(chrono::Duration::try_seconds(sv.token_timeout).unwrap())
        .unwrap();

    match tokio::task::spawn_blocking({
        let payload = payload.clone();
        let secret = sv.secret.clone();
        move || utils::verify_challenge(payload, secret, hash)
    })
    .await
    {
        Ok(x) => {
            if !x {
                return Err(ChallengeError::ChallengeFailed().to_error().to_response());
            }
        }
        Err(e) => {
            return Err(ChallengeError::TaskError(e).to_error().to_response());
        }
    }
    let nonce = utils::random_string(64);
    // let token = utils::generate_token(nonce, sv.secret);

    enum VerifyResult {
        Success,
        Invalid,
    }
    match db
        .transaction(|ts| {
            let challenge_outdated = challenge_outdated;
            let token_outdated = token_outdated;
            let payload = payload.clone();
            let nonce = nonce.clone();
            Box::pin(async move {
                // delete old data
                entity::challenge::Entity::delete_many()
                    .filter(entity::challenge::Column::CreatedAt.lt(challenge_outdated))
                    .exec(ts)
                    .await?;
                entity::token::Entity::delete_many()
                    .filter(entity::token::Column::CreatedAt.lt(token_outdated))
                    .exec(ts)
                    .await?;
                // try to delete the challenge record
                let deleted = entity::challenge::Entity::delete_by_id(payload)
                    .exec(ts)
                    .await?;
                // commit if none
                if deleted.rows_affected == 0 {
                    return Ok::<VerifyResult, DbErr>(VerifyResult::Invalid);
                }
                // insert token
                entity::token::Entity::insert(entity::token::ActiveModel {
                    nonce: sea_orm::ActiveValue::Set(nonce),
                    created_at: sea_orm::ActiveValue::Set(now),
                })
                .exec(ts)
                .await?;
                Ok::<VerifyResult, DbErr>(VerifyResult::Success)
            })
        })
        .await
    {
        Ok(x) => match x {
            VerifyResult::Success => {}
            VerifyResult::Invalid => {
                return Err(ChallengeError::InvalidPayload().to_error().to_response())
            }
        },
        Err(e) => {
            return match e {
                sea_orm::TransactionError::Connection(e) => {
                    Err(ChallengeError::DatabaseError(e).to_error().to_response())
                }
                sea_orm::TransactionError::Transaction(_e) => {
                    Err(ChallengeError::GeneratorError().to_error().to_response())
                }
            }
        }
    };

    Ok(nonce)
}

pub async fn revoke(
    State((db, sv)): State<(DatabaseConnection, ServiceConfig)>,
    Path((nonce, hash)): Path<(String, String)>,
) -> Result<String, ErrorResponse> {
    match tokio::task::spawn_blocking({
        let nonce = nonce.clone();
        let hash = hash.clone();
        let secret = sv.secret.clone();
        move || utils::verify_token(nonce, hash, secret)
    })
    .await
    {
        Ok(x) => {
            if !x {
                return Err(ChallengeError::InvalidToken().to_error().to_response());
            }
        }
        Err(e) => {
            return Err(ChallengeError::TaskError(e).to_error().to_response());
        }
    }

    enum VerifyResult {
        Success,
        Invalid,
    }
    match db
        .transaction(|ts| {
            let nonce = nonce.clone();
            Box::pin(async move {
                // try to delete the token record
                let deleted = entity::token::Entity::delete_by_id(nonce).exec(ts).await?;
                // commit if none
                if deleted.rows_affected == 0 {
                    return Ok::<VerifyResult, DbErr>(VerifyResult::Invalid);
                }
                Ok::<VerifyResult, DbErr>(VerifyResult::Success)
            })
        })
        .await
    {
        Ok(x) => match x {
            VerifyResult::Success => {}
            VerifyResult::Invalid => {
                return Err(ChallengeError::InvalidToken().to_error().to_response())
            }
        },
        Err(e) => {
            return match e {
                sea_orm::TransactionError::Connection(e) => {
                    Err(ChallengeError::DatabaseError(e).to_error().to_response())
                }
                sea_orm::TransactionError::Transaction(_e) => {
                    Err(ChallengeError::GeneratorError().to_error().to_response())
                }
            }
        }
    };

    Ok(nonce)
}
