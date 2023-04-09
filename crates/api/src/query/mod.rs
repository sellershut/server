use entity::async_graphql;

mod category;

#[derive(async_graphql::MergedObject, Default)]
pub struct Query(category::CategoryQuery);
