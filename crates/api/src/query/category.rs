use entity::{
    async_graphql::{self, SimpleObject},
    category,
    sea_orm::{DbErr, EntityTrait, PaginatorTrait, QueryOrder, RuntimeErr},
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
        category::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
    }

    /// If ok, returns an object with Categories and the number of pages.
    async fn find_categories_in_page(
        &self,
        ctx: &Context<'_>,
        page: u64,
        posts_per_page: u64,
    ) -> Result<PaginatedCategories, DbErr> {
        let db = ctx
            .data::<Database>()
            .map_err(|e| DbErr::Conn(RuntimeErr::Internal(e.message)))?;
        // Setup paginator
        let paginator = category::Entity::find()
            .order_by_asc(category::Column::Id)
            .paginate(db.get_connection(), posts_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated categories
        paginator
            .fetch_page(page - 1)
            .await
            .map(|p| PaginatedCategories {
                categories: p,
                pages: num_pages,
            })
    }
}
