use sea_orm::{Database, DatabaseConnection, DbErr};

pub struct DatabaseOptions {
    pub url: String,
}

pub async fn new_connection(opt: DatabaseOptions) -> Result<DatabaseConnection, DbErr> {
    let db: DatabaseConnection = Database::connect(opt.url).await?;
    Ok(db)
}
