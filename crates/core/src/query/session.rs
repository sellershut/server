use entity::{
    sea_orm::{DbConn, DbErr},
    session::{self, Entity as Session},
    user::{self, Entity as User},
};

use crate::Query;

#[derive(Default)]
pub struct SessionQuery;

impl Query {
    pub async fn find_user_by_session_token(
        db: &DbConn,
        session_token: String,
    ) -> Result<Option<(session::Model, Option<user::Model>)>, DbErr> {
        let result = Session::find_by_session_token(session_token)
            .find_also_related(User)
            .one(db)
            .await?;
        Ok(result)
    }
}
