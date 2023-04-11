use api_core::Query;
use entity::{
    async_graphql::{self, SimpleObject},
    category,
    sea_orm::{DbErr, EntityTrait, RuntimeErr},
};

use async_graphql::{Context, Object};

use crate::Database;

#[derive(Default)]
pub struct CategoryQuery;

#[derive(SimpleObject)]
pub struct PaginatedCategories {
    categories: Vec<category::Model>,
    pages: u64,
}

#[Object]
impl CategoryQuery {
    async fn find_category_by_id(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<Option<category::Model>, DbErr> {
        let db = ctx
            .data::<Database>()
            .map_err(|e| DbErr::Conn(RuntimeErr::Internal(e.message)))?;
        let conn = db.get_connection();

        Ok(Query::find_category_by_id(conn, id).await?)
    }

    /// If ok, returns an object with Categories and the number of pages.
    async fn find_categories_in_page(
        &self,
        ctx: &Context<'_>,
        page: u64,
        max_per_page: u64,
        parent_id: Option<i32>,
    ) -> Result<PaginatedCategories, DbErr> {
        let db = ctx
            .data::<Database>()
            .map_err(|e| DbErr::Conn(RuntimeErr::Internal(e.message)))?;
        let conn = db.get_connection();

        let (categories, pages) =
            Query::find_categories_in_page(conn, page, max_per_page, parent_id).await?;
        Ok(PaginatedCategories { categories, pages })
    }
}
