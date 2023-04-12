//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[sea_orm(table_name = "verification_token")]
#[graphql(concrete(name = "VerificationToken", params()))]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub identifier: Uuid,
    #[sea_orm(column_type = "Text", unique)]
    pub token: String,
    pub expires: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
