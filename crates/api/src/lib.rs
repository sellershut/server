mod query;

use entity::sea_orm::DatabaseConnection;

pub struct Database {
    pub connection: DatabaseConnection,
}

impl Database {
    pub async fn new() -> Self {
        let connection = entity::sea_orm::Database::connect(
            std::env::var("DATABASE_URL").expect("DATABASE_URL was not set"),
        )
        .await
        .expect("Could not connect to database");

        Self { connection }
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}
