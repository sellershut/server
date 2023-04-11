use entity::{
    async_graphql::{self, SimpleObject},
    category::{self, Entity as Category},
    sea_orm::{DbConn, DbErr, EntityTrait, PaginatorTrait, QueryOrder},
};

use crate::Query;

#[derive(Default)]
pub struct CategoryQuery;

impl Query {
    pub async fn find_category_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<Option<category::Model>, DbErr> {
        Category::find_by_id(id).one(db).await
    }

    /// If ok, returns an object with Categories and the number of pages.
    pub async fn find_categories_in_page(
        db: &DbConn,
        page: u64,
        max_per_page: u64,
        parent_id: Option<i32>,
    ) -> Result<(Vec<category::Model>, u64), DbErr> {
        let entities = if let Some(parent_id) = parent_id {
            category::Entity::find_by_parent_id(parent_id)
        } else {
            category::Entity::find()
        };
        // Setup paginator
        let paginator = entities
            .order_by_asc(category::Column::Id)
            .paginate(db, max_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated categories
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
