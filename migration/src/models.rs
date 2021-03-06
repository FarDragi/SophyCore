use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Level,
    Progress,
    XpUpdatedAt,
}

#[derive(Iden)]
pub enum Guild {
    Table,
    Id,
}

#[derive(Iden)]
pub enum Xp {
    Table,
    UserId,
    GuildId,
    Level,
    Progress,
    UpdatedAt,
}
