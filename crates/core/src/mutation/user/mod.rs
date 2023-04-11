use entity::{
    sea_orm::{prelude::Uuid, DbConn, DbErr, EntityTrait, Set},
    user,
    user::Entity as User,
};

use crate::Mutation;

impl Mutation {
    pub async fn create_user(db: &DbConn, form_data: user::Model) -> Result<user::Model, DbErr> {
        let active_model = user::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(form_data.name.to_owned()),
            email_verified: Set(form_data.email_verified.to_owned()),
            image: Set(form_data.image.to_owned()),
            saved_ads: Set(form_data.saved_ads.to_owned()),
            watched_categories: Set(form_data.watched_categories.to_owned()),
            ..Default::default()
        };
        let res = User::insert(active_model).exec(db).await?;

        Ok(user::Model {
            id: res.last_insert_id,
            ..form_data
        })
    }
}
