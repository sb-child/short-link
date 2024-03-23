use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ShortLink::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ShortLink::Name)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ShortLink::Target).string().not_null())
                    .col(ColumnDef::new(ShortLink::Enabled).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ShortLink::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ShortLink {
    Table,
    Name,
    Enabled,
    Target,
}
