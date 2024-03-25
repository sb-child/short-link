pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240321_085050_challenge;
mod m20240321_144055_token;
mod m20240323_073911_add_counter;
mod m20240325_171558_views;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240321_085050_challenge::Migration),
            Box::new(m20240321_144055_token::Migration),
            Box::new(m20240323_073911_add_counter::Migration),
            Box::new(m20240325_171558_views::Migration),
        ]
    }
}
