use axum::{
    Json,
    extract::{FromRequest, rejection::JsonRejection, Request}
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::AppError;

pub struct ValidatedJson<T>(pub T);

impl <T, S> FromRequest<S> for ValidatedJson<T>
where 
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S,) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|_| AppError::JsonParse)?;

        value.validate().map_err(|e| {
            let mut errors = Vec::new();

            for (field, field_errors) in e.field_errors() {
                for field_error in field_errors {
                    if let Some(msg) = &field_error.message {
                        errors.push(format!("field {} {}", field, msg));
                    } else {
                        errors.push(format!("field {} is invalid", field));
                    }
                }
            }

            AppError::Validation(errors.join(", "))
        })?;

        Ok(ValidatedJson(value))
    }
}