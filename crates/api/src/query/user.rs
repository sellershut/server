use api_core::Query;
use entity::{
    async_graphql,
    sea_orm::{prelude::Uuid, DbErr},
    user,
};

use async_graphql::{Context, Object};

use crate::Database;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_user_by_id(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<user::Model>, DbErr> {
        let conn = Database::get_connection_from_context(ctx)?;

        Ok(Query::find_user_by_id(conn, id).await?)
    }

    async fn get_user_by_email(
        &self,
        ctx: &Context<'_>,
        email: String,
    ) -> Result<Option<user::Model>, DbErr> {
        let conn = Database::get_connection_from_context(ctx)?;

        Ok(Query::find_user_by_email(conn, email).await?)
    }

    async fn get_user_by_account(
        &self,
        ctx: &Context<'_>,
        provider: String,
        provider_account_id: String,
    ) -> Result<Option<user::Model>, DbErr> {
        let conn = Database::get_connection_from_context(ctx)?;

        Ok(Query::find_user_by_account(conn, provider, provider_account_id).await?)
    }
}
