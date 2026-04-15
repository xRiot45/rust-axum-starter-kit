use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;
use crate::common::errors::AppError;

/// Axum extractor that deserializes AND validates the request body.
/// Uses `validator` crate annotations on the target struct.
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| AppError::BadRequest(e.to_string()))?;

        value.validate().map_err(|e| {
            let messages: Vec<String> = e
                .field_errors()
                .iter()
                .flat_map(|(field, errors)| {
                    errors.iter().map(move |err| {
                        format!(
                            "{}: {}",
                            field,
                            err.message.as_ref().map(|m| m.as_ref()).unwrap_or("invalid")
                        )
                    })
                })
                .collect();
            AppError::ValidationError(messages.join(", "))
        })?;

        Ok(ValidatedJson(value))
    }
}
