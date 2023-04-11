mod input;

use api_core::Mutation;
use async_graphql::{Context, Object, Result};
use entity::{async_graphql, sea_orm::prelude::Uuid, user};

use crate::Database;

use self::input::CreateUserInput;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<user::Model> {
        let conn = Database::get_connection_from_context(ctx)?;

        Ok(Mutation::create_user(conn, input.into_model_with_arbitrary_id()).await?)
    }

    async fn update_user(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: CreateUserInput,
    ) -> Result<user::Model> {
        let conn = Database::get_connection_from_context(ctx)?;
        let id = Uuid::parse_str(&id)?;

        Ok(Mutation::update_user(conn, id, input.into_model_with_arbitrary_id()).await?)
    }
}
