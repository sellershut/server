mod mutation;
mod query;

use entity::{
    async_graphql::{EmptyMutation, EmptySubscription, Schema},
    sea_orm::DatabaseConnection,
};

use self::query::Query;

pub struct Database {
    pub connection: DatabaseConnection,
}

impl Database {
    pub async fn new() -> Self {
        let connection = entity::sea_orm::Database::connect(
            std::env::var("DATABASE_URL").expect("DATABASE_URL was not set"),
        )
        .await
        .expect("Could not connect to database");

        Self { connection }
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub async fn build_schema() -> AppSchema {
    let db = Database::new().await;

    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(db)
        .finish()
}
