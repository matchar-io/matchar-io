use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::IntoResponse,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};

pub struct SessionToken {
    value: String,
}

pub enum SessionTokenError {
    NoCookies,
    NoCookie,
    Expired,
}

impl SessionToken {
    const KEY: &'static str = "matchar_session_token";

    pub fn new<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            value: value.into(),
        }
    }
}

impl std::str::FromStr for SessionToken {
    type Err = SessionTokenError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        // TODO: JWT 검증 추가.
        Ok(Self::new(source))
    }
}

impl IntoResponse for SessionToken {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        let cookie = Cookie::build((Self::KEY, self.value))
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Strict)
            .max_age(time::Duration::days(365))
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
            Self::NoCookies | Self::NoCookie | Self::Expired => {
                StatusCode::UNAUTHORIZED.into_response()
            }
        }
    }
}
