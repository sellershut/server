use api_core::Query;
use entity::{
    async_graphql::{self, SimpleObject},
    category,
    sea_orm::DbErr,
};

use async_graphql::{Context, Object};
use serde::{Deserialize, Serialize};
use tracing::{error, span, Level};

use crate::{cache::CacheKey, Database};

#[derive(Default)]
pub struct CategoryQuery;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Categories {
    categories: Vec<category::Model>,
    pages: u64,
}

#[Object]
impl CategoryQuery {
    async fn get_category_by_id(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<Option<category::Model>, DbErr> {
        let span = span!(Level::TRACE, "get_category_by_id");
        let _enter = span.enter();
        let (conn, mut redis) = Database::get_connection_from_context(ctx)?;
        let key = CacheKey::Category { id };
        if let Ok(cache) = Database::get_redis_cache::<category::Model>(&key, &mut redis).await {
            Ok(Some(cache))
        } else {
            let val = Query::find_category_by_id(conn, id).await;
            if let Ok(Some(ref cat)) = val {
                if let Err(e) = Database::set_redis_cache(&key, &mut redis, cat).await {
                    error!("{e}");
                }
            }
            val
        }
    }

    /// If ok, returns an object with Categories and the number of pages.
    async fn get_categories(
        &self,
        ctx: &Context<'_>,
        page: u64,
        max_per_page: u64,
        parent_id: Option<i32>,
    ) -> Result<Categories, DbErr> {
        let span = span!(Level::TRACE, "get_categories");
        let _enter = span.enter();
        let (conn, mut redis) = Database::get_connection_from_context(ctx)?;
        let key = CacheKey::Categories {
            page,
            max_per_page,
            parent_id,
        };
        if let Ok(cache) = Database::get_redis_cache::<Categories>(&key, &mut redis).await {
            Ok(cache)
        } else {
            let (categories, pages) =
                Query::find_categories_in_page(conn, page, max_per_page, parent_id).await?;
            let val = Categories { categories, pages };
            if let Err(e) = Database::set_redis_cache(&key, &mut redis, &val).await {
                error!("{e}");
            }
            Ok(val)
        }
    }
}
