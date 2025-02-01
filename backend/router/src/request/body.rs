use crate::{IntoResponse, Response};

use super::{FromRequest, Request};
use serde::de::DeserializeOwned;

pub struct Body<T>(pub T);

#[derive(Debug, Error)]
pub enum BodyError {
    #[error("Failed to deserialize body: {0}")]
    Deserialize(serde_json::Error),
}

impl<M, T> FromRequest<M> for Body<T>
where
    T: DeserializeOwned,
{
    type Rejection = BodyError;

    async fn from_request(request: Request) -> Result<Self, Self::Rejection> {
        let body = T::deserialize(&request.body).map_err(BodyError::Deserialize)?;

        Ok(Self(body))
    }
}

impl IntoResponse for BodyError {
    #[inline]
    fn into_response(self) -> Response {
        Response::error(self)
    }
}
