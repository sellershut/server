use entity::{
    account,
    account::Entity as Account,
    sea_orm::{prelude::Uuid, DbConn, DbErr, EntityTrait, Set},
};

use crate::{DeleteResult, Mutation};

impl Mutation {
    pub async fn link_account(
        db: &DbConn,
        form_data: account::Model,
    ) -> Result<account::Model, DbErr> {
        let active_model = account::ActiveModel {
            id: Set(Uuid::new_v4()),
            provider: Set(form_data.provider.to_owned()),
            provider_account_id: Set(form_data.provider_account_id.to_owned()),
            access_token: Set(form_data.access_token.to_owned()),
            expires_at: Set(form_data.expires_at.to_owned()),
            id_token: Set(form_data.id_token.to_owned()),
            user_id: Set(form_data.user_id.to_owned()),
            r#type: Set(form_data.r#type.to_owned()),
            refresh_token: Set(form_data.refresh_token.to_owned()),
            token_type: Set(form_data.token_type.to_owned()),
            scope: Set(form_data.scope.to_owned()),
            session_state: Set(form_data.session_state.to_owned()),
        };
        let res = Account::insert(active_model).exec(db).await?;

        Ok(account::Model {
            id: res.last_insert_id,
            ..form_data
        })
    }

    pub async fn unlink_account(
        db: &DbConn,
        provider: String,
        provider_account_id: String,
    ) -> Result<DeleteResult, DbErr> {
        let res = Account::delete_by_provider(provider, provider_account_id)
            .exec(db)
            .await?;
        Ok(DeleteResult {
            success: true,
            rows_affected: res.rows_affected,
        })
    }
}
