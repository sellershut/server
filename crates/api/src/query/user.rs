use api_core::Query;
use entity::{
    async_graphql,
    sea_orm::{prelude::Uuid, DbErr},
    user,
};

use async_graphql::{Context, Object};

use crate::Database;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_user_by_id(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<user::Model>, DbErr> {
        let (conn, mut redis) = Database::get_connection_from_context(ctx)?;
        let key = users_schema("id", &id.to_string());
        if let Ok(cache) = Database::get_redis_cache::<user::Model>(&key, &mut redis).await {
            Ok(Some(cache))
        } else {
            let val = Query::find_user_by_id(conn, id).await;
            if let Ok(Some(ref user)) = val {
                if let Err(e) = Database::set_redis_cache(&key, &mut redis, user).await {
                    println!("{e}");
                }
            }
            val
        }
    }

    async fn get_user_by_email(
        &self,
        ctx: &Context<'_>,
        email: String,
    ) -> Result<Option<user::Model>, DbErr> {
        let (conn, mut redis) = Database::get_connection_from_context(ctx)?;
        let key = users_schema("email", &email);
        if let Ok(cache) = Database::get_redis_cache::<user::Model>(&key, &mut redis).await {
            Ok(Some(cache))
        } else {
            let val = Query::find_user_by_email(conn, email).await;
            if let Ok(Some(ref user)) = val {
                if let Err(e) = Database::set_redis_cache(&key, &mut redis, user).await {
                    println!("{e}");
                }
            }
            val
        }
    }

    async fn get_user_by_account(
        &self,
        ctx: &Context<'_>,
        provider: String,
        provider_account_id: String,
    ) -> Result<Option<user::Model>, DbErr> {
        let (conn, mut redis) = Database::get_connection_from_context(ctx)?;
        let key = users_schema(
            "account",
            &format!("provider={provider}:provider_account_id={provider_account_id}"),
        );
        if let Ok(cache) = Database::get_redis_cache::<user::Model>(&key, &mut redis).await {
            Ok(Some(cache))
        } else {
            let val = Query::find_user_by_account(conn, provider, provider_account_id).await;
            if let Ok(Some(ref user)) = val {
                if let Err(e) = Database::set_redis_cache(&key, &mut redis, user).await {
                    println!("{e}");
                }
            }
            val
        }
    }
}

fn users_schema(field: &str, value: &str) -> String {
    format!("user:{field}:{value}")
}
