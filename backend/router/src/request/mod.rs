pub mod body;
pub mod cache;
pub mod part;

pub use body::*;
pub use cache::*;
pub use part::*;

use crate::{Extensions, IntoResponse};
use serde_json::Value;
use std::future::Future;

pub trait FromRequest<T>: Sized {
    type Rejection: IntoResponse;

    fn from_request(request: Request)
        -> impl Future<Output = Result<Self, Self::Rejection>> + Send;
}

pub struct Request {
    pub(crate) parts: Parts,
    pub(crate) body: Value,
}

impl Request {
    #[inline]
    pub const fn new(body: Value, extensions: Extensions) -> Self {
        Self {
            parts: Parts::new(extensions),
            body,
        }
    }
}
