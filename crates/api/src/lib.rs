mod mutation;
mod query;

use entity::{
    async_graphql::{Context, EmptySubscription, Schema},
    sea_orm::{DatabaseConnection, DbErr, RuntimeErr},
};

use self::{mutation::Mutation, query::Query};

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

    pub fn get_connection_from_context<'a>(
        ctx: &'a Context<'a>,
    ) -> Result<&'a DatabaseConnection, DbErr> {
        let db = ctx
            .data::<Self>()
            .map_err(|e| DbErr::Conn(RuntimeErr::Internal(e.message)))?;
        Ok(db.get_connection())
    }
}

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub async fn build_schema() -> AppSchema {
    let db = Database::new().await;

    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .finish()
}
