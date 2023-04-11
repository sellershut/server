use async_graphql::InputObject;
use entity::{
    async_graphql, sea_orm::entity::prelude::Uuid, sea_orm::prelude::DateTimeWithTimeZone, user,
};

#[derive(InputObject, Debug)]
pub struct CreateUserInput {
    name: Option<String>,
    email: Option<String>,
    email_verified: Option<DateTimeWithTimeZone>,
    image: Option<String>,
    saved_ads: Option<Vec<String>>,
    watched_categories: Option<Vec<i32>>,
}

impl CreateUserInput {
    pub(crate) fn into_model_with_arbitrary_id(self) -> user::Model {
        user::Model {
            id: Uuid::nil(),
            name: self.name,
            email: self.email,
            email_verified: self.email_verified,
            image: self.image,
            saved_ads: self.saved_ads,
            watched_categories: self.watched_categories,
        }
    }
}
