use super::{FromRequest, Request};
use crate::Extension;

#[derive(Clone)]
pub struct Cached<T>(pub T);

#[derive(Clone)]
struct CachedEntry<T>(T);

impl<T> FromRequest for Cached<T>
where
    T: FromRequest + Clone + Sync + Send + 'static,
{
    type Rejection = T::Rejection;

    async fn from_request(request: &mut Request) -> Result<Self, Self::Rejection> {
        match Extension::<CachedEntry<T>>::from_request(request).await {
            Ok(Extension(CachedEntry(value))) => Ok(Self(value)),
            Err(_) => {
                let value = T::from_request(request).await?;
                request
                    .extensions
                    .insert(Extension(CachedEntry(value.clone())));

                Ok(Self(value))
            }
        }
    }
}
