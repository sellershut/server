mod input;

use api_core::Mutation;
use async_graphql::{Context, Object, Result};
use entity::{
    async_graphql,
    sea_orm::prelude::{DateTimeWithTimeZone, Uuid},
    session,
};

use crate::Database;

use self::input::SessionInput;

#[derive(Default)]
pub struct SessionMutation;

#[Object]
impl SessionMutation {
    async fn create_session(
        &self,
        ctx: &Context<'_>,
        input: SessionInput,
    ) -> Result<session::Model> {
        let conn = Database::get_connection_from_context(ctx)?;

        Ok(Mutation::create_session(conn, input.into_model_with_arbitrary_id()).await?)
    }

    async fn update_session(
        &self,
        ctx: &Context<'_>,
        session_token: String,
        user_id: Option<Uuid>,
        expires: Option<DateTimeWithTimeZone>,
    ) -> Result<session::Model> {
        let conn = Database::get_connection_from_context(ctx)?;

        Ok(Mutation::update_session(conn, session_token, user_id, expires).await?)
    }
}
