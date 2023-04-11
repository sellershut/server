use entity::{
    account::Entity as Account,
    sea_orm::{prelude::Uuid, DbConn, DbErr, EntityTrait, ModelTrait},
    user::{self, Entity as User},
};

use crate::Query;

#[derive(Default)]
pub struct UserQuery;

impl Query {
    pub async fn find_user_by_id(db: &DbConn, id: Uuid) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(db).await
    }
    pub async fn find_user_by_email(
        db: &DbConn,
        email: String,
    ) -> Result<Option<user::Model>, DbErr> {
        User::find_by_email(email).one(db).await
    }

    pub async fn find_user_by_account(
        db: &DbConn,
        provider: String,
        provider_account_id: String,
    ) -> Result<Option<user::Model>, DbErr> {
        // Find a cake model first
        if let Some(account) =
            Account::find_by_provider(provider.clone(), provider_account_id.clone())
                .one(db)
                .await?
        {
            // Then, find all related fruits of this cake
            Ok(account.find_related(User).one(db).await?)
        } else {
            Err(DbErr::RecordNotFound(format!(
                "no user found with account provider: {provider} with id {provider_account_id}"
            )))
        }
    }
}
