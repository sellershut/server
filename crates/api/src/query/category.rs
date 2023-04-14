use api_core::Query;
use entity::{
    async_graphql::{self, SimpleObject},
    category,
    sea_orm::DbErr,
};

use async_graphql::{Context, Object};

use crate::Database;

#[derive(Default)]
pub struct CategoryQuery;

#[derive(SimpleObject)]
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
        let conn = Database::get_connection_from_context(ctx)?;

        Query::find_category_by_id(conn, id).await
    }

    /// If ok, returns an object with Categories and the number of pages.
    async fn get_categories(
        &self,
        ctx: &Context<'_>,
        page: u64,
        max_per_page: u64,
        parent_id: Option<i32>,
    ) -> Result<Categories, DbErr> {
        let conn = Database::get_connection_from_context(ctx)?;

        let (categories, pages) =
            Query::find_categories_in_page(conn, page, max_per_page, parent_id).await?;
        Ok(Categories { categories, pages })
    }
}
