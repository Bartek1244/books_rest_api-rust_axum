use sqlx::PgPool;
use async_trait::async_trait;
use crate::{model::author_model::Author};

#[async_trait]
pub trait AuthorRepository: Send + Sync {
    async fn get_all_authors(&self) -> Result<Vec<Author>, sqlx::Error>;
}

#[derive(Clone)]
pub struct PostgresAuthorRepository {
    pool: PgPool,    
}

impl PostgresAuthorRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {pool}
    }
}

#[async_trait]
impl AuthorRepository for PostgresAuthorRepository {
    async fn get_all_authors(&self) -> Result<Vec<Author>, sqlx::Error> {
        sqlx::query_as!(
            Author,
            "SELECT * FROM book_library.author"
        )
        .fetch_all(&self.pool)
        .await
    }
}