use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::IntoResponse,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use refinement::SessionId;
use time::{Duration, OffsetDateTime, UtcOffset};

pub struct SessionToken {
    session_id: SessionId,
    expired_at: OffsetDateTime,
}

pub enum SessionTokenError {
    NoCookies,
    NoCookie,
    Invalid,
    InvalidSessionId,
    InvalidTimestamp,
    Encoding,
}

#[derive(Serialize, Deserialize)]
struct Claim {
    session_id: String,
    exp: usize,
}

impl SessionToken {
    const KEY: &'static str = "matchar::session_token";
    const EXPIRING_DAYS: Duration = Duration::days(30);

    pub fn new(session_id: SessionId) -> Self {
        Self {
            session_id,
            expired_at: OffsetDateTime::now_utc() + Self::EXPIRING_DAYS,
        }
    }
}

impl std::str::FromStr for SessionToken {
    type Err = SessionTokenError;

    fn from_str(token: &str) -> Result<Self, Self::Err> {
        let decoding_key = DecodingKey::from_secret(crate::SESSION_SECRET.as_bytes());
        let validation = Validation::default();
        let TokenData { claims, .. } =
            jsonwebtoken::decode::<Claim>(token, &decoding_key, &validation)
                .map_err(|_| SessionTokenError::Invalid)?;
        let session_id = claims
            .session_id
            .parse()
            .map_err(|_| SessionTokenError::InvalidSessionId)?;
        let expired_at = OffsetDateTime::from_unix_timestamp(claims.exp as i64)
            .map_err(|_| SessionTokenError::InvalidTimestamp)?;

        Ok(Self {
            session_id,
            expired_at,
        })
    }
}

impl IntoResponse for SessionToken {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        let token = {
            let header = Header::new(Algorithm::HS256);
            let claim = Claim {
                session_id: self.session_id.to_string(),
                exp: self.expired_at.to_offset(UtcOffset::UTC).unix_timestamp() as usize,
            };
            let encoding_key = EncodingKey::from_secret(crate::SESSION_SECRET.as_bytes());

            match jsonwebtoken::encode(&header, &claim, &encoding_key) {
                Ok(token) => token,
                Err(error) => return SessionTokenError::Encoding.into_response(),
            }
        };
        let cookie = Cookie::build((Self::KEY, token))
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Strict)
            .max_age(Self::EXPIRING_DAYS)
            .path("/")
            .domain(crate::DOMAIN)
            .build();
        let cookie_jar = CookieJar::new().add(cookie);

        cookie_jar.into_response()
    }
}

impl<S> FromRequestParts<S> for SessionToken
where
    S: Send + Sync,
{
    type Rejection = SessionTokenError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookie_jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| SessionTokenError::NoCookies)?;
        let cookie = cookie_jar
            .get(Self::KEY)
            .ok_or(SessionTokenError::NoCookie)?;
        let session_token = cookie.value().parse()?;

        Ok(session_token)
    }
}

impl IntoResponse for SessionTokenError {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        match self {
            Self::NoCookies | Self::NoCookie | Self::Invalid | Self::InvalidSessionId => {
                StatusCode::UNAUTHORIZED.into_response()
            }
            Self::InvalidTimestamp | Self::Encoding => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
