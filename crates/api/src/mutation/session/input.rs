use async_graphql::InputObject;
use entity::{
    async_graphql,
    sea_orm::prelude::{DateTimeWithTimeZone, Uuid},
    session,
};

#[derive(Debug, InputObject)]
pub struct SessionInput {
    session_token: String,
    user_id: Uuid,
    expires: DateTimeWithTimeZone,
}

impl SessionInput {
    pub(crate) fn into_model_with_arbitrary_id(self) -> session::Model {
        session::Model {
            id: Uuid::nil(),
            session_token: self.session_token,
            expires: self.expires,
            user_id: self.user_id,
        }
    }
}
