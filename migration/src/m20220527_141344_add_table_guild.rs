use sea_orm_migration::prelude::*;

use crate::models::Guild;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220527_141344_add_guild"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Guild::Table)
                    .col(
                        ColumnDef::new(Guild::Id)
                            .string_len(19)
                            .primary_key()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Guild::Table).to_owned())
            .await
    }
}
