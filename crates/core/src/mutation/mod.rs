use async_graphql::SimpleObject;
use entity::async_graphql;

mod account;
mod session;
mod user;

pub struct Mutation;

#[derive(SimpleObject)]
pub struct DeleteResult {
    pub success: bool,
    pub rows_affected: u64,
}
