use super::{FromRequest, FromRequestParts, Parts, Request};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Parameters {
    pub(crate) map: HashMap<String, String>,
}

impl Parameters {
    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn get(&self, key: impl AsRef<str>) -> Option<&str> {
        self.map.get(key.as_ref()).map(|source| source.as_str())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.map
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }

    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.map.keys().map(|key| key.as_str())
    }
}

impl<'k, 'v> From<matchit::Params<'k, 'v>> for Parameters {
    fn from(params: matchit::Params) -> Self {
        let map = params
            .iter()
            .map(|(key, value)| (key.to_owned(), value.to_owned()))
            .collect();

        Self { map }
    }
}

impl FromRequestParts for Parameters {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts) -> Result<Self, Self::Rejection> {
        Ok(parts.parameters.clone())
    }
}

impl<T> FromRequest<T> for Parameters {
    type Rejection = std::convert::Infallible;

    async fn from_request(mut request: Request) -> Result<Self, Self::Rejection> {
        Ok(Parameters::from_request_parts(&mut request.parts).await?)
    }
}
