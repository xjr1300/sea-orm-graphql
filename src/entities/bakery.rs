//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5

use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "bakery")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub profit_margin: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::chef::Entity")]
    Chef,
}

impl Related<super::chef::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Chef.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
