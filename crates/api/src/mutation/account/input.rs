use async_graphql::InputObject;
use entity::{account, async_graphql, sea_orm::prelude::Uuid};

#[derive(Debug, InputObject)]
pub struct AccountInput {
    provider_account_id: String,
    provider: String,
    r#type: String,
    scope: Option<String>,
    user_id: Uuid,
    id_token: Option<String>,
    expires_in: Option<i32>,
    token_type: Option<String>,
    access_token: Option<String>,
    refresh_token: Option<String>,
    session_state: Option<String>,
}

impl AccountInput {
    pub(crate) fn into_model_with_arbitrary_id(self) -> account::Model {
        account::Model {
            id: Uuid::nil(),
            provider_account_id: self.provider_account_id,
            provider: self.provider,
            refresh_token: self.refresh_token,
            r#type: self.r#type,
            scope: self.scope,
            user_id: self.user_id,
            expires_at: self.expires_in,
            token_type: self.token_type,
            access_token: self.access_token,
            id_token: self.id_token,
            session_state: self.session_state,
        }
    }
}
