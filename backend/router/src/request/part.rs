use crate::{Extensions, IntoResponse};
use std::future::Future;

pub trait FromRequestParts: Sized {
    type Rejection: IntoResponse;

    fn from_request_parts(
        parts: &mut Parts,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send;
}

pub struct Parts {
    pub(crate) extensions: Extensions,
}

impl Parts {
    #[inline]
    pub(crate) const fn new(extensions: Extensions) -> Self {
        Self { extensions }
    }
}
