use entity::async_graphql;

mod account;
mod session;
mod user;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(
    user::UserMutation,
    account::AccountMutation,
    session::SessionMutation,
);
