use entity::{
    sea_orm::{prelude::Uuid, ActiveModelTrait, DbConn, DbErr, EntityTrait, Set},
    user,
    user::Entity as User,
};

use crate::Mutation;

impl Mutation {
    pub async fn create_user(db: &DbConn, form_data: user::Model) -> Result<user::Model, DbErr> {
        let active_model = user::ActiveModel {
            id: Set(Uuid::new_v4()),
            email: Set(form_data.email.to_owned()),
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

    pub async fn update_user(
        db: &DbConn,
        id: Uuid,
        form_data: user::Model,
    ) -> Result<user::Model, DbErr> {
        if let Some(user) = User::find_by_id(id).one(db).await? {
            // Into ActiveModel
            let mut user: user::ActiveModel = user.into();
            if let Some(ref name) = form_data.name {
                user.name = Set(Some(name.to_owned()))
            }
            if let Some(ref email_verified) = form_data.email_verified {
                user.email_verified = Set(Some(email_verified.to_owned()))
            }
            if let Some(ref email) = form_data.email {
                user.email = Set(Some(email.to_owned()))
            }
            if let Some(ref saved_ads) = form_data.saved_ads {
                user.saved_ads = Set(Some(saved_ads.to_owned()))
            }
            if let Some(ref image) = form_data.image {
                user.image = Set(Some(image.to_owned()))
            }
            if let Some(ref watched_categories) = form_data.watched_categories {
                user.watched_categories = Set(Some(watched_categories.to_owned()))
            }

            Ok(user.update(db).await?)
        } else {
            Err(DbErr::RecordNotFound(id.to_string()))
        }
    }
}
