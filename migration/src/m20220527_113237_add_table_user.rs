use sea_orm_migration::prelude::*;

use crate::models::User;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220527_113237_add_user"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .col(
                        ColumnDef::new(User::Id)
                            .string_len(19)
                            .primary_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::Level).integer().not_null().default(0))
                    .col(
                        ColumnDef::new(User::Progress)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(User::XpUpdatedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}
