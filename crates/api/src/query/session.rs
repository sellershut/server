use api_core::Query;
use entity::{
    async_graphql::{self, SimpleObject},
    sea_orm::DbErr,
    session, user,
};

use async_graphql::{Context, Object};
use serde::{Deserialize, Serialize};

use crate::Database;

#[derive(Default)]
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
        let (conn, mut redis) = Database::get_connection_from_context(ctx)?;
        let key = format!("user+session:token={session_token}");
        if let Ok(cache) = Database::get_redis_cache::<UserSession>(&key, &mut redis).await {
            Ok(cache)
        } else {
            let result = Query::find_user_by_session_token(conn, session_token.clone())
                .await?
                .map(|(session, user)| user.map(|user| UserSession { session, user }));
            match result {
                Some(Some(user_session)) => {
                    if let Err(e) = Database::set_redis_cache(&key, &mut redis, &user_session).await
                    {
                        println!("{e}");
                    }
                    Ok(user_session)
                }
                _ => Err(DbErr::RecordNotFound(format!(
                    "no records match with session token: {session_token}"
                ))),
            }
        }
    }
}
