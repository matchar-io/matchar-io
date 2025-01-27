use super::ReceivedSessionTokenError;
use crate::ReceivedSessionToken;
use axum::{
    body::Body,
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, Response},
};
use axum_extra::extract::Cached;
use refinement::SessionId;

#[derive(Debug, Clone, Copy)]
pub struct Session(pub InnerSession);

#[derive(Debug, Clone, Copy)]
pub struct InnerSession {
    session_id: SessionId,
}

pub enum SessionError {
    SessionToken(ReceivedSessionTokenError),
}

impl InnerSession {
    pub const fn session_id(self) -> SessionId {
        self.session_id
    }
}

impl<S> FromRequestParts<S> for Session
where
    S: Send + Sync,
{
    type Rejection = SessionError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Cached(ReceivedSessionToken(session_token)) = Cached::from_request_parts(parts, state)
            .await
            .map_err(SessionError::SessionToken)?;
        let session_id = session_token.session_id();

        Ok(Self(InnerSession { session_id }))
    }
}

impl IntoResponse for SessionError {
    fn into_response(self) -> Response<Body> {
        match self {
            Self::SessionToken(error) => error.into_response(),
        }
    }
}
