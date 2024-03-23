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
                    .add_column_if_not_exists(
                        ColumnDef::new(ShortLink::Counter).big_integer().not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ShortLink::Table)
                    .drop_column(ShortLink::Counter)
                    .to_owned(),
            )
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
