//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use async_graphql::SimpleObject;
use sea_orm::{entity::prelude::*, Condition, DeleteMany};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "account")]
#[graphql(concrete(name = "Account", params()))]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub r#type: String,
    #[sea_orm(column_type = "Text")]
    pub provider: String,
    #[sea_orm(column_type = "Text")]
    pub provider_account_id: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub refresh_token: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub access_token: Option<String>,
    pub expires_at: Option<i32>,
    #[sea_orm(column_type = "Text", nullable)]
    pub token_type: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub scope: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub id_token: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub session_state: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_provider(provider: String, provider_id: String) -> Select<Entity> {
        Self::find().filter(
            Condition::all()
                .add(Column::Provider.eq(provider))
                .add(Column::ProviderAccountId.eq(provider_id)),
        )
    }

    pub fn delete_by_provider(provider: String, provider_account_id: String) -> DeleteMany<Entity> {
        Self::delete_many().filter(
            Condition::all()
                .add(Column::Provider.eq(provider))
                .add(Column::ProviderAccountId.eq(provider_account_id)),
        )
    }
}
