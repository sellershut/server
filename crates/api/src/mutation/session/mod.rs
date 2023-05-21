mod input;

use api_core::{DeleteResult, Mutation};
use async_graphql::{Context, Object, Result};
use entity::{
    async_graphql,
    sea_orm::prelude::{DateTimeWithTimeZone, Uuid},
    session,
};
use tracing::{error, span, Level};

use crate::{cache::CacheKey, Database};

use self::input::SessionInput;

#[derive(Default)]
pub struct SessionMutation;

#[Object]
impl SessionMutation {
    async fn create_session(
        &self,
        ctx: &Context<'_>,
        input: SessionInput,
    ) -> Result<session::Model> {
        let span = span!(Level::TRACE, "create_session");
        let _enter = span.enter();
        let (conn, mut redis) = Database::get_connection_from_context(ctx)?;
        let key = CacheKey::Session {
            token: input.session_token.clone(),
        };
        let session = Mutation::create_session(conn, input.into_model_with_arbitrary_id()).await?;
        if let Err(e) = Database::set_redis_cache(&key, &mut redis, &session).await {
            error!("{e}")
        }
        Ok(session)
    }

    async fn update_session(
        &self,
        ctx: &Context<'_>,
        session_token: String,
        user_id: Option<Uuid>,
        expires: Option<DateTimeWithTimeZone>,
    ) -> Result<session::Model> {
        let span = span!(Level::TRACE, "update_session");
        let _enter = span.enter();
        let (conn, mut redis) = Database::get_connection_from_context(ctx)?;
        let key = CacheKey::Session {
            token: session_token.clone(),
        };
        let update = Mutation::update_session(conn, session_token, user_id, expires).await?;
        if let Err(err) = Database::set_redis_cache(&key, &mut redis, &update).await {
            error!("{err}");
            if let Err(err) = Database::delete_redis_cache(&key, &mut redis).await {
                error!("{err}");
            }
        }
        Ok(update)
    }

    async fn delete_session(
        &self,
        ctx: &Context<'_>,
        session_token: String,
    ) -> Result<DeleteResult> {
        let span = span!(Level::TRACE, "delete_session");
        let _enter = span.enter();
        let (conn, mut redis) = Database::get_connection_from_context(ctx)?;
        let key = CacheKey::Session {
            token: session_token.clone(),
        };
        if let Err(e) = Database::delete_redis_cache(&key, &mut redis).await {
            error!("{e}");
        }

        Ok(Mutation::delete_session(conn, session_token).await?)
    }
}
