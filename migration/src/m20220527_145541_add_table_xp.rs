use sea_orm_migration::prelude::*;

use crate::models::{Guild, User, Xp};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220527_145541_add_xp"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Xp::Table)
                    .col(ColumnDef::new(Xp::GuildId).string_len(19).not_null())
                    .col(ColumnDef::new(Xp::UserId).string_len(19).not_null())
                    .col(ColumnDef::new(Xp::Level).integer().not_null().default(0))
                    .col(
                        ColumnDef::new(Xp::Progress)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Xp::UpdatedAt).timestamp_with_time_zone())
                    .primary_key(Index::create().col(Xp::UserId).col(Xp::GuildId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_guild_id")
                            .from(Xp::Table, Xp::GuildId)
                            .to(Guild::Table, Guild::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_id")
                            .from(Xp::Table, Xp::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Xp::Table).to_owned())
            .await
    }
}
