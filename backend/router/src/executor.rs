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

#[derive(Deserialize)]
struct Event {
    #[serde(rename = "type")]
    path: String,
    #[serde(rename = "payload")]
    body: Value,
}

impl Executor {
    pub const fn new(router: Router) -> Self {
        Self { router }
    }

    pub async fn from_str(&self, source: &str) -> Response {
        match source.parse() {
            Ok(Event { path, body }) => self.execute(&path, body).await,
            Err(error) => error.into_response(),
        }
    }

    pub async fn execute(&self, path: &str, body: Value) -> Response {
        match self.router.router.at(path) {
            Ok(matchit::Match {
                params,
                value: handler,
            }) => handler.call(Request::new(params.into(), body)).await,
            Err(_) => Response::error(Error::NotFound),
        }
    }
}

impl std::str::FromStr for Event {
    type Err = Error;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(source).map_err(Error::Deserialize)
    }
}

impl IntoResponse for Error {
    #[inline]
    fn into_response(self) -> Response {
        Response::error(self)
    }
}
