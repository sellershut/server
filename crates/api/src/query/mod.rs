use entity::async_graphql;

mod category;
mod user;

#[derive(async_graphql::MergedObject, Default)]
pub struct Query(category::CategoryQuery, user::UserQuery);
