use super::{FromRequest, Request};
use crate::{IntoResponse, Response};
use serde::de::DeserializeOwned;

pub struct Json<T>(pub T);

#[derive(Debug, Error)]
pub enum JsonError {
    #[error("Failed to deserialize body: {0}")]
    Deserialize(serde_json::Error),
}

impl<T> FromRequest for Json<T>
where
    T: DeserializeOwned,
{
    type Rejection = JsonError;

    async fn from_request(request: &mut Request) -> Result<Self, Self::Rejection> {
        let value = T::deserialize(&request.value).map_err(JsonError::Deserialize)?;

        Ok(Self(value))
    }
}

impl IntoResponse for JsonError {
    #[inline]
    fn into_response(self) -> Response {
        Response::error(self)
    }
}
