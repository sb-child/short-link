use axum::{
    extract::{Path, State},
    response::Redirect,
    Json,
};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, TransactionTrait};

use crate::{
    config::ServiceConfig,
    errors::{short_link::ShortLinkError, ErrorResponse, ToError},
    utils,
};

pub async fn index(
    State((db, _sv)): State<(DatabaseConnection, ServiceConfig)>,
    Path(id): Path<String>,
) -> Result<Redirect, ErrorResponse> {
    match db
        .transaction(|ts| {
            Box::pin(async move {
                let link = entity::short_link::Entity::find()
                    .filter(entity::short_link::Column::Name.eq(id.clone()))
                    .filter(entity::short_link::Column::Enabled.eq(true))
                    .one(ts)
                    .await;
                let link = match link {
                    Ok(link) => link,
                    Err(e) => return Err(e),
                };
                return if let Some(link) = link {
                    match entity::short_link::Entity::update(entity::short_link::ActiveModel {
                        name: sea_orm::ActiveValue::set(id.clone()),
                        target: sea_orm::ActiveValue::NotSet,
                        enabled: sea_orm::ActiveValue::NotSet,
                        counter: sea_orm::ActiveValue::set(link.counter + 1),
                    })
                    .filter(entity::short_link::Column::Name.eq(id))
                    .exec(ts)
                    .await
                    {
                        Ok(_) => {}
                        Err(e) => return Err(e),
                    };
                    Ok(Some(link))
                } else {
                    Ok(None)
                };
            })
        })
        .await
    {
        Ok(x) => match x {
            Some(link) => {
                return Ok(Redirect::to(&link.target));
            }
            None => return Err(ShortLinkError::Invalid().to_error().to_response()),
        },
        Err(e) => {
            return match e {
                sea_orm::TransactionError::Connection(e) => {
                    Err(ShortLinkError::DatabaseError(e).to_error().to_response())
                }
                sea_orm::TransactionError::Transaction(_e) => {
                    Err(ShortLinkError::Invalid().to_error().to_response())
                }
            }
        }
    };
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    View,
    Insert,
    Update,
    Delete,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateReq {
    action: Action,
    props: LinkProps,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateResp {
    props: LinkProps,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkProps {
    target: Option<String>,
    enabled: Option<bool>,
    counter: Option<i64>,
}

pub async fn update(
    State((db, sv)): State<(DatabaseConnection, ServiceConfig)>,
    Path((token, id)): Path<(String, String)>,
    Json(data): Json<UpdateReq>,
) -> Result<Json<UpdateResp>, ErrorResponse> {
    if !utils::auth_token(
        token,
        sv.secret,
        &db,
        sv.challenge_timeout,
        sv.token_timeout,
    )
    .await
    {
        return Err(ShortLinkError::InvalidToken().to_error().to_response());
    }
    match data.action {
        Action::View => view_link(id, &db).await,
        Action::Insert => insert_link(id, data.props, &db).await,
        Action::Update => update_link(id, data.props, &db).await,
        Action::Delete => delete_link(id, &db).await,
    }
}

async fn view_link(
    link_id: String,
    db: &DatabaseConnection,
) -> Result<Json<UpdateResp>, ErrorResponse> {
    match db
        .transaction(|ts| {
            Box::pin(async move {
                let link = entity::short_link::Entity::find_by_id(link_id)
                    .one(ts)
                    .await?;
                Ok::<_, DbErr>(link)
            })
        })
        .await
    {
        Ok(x) => match x {
            Some(x) => {
                return Ok(axum::Json(UpdateResp {
                    props: LinkProps {
                        target: Some(x.target),
                        enabled: Some(x.enabled),
                        counter: Some(x.counter),
                    },
                }))
            }
            None => return Err(ShortLinkError::Invalid().to_error().to_response()),
        },
        Err(e) => {
            return match e {
                sea_orm::TransactionError::Connection(e) => {
                    Err(ShortLinkError::DatabaseError(e).to_error().to_response())
                }
                sea_orm::TransactionError::Transaction(_e) => {
                    Err(ShortLinkError::Invalid().to_error().to_response())
                }
            }
        }
    };
}

async fn insert_link(
    link_id: String,
    props: LinkProps,
    db: &DatabaseConnection,
) -> Result<Json<UpdateResp>, ErrorResponse> {
    let target = if let Some(x) = props.target {
        x
    } else {
        return Err(ShortLinkError::FieldRequired().to_error().to_response());
    };
    let enabled = if let Some(x) = props.enabled { x } else { true };
    let counter = if let Some(x) = props.counter { x } else { 0 };
    let props_2 = LinkProps {
        target: Some(target.clone()),
        enabled: Some(enabled),
        counter: Some(counter),
    };
    match db
        .transaction(|ts| {
            Box::pin(async move {
                entity::short_link::Entity::insert(entity::short_link::ActiveModel {
                    name: sea_orm::ActiveValue::Set(link_id),
                    target: sea_orm::ActiveValue::Set(target),
                    enabled: sea_orm::ActiveValue::Set(enabled),
                    counter: sea_orm::ActiveValue::Set(counter),
                })
                .exec(ts)
                .await?;
                Ok::<_, DbErr>(())
            })
        })
        .await
    {
        Ok(_x) => {
            return Ok(axum::Json(UpdateResp { props: props_2 }));
        }
        Err(e) => {
            return match e {
                sea_orm::TransactionError::Connection(e) => {
                    Err(ShortLinkError::DatabaseError(e).to_error().to_response())
                }
                sea_orm::TransactionError::Transaction(_e) => {
                    Err(ShortLinkError::Exists().to_error().to_response())
                }
            }
        }
    };
}

async fn delete_link(
    link_id: String,
    db: &DatabaseConnection,
) -> Result<Json<UpdateResp>, ErrorResponse> {
    match db
        .transaction(|ts| {
            Box::pin(async move {
                entity::short_link::Entity::delete_by_id(link_id)
                    .exec(ts)
                    .await?;
                Ok::<_, DbErr>(())
            })
        })
        .await
    {
        Ok(_x) => {
            return Ok(axum::Json(UpdateResp {
                props: LinkProps {
                    target: None,
                    enabled: None,
                    counter: None,
                },
            }));
        }
        Err(e) => {
            return match e {
                sea_orm::TransactionError::Connection(e) => {
                    Err(ShortLinkError::DatabaseError(e).to_error().to_response())
                }
                sea_orm::TransactionError::Transaction(_e) => {
                    Err(ShortLinkError::Invalid().to_error().to_response())
                }
            }
        }
    };
}

async fn update_link(
    link_id: String,
    props: LinkProps,
    db: &DatabaseConnection,
) -> Result<Json<UpdateResp>, ErrorResponse> {
    match db
        .transaction(|ts| {
            Box::pin(async move {
                let a = entity::short_link::Entity::update(entity::short_link::ActiveModel {
                    name: sea_orm::ActiveValue::set(link_id.clone()),
                    target: if let Some(x) = props.target {
                        sea_orm::ActiveValue::Set(x)
                    } else {
                        sea_orm::ActiveValue::NotSet
                    },
                    enabled: if let Some(x) = props.enabled {
                        sea_orm::ActiveValue::Set(x)
                    } else {
                        sea_orm::ActiveValue::NotSet
                    },
                    counter: if let Some(x) = props.counter {
                        sea_orm::ActiveValue::Set(x)
                    } else {
                        sea_orm::ActiveValue::NotSet
                    },
                })
                .filter(entity::short_link::Column::Name.eq(link_id))
                .exec(ts)
                .await?;
                Ok::<_, DbErr>(a)
            })
        })
        .await
    {
        Ok(x) => {
            return Ok(axum::Json(UpdateResp {
                props: LinkProps {
                    target: Some(x.target),
                    enabled: Some(x.enabled),
                    counter: Some(x.counter),
                },
            }));
        }
        Err(e) => {
            return match e {
                sea_orm::TransactionError::Connection(e) => {
                    Err(ShortLinkError::DatabaseError(e).to_error().to_response())
                }
                sea_orm::TransactionError::Transaction(_e) => {
                    Err(ShortLinkError::Invalid().to_error().to_response())
                }
            }
        }
    };
}
