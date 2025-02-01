pub mod extension;

pub use extension::*;

use crate::handler::{EventHandler, Handler};

pub struct Router {
    pub(crate) router: matchit::Router<EventHandler>,
    pub(crate) extensions: Extensions,
}

impl Router {
    pub fn new() -> Self {
        Self {
            router: matchit::Router::new(),
            extensions: Extensions::new(),
        }
    }

    pub fn on<T>(
        mut self,
        path: &str,
        handler: impl Handler<T>,
    ) -> Result<Self, matchit::InsertError> {
        self.router.insert(path, EventHandler::new(handler))?;

        Ok(self)
    }

    pub fn extension<T: Sync + Send + 'static>(mut self, extension: Extension<T>) -> Self {
        self.extensions.insert(extension);

        self
    }
}
