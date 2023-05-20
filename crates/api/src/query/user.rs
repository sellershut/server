use api_core::Query;
use entity::{
    async_graphql, redis,
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

        let redis_cmd = redis::Cmd::get(&key);
        let result = redis
            .send_packed_command(&redis_cmd)
            .await
            .expect("could not send redis query");
        match result {
            redis::Value::Nil => {
                println!("redis no exist");
                let ret_val = Query::find_user_by_id(conn, id).await;
                if let Ok(Some(value)) = &ret_val {
                    if let Ok(json) = serde_json::to_string(&value) {
                        let redis_cmd = redis::Cmd::set(&key, json);
                        if let Err(e) = redis.send_packed_command(&redis_cmd).await {
                            println!("{e}");
                        } else {
                            println!("value set");
                        }
                    }
                }
                //set redis
                ret_val
            }
            redis::Value::Int(_) => todo!(),
            redis::Value::Data(_) => todo!(),
            redis::Value::Bulk(_) => todo!(),
            redis::Value::Status(_) => todo!(),
            redis::Value::Okay => todo!(),
        }
    }

    async fn get_user_by_email(
        &self,
        ctx: &Context<'_>,
        email: String,
    ) -> Result<Option<user::Model>, DbErr> {
        let (conn, _redis) = Database::get_connection_from_context(ctx)?;

        Query::find_user_by_email(conn, email).await
    }

    async fn get_user_by_account(
        &self,
        ctx: &Context<'_>,
        provider: String,
        provider_account_id: String,
    ) -> Result<Option<user::Model>, DbErr> {
        let (conn, _redis) = Database::get_connection_from_context(ctx)?;

        Query::find_user_by_account(conn, provider, provider_account_id).await
    }
}

fn users_schema(field: &str, value: &str) -> String {
    format!("user:{field}:{value}")
}
