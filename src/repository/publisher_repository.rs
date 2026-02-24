use sqlx::PgPool;
use async_trait::async_trait;
use crate::{model::publisher_model::Publisher};

#[async_trait]
pub trait PublisherRepository: Send + Sync {
    async fn get_all_publishers(&self) -> Result<Vec<Publisher>, sqlx::Error>;
}

#[derive(Clone)]
pub struct PostgresPublisherRepository {
    pool: PgPool,    
}

impl PostgresPublisherRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {pool}
    }
}

#[async_trait]
impl PublisherRepository for PostgresPublisherRepository {
    async fn get_all_publishers(&self) -> Result<Vec<Publisher>, sqlx::Error> {
        sqlx::query_as!(
            Publisher,
            "SELECT * FROM book_library.publisher"
        )
        .fetch_all(&self.pool)
        .await
    }
}