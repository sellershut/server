use api_core::Query;
use entity::{
    async_graphql,
    sea_orm::{prelude::Uuid, DbErr},
    user,
};

use async_graphql::{Context, Object};
use tracing::{error, span, Level};

use crate::{
    cache::{CacheKey, CacheUserFilter, CacheUserIdType},
    Database,
};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_user_by_id(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<user::Model>, DbErr> {
        let span = span!(Level::TRACE, "get_user_by_id");
        let _enter = span.enter();
        let (conn, mut redis) = Database::get_connection_from_context(ctx)?;
        let key = CacheKey::User(CacheUserFilter::Id(CacheUserIdType::ID(id)));
        if let Ok(cache) = Database::get_redis_cache::<user::Model>(&key, &mut redis).await {
            Ok(Some(cache))
        } else {
            let val = Query::find_user_by_id(conn, id).await;
            if let Ok(Some(ref user)) = val {
                if let Err(e) = Database::set_redis_cache(&key, &mut redis, user).await {
                    error!("{e}");
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
        let span = span!(Level::TRACE, "get_user_by_email");
        let _enter = span.enter();
        let (conn, mut redis) = Database::get_connection_from_context(ctx)?;
        let key = CacheKey::User(CacheUserFilter::Id(CacheUserIdType::Email(email.clone())));
        if let Ok(cache) = Database::get_redis_cache::<user::Model>(&key, &mut redis).await {
            Ok(Some(cache))
        } else {
            let val = Query::find_user_by_email(conn, email).await;
            if let Ok(Some(ref user)) = val {
                if let Err(e) = Database::set_redis_cache(&key, &mut redis, user).await {
                    error!("{e}");
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
        let span = span!(Level::TRACE, "get_user_by_account");
        let _enter = span.enter();
        let (conn, mut redis) = Database::get_connection_from_context(ctx)?;
        let key = CacheKey::User(CacheUserFilter::Account {
            provider: provider.clone(),
            provider_account_id: provider_account_id.clone(),
        });
        if let Ok(cache) = Database::get_redis_cache::<user::Model>(&key, &mut redis).await {
            Ok(Some(cache))
        } else {
            let val = Query::find_user_by_account(conn, provider, provider_account_id).await;
            if let Ok(Some(ref user)) = val {
                if let Err(e) = Database::set_redis_cache(&key, &mut redis, user).await {
                    error!("{e}");
                }
            }
            val
        }
    }
}
