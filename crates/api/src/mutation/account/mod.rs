mod input;

use api_core::{DeleteResult, Mutation};
use async_graphql::{Context, Object, Result};
use entity::{account, async_graphql};

use crate::Database;

use self::input::AccountInput;

#[derive(Default)]
pub struct AccountMutation;

#[Object]
impl AccountMutation {
    async fn create_account(
        &self,
        ctx: &Context<'_>,
        input: AccountInput,
    ) -> Result<account::Model> {
        let conn = Database::get_connection_from_context(ctx)?;

        Ok(Mutation::link_account(conn, input.into_model_with_arbitrary_id()).await?)
    }

    async fn delete_account(
        &self,
        ctx: &Context<'_>,
        provider: String,
        provider_account_id: String,
    ) -> Result<DeleteResult> {
        let conn = Database::get_connection_from_context(ctx)?;

        Ok(Mutation::unlink_account(conn, provider, provider_account_id).await?)
    }
}
