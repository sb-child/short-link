use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ShortLink::Table)
                    .drop_column(ShortLink::Counter)
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Views::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Views::Id)
                            .big_integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Views::LinkName).string().not_null())
                    .col(ColumnDef::new(Views::Target).string().not_null())
                    .col(ColumnDef::new(Views::UserAgent).string().not_null())
                    .col(ColumnDef::new(Views::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ShortLink::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(ShortLink::Counter).big_integer().not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Views::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[allow(dead_code)]
enum ShortLink {
    Table,
    Name,
    Enabled,
    Target,
    Counter,
}

#[derive(DeriveIden)]
enum Views {
    Table,
    Id,
    LinkName,
    Target,
    UserAgent,
    CreatedAt,
}
