//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "guild")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::xp::Entity")]
    Xp,
}

impl Related<super::xp::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Xp.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}