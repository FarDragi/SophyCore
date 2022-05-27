pub use sea_orm_migration::prelude::*;

mod m20220527_113237_add_table_user;
mod m20220527_141344_add_table_guild;
mod m20220527_145541_add_table_xp;
mod models;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220527_113237_add_table_user::Migration),
            Box::new(m20220527_141344_add_table_guild::Migration),
            Box::new(m20220527_145541_add_table_xp::Migration),
        ]
    }
}
