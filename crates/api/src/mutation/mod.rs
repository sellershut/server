use entity::async_graphql;

mod user;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(user::UserMutation);
