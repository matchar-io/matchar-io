use serde::Serialize;
use serde_json::Value;
use std::borrow::Cow;

pub trait IntoResponse {
    fn into_response(self) -> Response;
}

pub enum Response {
    Empty(()),
    Event(String),
    Error(anyhow::Error),
}

pub struct Event {
    r#type: Cow<'static, str>,
    payload: Value,
}

impl Response {
    #[inline]
    pub const fn empty() -> Self {
        Self::Empty(())
    }

    #[inline]
    pub const fn event(body: String) -> Self {
        Self::Event(body)
    }

    #[inline]
    pub fn error<E>(error: E) -> Self
    where
        E: Into<anyhow::Error>,
    {
        Self::Error(error.into())
    }
}

impl Event {
    pub fn new<K, P>(r#type: K, payload: P) -> Result<Self, serde_json::Error>
    where
        K: Into<Cow<'static, str>>,
        P: Serialize,
    {
        Ok(Self {
            r#type: r#type.into(),
            payload: serde_json::to_value(payload)?,
        })
    }
}

impl IntoResponse for () {
    #[inline]
    fn into_response(self) -> Response {
        Response::empty()
    }
}

impl<T> IntoResponse for (String, T)
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        Event::new(self.0, self.1)
            .map(IntoResponse::into_response)
            .unwrap_or_else(Response::error)
    }
}

impl<T> IntoResponse for (&'static str, T)
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        Event::new(self.0, self.1)
            .map(IntoResponse::into_response)
            .unwrap_or_else(Response::error)
    }
}

impl<T> IntoResponse for Option<T>
where
    T: IntoResponse,
{
    fn into_response(self) -> Response {
        match self {
            Some(value) => value.into_response(),
            None => Response::empty(),
        }
    }
}

impl<T, E> IntoResponse for Result<T, E>
where
    T: IntoResponse,
    E: IntoResponse,
{
    fn into_response(self) -> Response {
        self.map(IntoResponse::into_response)
            .unwrap_or_else(IntoResponse::into_response)
    }
}

impl IntoResponse for std::convert::Infallible {
    #[inline]
    fn into_response(self) -> Response {
        Response::empty()
    }
}

impl IntoResponse for Response {
    #[inline]
    fn into_response(self) -> Response {
        self
    }
}

impl IntoResponse for Event {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct Body {
            r#type: Cow<'static, str>,
            payload: Value,
        }

        serde_json::to_string(&Body {
            r#type: self.r#type,
            payload: self.payload,
        })
        .map(Response::event)
        .unwrap_or_else(Response::error)
    }
}
