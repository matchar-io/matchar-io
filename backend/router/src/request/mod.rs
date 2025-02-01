pub mod cache;
pub mod json;

pub use cache::*;
pub use json::*;

use crate::{Extensions, IntoResponse};
use serde_json::Value;
use std::future::Future;

pub trait FromRequest: Sized {
    type Rejection: IntoResponse;

    fn from_request(
        request: &mut Request,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send;
}

pub struct Request {
    pub(crate) extensions: Extensions,
    pub(crate) value: Value,
}

impl Request {
    #[inline]
    pub const fn new(value: Value, extensions: Extensions) -> Self {
        Self { extensions, value }
    }
}
