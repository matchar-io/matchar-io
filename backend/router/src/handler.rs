use crate::{FromRequest, FromRequestParts, IntoResponse, Request, Response};
use std::{future::Future, pin::Pin};

pub trait Handler<T>: Clone + Sized + Sync + Send + 'static {
    type Future: Future<Output = Response> + Send + 'static;

    fn call(self, request: Request) -> Self::Future;
}

type BoxedHandler = Box<dyn Fn(Request) -> Pin<Box<dyn Future<Output = Response> + Send>> + Send>;

pub struct EventHandler {
    pub handler: BoxedHandler,
}

impl EventHandler {
    pub fn new<T>(handler: impl Handler<T>) -> Self {
        Self {
            handler: Box::new(move |request| Box::pin(handler.clone().call(request))),
        }
    }

    pub async fn call(&self, request: Request) -> Response {
        (self.handler)(request).await
    }
}

impl<F, Fut, Res> Handler<()> for F
where
    F: FnOnce() -> Fut + Clone + Sync + Send + 'static,
    Fut: Future<Output = Res> + Send,
    Res: IntoResponse,
{
    type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

    fn call(self, _: Request) -> Self::Future {
        Box::pin(async move { self().await.into_response() })
    }
}

macro_rules! impl_handler {
    (
        [$($ty:ident),*], $last:ident
    ) => {
        #[allow(non_snake_case, unused_mut, unused_parens)]
        impl<F, Fut, Res, $($ty,)* $last> Handler<($($ty,)* $last)> for F
        where
            F: FnOnce($($ty,)* $last) -> Fut + Clone + Sync + Send + 'static,
            Fut: Future<Output = Res> + Send,
            Res: IntoResponse,
            $($ty: FromRequestParts + Send,)*
            $last: FromRequest<($($ty,)* $last)> + Send,
        {
            type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

            fn call(self, mut request: Request) -> Self::Future {
                Box::pin(async move {
                    $(
                        let $ty = match $ty::from_request_parts(&mut request.parts).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*
                    let $last = match $last::from_request(request).await {
                        Ok($last) => $last,
                        Err(rejection) => return rejection.into_response(),
                    };
                    let response = self($($ty,)* $last).await;

                    response.into_response()
                })
            }
        }
    };
}

impl_handler!([], Last);
impl_handler!([T0], Last);
impl_handler!([T0, T1], Last);
impl_handler!([T0, T1, T2], Last);
impl_handler!([T0, T1, T2, T3], Last);
impl_handler!([T0, T1, T2, T3, T4], Last);
impl_handler!([T0, T1, T2, T3, T4, T5], Last);
impl_handler!([T0, T1, T2, T3, T4, T5, T6], Last);
impl_handler!([T0, T1, T2, T3, T4, T5, T6, T7], Last);
impl_handler!([T0, T1, T2, T3, T4, T5, T6, T7, T8], Last);
impl_handler!([T0, T1, T2, T3, T4, T5, T6, T7, T8, T9], Last);
impl_handler!([T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10], Last);
impl_handler!([T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11], Last);
impl_handler!(
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12],
    Last
);
impl_handler!(
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13],
    Last
);
impl_handler!(
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14],
    Last
);
impl_handler!(
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15],
    Last
);
impl_handler!(
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16],
    Last
);
impl_handler!(
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17],
    Last
);
impl_handler!(
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18],
    Last
);
impl_handler!(
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19],
    Last
);
impl_handler!(
    [
        T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19,
        T20
    ],
    Last
);
impl_handler!(
    [
        T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19,
        T20, T21
    ],
    Last
);
impl_handler!(
    [
        T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19,
        T20, T21, T22
    ],
    Last
);
impl_handler!(
    [
        T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19,
        T20, T21, T22, T23
    ],
    Last
);
impl_handler!(
    [
        T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19,
        T20, T21, T22, T23, T24
    ],
    Last
);
impl_handler!(
    [
        T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19,
        T20, T21, T22, T23, T24, T25
    ],
    Last
);
impl_handler!(
    [
        T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19,
        T20, T21, T22, T23, T24, T25, T26
    ],
    Last
);
impl_handler!(
    [
        T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19,
        T20, T21, T22, T23, T24, T25, T26, T27
    ],
    Last
);
impl_handler!(
    [
        T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19,
        T20, T21, T22, T23, T24, T25, T26, T27, T28
    ],
    Last
);
impl_handler!(
    [
        T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19,
        T20, T21, T22, T23, T24, T25, T26, T27, T28, T29
    ],
    Last
);
impl_handler!(
    [
        T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19,
        T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30
    ],
    Last
);
impl_handler!(
    [
        T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19,
        T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31
    ],
    Last
);
