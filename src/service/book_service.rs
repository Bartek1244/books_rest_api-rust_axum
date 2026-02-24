use crate::{
    BookRepository, Book, CreateBook, error::AppError, 
};
use std::sync::Arc;

#[derive(Clone)]
pub struct BookService {
    repo: Arc<dyn BookRepository>,
}

impl BookService {
    pub fn new(repo: Arc<dyn BookRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_all_books(&self) -> Result<Vec<Book>, AppError> {
        let books = self
            .repo
            .get_all_books()
            .await?;

        Ok(books)
    }

    pub async fn get_book_by_id(&self, id: i32) -> Result<Book, AppError> {
        let book = self
            .repo
            .get_book_by_id(id)
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(book)
    }

    pub async fn create_book(&self, payload: &CreateBook) -> Result<Book, AppError> {
        let created_book = self
            .repo
            .create_book(&payload)
            .await?;

        Ok(created_book)
    }
}