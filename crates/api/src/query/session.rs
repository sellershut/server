use api_core::Query;
use entity::{
    async_graphql::{self, SimpleObject},
    sea_orm::DbErr,
    session, user,
};

use async_graphql::{Context, Object};
use serde::{Deserialize, Serialize};
use tracing::{error, span, Level};

use crate::{
    cache::{CacheKey, CacheUserFilter},
    Database,
};

#[derive(Default, Debug)]
pub struct SessionQuery;

#[derive(SimpleObject, Deserialize, Serialize)]
pub struct UserSession {
    session: session::Model,
    user: user::Model,
}

#[Object]
impl SessionQuery {
    async fn get_user_and_session(
        &self,
        ctx: &Context<'_>,
        session_token: String,
    ) -> Result<UserSession, DbErr> {
        let span = span!(Level::TRACE, "get_user_and_session()");
        let _enter = span.enter();
        let (conn, mut redis) = Database::get_connection_from_context(ctx)?;
        let session_key = CacheKey::Session {
            token: session_token.clone(),
        };
        let user_session_key = CacheKey::User(CacheUserFilter::Session {
            token: session_token.clone(),
        });
        let session = Database::get_redis_cache::<session::Model>(&session_key, &mut redis).await;
        let user = Database::get_redis_cache::<user::Model>(&user_session_key, &mut redis).await;
        let result = session.and_then(|session| user.map(|user| UserSession { session, user }));
        if let Ok(result) = result {
            Ok(result)
        } else {
            let result = Query::find_user_by_session_token(conn, session_token.clone())
                .await?
                .map(|(session, user)| user.map(|user| UserSession { session, user }));
            if let Some(Some(user_session)) = result {
                if let Err(e) =
                    Database::set_redis_cache(&session_key, &mut redis, &user_session.session).await
                {
                    error!("{e}");
                }
                if let Err(e) =
                    Database::set_redis_cache(&user_session_key, &mut redis, &user_session.user)
                        .await
                {
                    error!("{e}");
                }
                Ok(user_session)
            } else {
                Err(DbErr::RecordNotFound(format!(
                    "no records match with session token: {session_token}"
                )))
            }
        }
    }
}
