use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{DecodingKey, TokenData, Validation};
use refinement::SessionId;
use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub struct ReceivedSessionToken {
    session_id: SessionId,
}

pub enum ReceivedSessionTokenError {
    InvalidToken,
    InvalidSessionId,
    InvalidTimestamp,
    Expired,
    NoCookies,
    NoCookie,
}

impl ReceivedSessionToken {
    #[inline]
    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }
}

impl std::str::FromStr for ReceivedSessionToken {
    type Err = ReceivedSessionTokenError;

    fn from_str(token: &str) -> Result<Self, Self::Err> {
        #[derive(Deserialize)]
        struct Claim {
            session_id: String,
            exp: usize,
        }

        let decoding_key = DecodingKey::from_secret(crate::SESSION_SECRET.as_bytes());
        let validation = Validation::default();
        let TokenData { claims, .. } =
            jsonwebtoken::decode::<Claim>(token, &decoding_key, &validation)
                .map_err(|_| ReceivedSessionTokenError::InvalidToken)?;
        let session_id = claims
            .session_id
            .parse()
            .map_err(|_| ReceivedSessionTokenError::InvalidSessionId)?;
        let expired_at = OffsetDateTime::from_unix_timestamp(claims.exp as i64)
            .map_err(|_| ReceivedSessionTokenError::InvalidTimestamp)?;
        if expired_at < OffsetDateTime::now_utc() {
            return Err(ReceivedSessionTokenError::Expired);
        }

        Ok(Self { session_id })
    }
}

impl<S> FromRequestParts<S> for ReceivedSessionToken
where
    S: Send + Sync,
{
    type Rejection = ReceivedSessionTokenError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookie_jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| ReceivedSessionTokenError::NoCookies)?;
        let cookie = cookie_jar
            .get(super::KEY)
            .ok_or(ReceivedSessionTokenError::NoCookie)?;
        let session_token = cookie.value().parse()?;

        Ok(session_token)
    }
}

impl IntoResponse for ReceivedSessionTokenError {
    fn into_response(self) -> Response<Body> {
        match self {
            Self::InvalidToken
            | Self::InvalidSessionId
            | Self::InvalidTimestamp
            | Self::Expired
            | Self::NoCookies
            | Self::NoCookie => StatusCode::UNAUTHORIZED.into_response(),
        }
    }
}
