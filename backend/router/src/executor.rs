use crate::{IntoResponse, Request, Response, Router};
use serde_json::Value;

pub struct Executor {
    router: Router,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Not Found")]
    NotFound,
    #[error("Serde Error: {0}")]
    Deserialize(#[from] serde_json::Error),
}

impl Executor {
    pub const fn new(router: Router) -> Self {
        Self { router }
    }

    pub async fn from_str(&self, source: &str) -> Response {
        #[derive(Deserialize)]
        struct Data {
            #[serde(rename = "type")]
            path: String,
            #[serde(rename = "payload")]
            body: Value,
        }

        match serde_json::from_str(source) {
            Ok(Data { path, body }) => self.execute(&path, body).await,
            Err(error) => Error::Deserialize(error).into_response(),
        }
    }

    pub async fn execute(&self, path: &str, body: Value) -> Response {
        match self.router.execute(path, Request::new(body)).await {
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
