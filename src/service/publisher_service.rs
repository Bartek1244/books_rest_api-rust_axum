use crate::{
    error::AppError, repository::publisher_repository::PublisherRepository, model::publisher_model::Publisher
};
use std::sync::Arc;

#[derive(Clone)]
pub struct PublisherService {
    repo: Arc<dyn PublisherRepository>,
}

impl PublisherService {
    pub fn new(repo: Arc<dyn PublisherRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_all_publishers(&self) -> Result<Vec<Publisher>, AppError> {
        let publishers = self
            .repo
            .get_all_publishers()
            .await?;

        Ok(publishers)
    }
}
