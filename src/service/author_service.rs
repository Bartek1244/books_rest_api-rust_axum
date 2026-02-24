use crate::{
    error::AppError, repository::author_repository::AuthorRepository, model::author_model::Author 
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthorService {
    repo: Arc<dyn AuthorRepository>,
}

impl AuthorService {
    pub fn new(repo: Arc<dyn AuthorRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_all_authors(&self) -> Result<Vec<Author>, AppError> {
        let authors = self
            .repo
            .get_all_authors()
            .await?;

        Ok(authors)
    }
}
