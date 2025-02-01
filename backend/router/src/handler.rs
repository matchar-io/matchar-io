use crate::{FromRequest, IntoResponse, Request, Response};
use std::{future::Future, pin::Pin};

pub trait Handler<T>: Clone + Sized + Sync + Send + 'static {
    type Future: Future<Output = Response> + Send + 'static;

    fn call(self, request: Request) -> Self::Future;
}

pub type BoxedHandler =
    Box<dyn Fn(Request) -> Pin<Box<dyn Future<Output = Response> + Send>> + Send>;

macro_rules! impl_handler {
    (
        $($ty:ident),*
    ) => {
        #[allow(non_snake_case, unused)]
        impl<F, Fut, Res, $($ty,)*> Handler<($($ty,)*)> for F
        where
            F: FnOnce($($ty,)*) -> Fut + Clone + Sync + Send + 'static,
            Fut: Future<Output = Res> + Send,
            Res: IntoResponse,
            $($ty: FromRequest + Send,)*
        {
            type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

            fn call(self, mut request: Request) -> Self::Future {
                Box::pin(async move {
                    $(
                        let $ty = match $ty::from_request(&mut request).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*
                    let response = self($($ty,)*).await;

                    response.into_response()
                })
            }
        }
    };
}

impl_handler!();
impl_handler!(T1);
impl_handler!(T1, T2);
impl_handler!(T1, T2, T3);
impl_handler!(T1, T2, T3, T4);
impl_handler!(T1, T2, T3, T4, T5);
impl_handler!(T1, T2, T3, T4, T5, T6);
impl_handler!(T1, T2, T3, T4, T5, T6, T7);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
impl_handler!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20
);
impl_handler!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21
);
impl_handler!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22
);
impl_handler!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23
);
impl_handler!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24
);
impl_handler!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25
);
impl_handler!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26
);
impl_handler!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27
);
impl_handler!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28
);
impl_handler!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29
);
impl_handler!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29, T30
);
impl_handler!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29, T30, T31
);
