use hex::encode;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, TransactionTrait};
use sha3::{Digest, Sha3_512};

pub fn random_string(n: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect()
}

pub fn verify_challenge(payload: String, secret: String, hash: String) -> bool {
    let mut h = Sha3_512::new();
    h.update(payload.as_bytes());
    h.update(secret.as_bytes());
    let result = h.finalize();
    encode(result) == hash
}

pub fn verify_token(nonce: String, token: String, secret: String) -> bool {
    let mut h = Sha3_512::new();
    h.update("short-link-token_");
    h.update(nonce.as_bytes());
    h.update(secret.as_bytes());
    let result = h.finalize();
    encode(result) == token
}

pub async fn auth_token(
    token: String,
    secret: String,
    db: &DatabaseConnection,
    challenge_timeout: i64,
    token_timeout: i64,
) -> bool {
    let now = chrono::Utc::now().naive_utc();
    let challenge_outdated = now
        .checked_sub_signed(chrono::Duration::try_seconds(challenge_timeout).unwrap())
        .unwrap();
    let token_outdated = now
        .checked_sub_signed(chrono::Duration::try_seconds(token_timeout).unwrap())
        .unwrap();
    let (nonce, token) = if let Some((nonce, token)) = token.split_once("_") {
        (nonce.to_string(), token.to_string())
    } else {
        return false;
    };
    let nonce_2 = nonce.clone();
    match tokio::task::spawn_blocking(move || verify_token(nonce, token, secret)).await {
        Ok(x) => {
            if !x {
                return false;
            }
        }
        Err(_e) => {
            return false;
        }
    }

    enum VerifyResult {
        Success,
        Invalid,
    }
    match db
        .transaction(|ts| {
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
                // try to delete the token record
                let deleted = entity::token::Entity::delete_by_id(nonce_2)
                    .exec(ts)
                    .await?;
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
            VerifyResult::Invalid => return false,
        },
        Err(_e) => return false,
    };

    true
}
