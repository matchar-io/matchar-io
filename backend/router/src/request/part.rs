use crate::{Extensions, IntoResponse, Parameters};
use std::future::Future;

pub trait FromRequestParts: Sized {
    type Rejection: IntoResponse;

    fn from_request_parts(
        parts: &mut Parts,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send;
}

pub struct Parts {
    pub(crate) parameters: Parameters,
    pub(crate) extensions: Extensions,
}

impl Parts {
    pub(crate) fn new(parameters: Parameters) -> Self {
        Self {
            parameters,
            extensions: Extensions::new(),
        }
    }
}
