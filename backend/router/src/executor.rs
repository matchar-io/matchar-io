use crate::{IntoResponse, Response, Router};
use serde_json::Value;

pub struct Executor {
    router: Router,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Not found")]
    NotFound,
}

impl Executor {
    #[inline]
    pub const fn new(router: Router) -> Self {
        Self { router }
    }

    pub async fn execute(&self, path: &str, value: Value) -> Response {
        match self.router.execute(path, value).await {
            Some(response) => response,
            None => Error::NotFound.into_response(),
        }
    }
}

impl IntoResponse for Error {
    #[inline]
    fn into_response(self) -> Response {
        Response::error(self)
    }
}
