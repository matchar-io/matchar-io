pub mod extension;

pub use extension::*;

use crate::{
    handler::{BoxedHandler, Handler},
    Request, Response,
};
use serde_json::Value;
use std::collections::HashMap;

pub struct Router {
    extensions: Extensions,
    routes: HashMap<&'static str, BoxedHandler>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            extensions: Extensions::new(),
        }
    }

    pub fn on<T>(mut self, path: &'static str, handler: impl Handler<T>) -> Self {
        self.routes.insert(
            path,
            Box::new(move |request| Box::pin(handler.clone().call(request))),
        );

        self
    }

    pub fn extension<T: Clone + Send + 'static>(mut self, extension: Extension<T>) -> Self {
        self.extensions.insert(extension);

        self
    }

    pub async fn execute(&self, path: &str, body: Value) -> Option<Response> {
        let handler = self.routes.get(path)?;

        let extensions = self.extensions.clone_all();
        let request = Request::new(body, extensions);

        Some(handler(request).await)
    }
}
