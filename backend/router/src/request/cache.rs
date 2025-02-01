use super::{FromRequestParts, Parts};
use crate::Extension;

#[derive(Clone)]
pub struct Cached<T>(pub T);

#[derive(Clone)]
struct CachedEntry<T>(T);

impl<T> FromRequestParts for Cached<T>
where
    T: FromRequestParts + Clone + Sync + Send + 'static,
{
    type Rejection = T::Rejection;

    async fn from_request_parts(parts: &mut Parts) -> Result<Self, Self::Rejection> {
        match Extension::<CachedEntry<T>>::from_request_parts(parts).await {
            Ok(Extension(CachedEntry(value))) => Ok(Self(value)),
            Err(_) => {
                let value = T::from_request_parts(parts).await?;
                parts
                    .extensions
                    .insert(Extension(CachedEntry(value.clone())));

                Ok(Self(value))
            }
        }
    }
}
