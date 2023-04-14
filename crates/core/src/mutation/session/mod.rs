use entity::{
    sea_orm::{
        prelude::{DateTimeWithTimeZone, Uuid},
        ActiveModelTrait, DbConn, DbErr, EntityTrait, Set,
    },
    session,
    session::Entity as Session,
};

use crate::{DeleteResult, Mutation};

impl Mutation {
    pub async fn create_session(
        db: &DbConn,
        form_data: session::Model,
    ) -> Result<session::Model, DbErr> {
        let active_model = session::ActiveModel {
            id: Set(Uuid::new_v4()),
            session_token: Set(form_data.session_token.to_owned()),
            user_id: Set(form_data.user_id.to_owned()),
            expires: Set(form_data.expires.to_owned()),
        };
        let res = Session::insert(active_model).exec(db).await?;

        Ok(session::Model {
            id: res.last_insert_id,
            ..form_data
        })
    }

    pub async fn update_session(
        db: &DbConn,
        session_token: String,
        user_id: Option<Uuid>,
        expires: Option<DateTimeWithTimeZone>,
    ) -> Result<session::Model, DbErr> {
        if let Some(session) = Session::find_by_session_token(session_token.clone())
            .one(db)
            .await?
        {
            // Into ActiveModel
            let mut session: session::ActiveModel = session.into();
            if let Some(ref user_id) = user_id {
                session.user_id = Set(user_id.to_owned())
            }
            if let Some(ref expires) = expires {
                session.expires = Set(expires.to_owned())
            }
            Ok(session.update(db).await?)
        } else {
            Err(DbErr::RecordNotFound(session_token.to_string()))
        }
    }

    pub async fn delete_session(db: &DbConn, session_token: String) -> Result<DeleteResult, DbErr> {
        let res = Session::delete_by_session(session_token).exec(db).await?;
        Ok(DeleteResult {
            success: true,
            rows_affected: res.rows_affected,
        })
    }
}
