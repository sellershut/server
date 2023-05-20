mod mutation;
mod query;

use entity::{
    async_graphql::{Context, EmptySubscription, Schema},
    redis::{self, FromRedisValue, RedisError},
    sea_orm::{DatabaseConnection, DbErr, RuntimeErr},
};
use serde::{Deserialize, Serialize};

use self::{mutation::Mutation, query::Query};

pub struct Database {
    pub connection: DatabaseConnection,
    pub redis: redis::aio::ConnectionManager,
}

impl Database {
    pub async fn new() -> Self {
        let connection = entity::sea_orm::Database::connect(
            std::env::var("DATABASE_URL").expect("DATABASE_URL was not set"),
        )
        .await
        .expect("Could not connect to database");

        let client =
            redis::Client::open("redis://127.0.0.1/").expect("could not open redis instance");

        let connection_manager = redis::aio::ConnectionManager::new(client)
            .await
            .expect("could not create connection manager");
        Self {
            connection,
            redis: connection_manager,
        }
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }

    pub fn get_redis(&self) -> redis::aio::ConnectionManager {
        self.redis.clone()
    }

    pub fn get_connection_from_context<'a>(
        ctx: &'a Context<'a>,
    ) -> Result<(&'a DatabaseConnection, redis::aio::ConnectionManager), DbErr> {
        let db = ctx
            .data::<Self>()
            .map_err(|e| DbErr::Conn(RuntimeErr::Internal(e.message)))?;
        Ok((db.get_connection(), db.get_redis()))
    }

    pub async fn get_redis_cache<T>(
        key: &str,
        connection_manager: &mut redis::aio::ConnectionManager,
    ) -> Result<T, RedisError>
    where
        T: for<'a> Deserialize<'a>,
    {
        let redis_cmd = redis::Cmd::get(key);
        let res = connection_manager.send_packed_command(&redis_cmd).await?;
        let result = String::from_redis_value(&res)?;
        let output = serde_json::from_str(&result).expect("err deserializing");
        Ok(output)
    }

    pub async fn set_redis_cache(
        key: &str,
        connection_manager: &mut redis::aio::ConnectionManager,
        data: impl Serialize,
    ) -> Result<(), RedisError> {
        let stream = serde_json::to_string(&data).unwrap();
        let redis_cmd = redis::Cmd::set(key, stream);
        connection_manager.send_packed_command(&redis_cmd).await?;
        Ok(())
    }
}

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub async fn build_schema() -> AppSchema {
    let db = Database::new().await;
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .finish()
}
