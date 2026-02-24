use sqlx::PgPool;
use async_trait::async_trait;
use crate::{model::book_model::Book, model::book_model::CreateBook};

#[async_trait]
pub trait BookRepository: Send + Sync {
    async fn get_all_books(&self) -> Result<Vec<Book>, sqlx::Error>;
    async fn get_book_by_id(&self, id: i32) -> Result<Option<Book>, sqlx::Error>;
    async fn create_book(&self, payload: &CreateBook) -> Result<Book, sqlx::Error>;
}

#[derive(Clone)]
pub struct PostgresBookRepository {
    pool: PgPool,    
}

impl PostgresBookRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {pool}
    }
}

#[async_trait]
impl BookRepository for PostgresBookRepository {
    async fn get_all_books(&self) -> Result<Vec<Book>, sqlx::Error> {
        sqlx::query_as!(
            Book,
            "SELECT id, title, author FROM test.book"
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn get_book_by_id(&self, id: i32) -> Result<Option<Book>, sqlx::Error> {
        sqlx::query_as!(
            Book,
            "SELECT id, title, author FROM test.book WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn create_book(&self, payload: &CreateBook) -> Result<Book, sqlx::Error> {
        sqlx::query_as!(
            Book,
            "INSERT INTO test.book (title, author) VALUES ($1, $2) RETURNING id, title, author",
            payload.title,
            payload.author
        )
        .fetch_one(&self.pool)
        .await
    }
}