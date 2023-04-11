use async_graphql::InputObject;
use entity::{async_graphql, sea_orm::prelude::DateTimeWithTimeZone};

#[derive(InputObject, Debug)]
pub struct UserInput {
    name: Option<String>,
    email: Option<String>,
    email_verified: Option<DateTimeWithTimeZone>,
    image: Option<String>,
    saved_ads: Option<Vec<String>>,
    watched_categories: Option<Vec<i32>>,
}
