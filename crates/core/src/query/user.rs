use entity::{
    sea_orm::{prelude::Uuid, DbConn, DbErr, EntityTrait},
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
}
