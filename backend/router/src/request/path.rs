use super::{FromRequest, FromRequestParts, Parameters, Request};
use crate::{IntoResponse, Response};

pub struct Path<T>(pub T);

#[derive(Debug, Error)]
pub enum PathError {
    #[error("Internal error")]
    Internal,
    #[error("Failed to deserialize parameters")]
    FromHashMap(serde_json::Error),
    #[error("Failed to serialize parameters")]
    ToPath(serde_json::Error),
}

impl<T> FromRequestParts for Path<T>
where
    T: serde::de::DeserializeOwned,
{
    type Rejection = PathError;

    async fn from_request_parts(parts: &mut super::Parts) -> Result<Self, Self::Rejection> {
        let parameters = Parameters::from_request_parts(parts)
            .await
            .map_err(|_| PathError::Internal)?;
        let value = serde_json::to_value(parameters.map).map_err(PathError::FromHashMap)?;
        let value = serde_json::from_value(value).map_err(PathError::ToPath)?;

        Ok(Path(value))
    }
}

impl<T> FromRequest<Path<T>> for Path<T>
where
    T: serde::de::DeserializeOwned,
{
    type Rejection = PathError;

    async fn from_request(mut request: Request) -> Result<Self, Self::Rejection> {
        Path::from_request_parts(&mut request.parts).await
    }
}

impl IntoResponse for PathError {
    #[inline]
    fn into_response(self) -> Response {
        Response::error(self)
    }
}
