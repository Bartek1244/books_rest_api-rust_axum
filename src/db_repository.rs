use sqlx::PgPool;
use crate::{model::book_model::Book, model::book_model::CreateBook};


#[derive(Clone)]
pub struct DbRepository {
    pool: PgPool,    
}

impl DbRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {pool}
    }

    pub async fn get_all_books(&self) -> Result<Vec<Book>, sqlx::Error> {
        sqlx::query_as!(
            Book,
            "SELECT id, title, author FROM test.book"
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_book_by_id(&self, id: i32) -> Result<Option<Book>, sqlx::Error> {
        sqlx::query_as!(
            Book,
            "SELECT id, title, author FROM test.book WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn insert_book(&self, payload: &CreateBook) -> Result<Book, sqlx::Error> {
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