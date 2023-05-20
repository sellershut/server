use api_core::Query;
use entity::{
    async_graphql::{self, SimpleObject},
    category,
    sea_orm::DbErr,
};

use async_graphql::{Context, Object};
use serde::{Deserialize, Serialize};

use crate::Database;

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
        let (conn, mut redis) = Database::get_connection_from_context(ctx)?;
        let key = category_schema("id", id);
        if let Ok(cache) = Database::get_redis_cache::<category::Model>(&key, &mut redis).await {
            Ok(Some(cache))
        } else {
            let val = Query::find_category_by_id(conn, id).await;
            if let Ok(Some(ref cat)) = val {
                if let Err(e) = Database::set_redis_cache(&key, &mut redis, cat).await {
                    println!("{e}");
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
        let (conn, mut redis) = Database::get_connection_from_context(ctx)?;
        let key = format!(
            "category:all:page={page}:max={max_per_page}{}",
            match parent_id {
                Some(parent) => format!(":parent:{parent}"),
                None => "".to_string(),
            }
        );
        if let Ok(cache) = Database::get_redis_cache::<Categories>(&key, &mut redis).await {
            Ok(cache)
        } else {
            let (categories, pages) =
                Query::find_categories_in_page(conn, page, max_per_page, parent_id).await?;
            let val = Categories { categories, pages };
            if let Err(e) = Database::set_redis_cache(&key, &mut redis, &val).await {
                println!("{e}");
            }
            Ok(val)
        }
    }
}

fn category_schema(field: &str, value: i32) -> String {
    format!("category:{field}:{value}")
}
