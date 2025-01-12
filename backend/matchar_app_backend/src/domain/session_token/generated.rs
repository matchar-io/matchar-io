use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, IntoResponseParts, Response, ResponseParts},
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use refinement::{ImageUrl, SessionId, UserName};
use time::{OffsetDateTime, UtcOffset};

pub struct GeneratedSessionToken {
    session_id: SessionId,
    name: UserName,
    image_url: ImageUrl,
    expired_at: OffsetDateTime,
}

pub enum GeneratedSessionTokenError {
    Encoding,
    Infallible,
}

#[derive(Serialize)]
struct Claim {
    session_id: String,
    name: String,
    image_url: String,
    exp: usize,
}

impl GeneratedSessionToken {
    pub fn new(session_id: SessionId, name: UserName, image_url: ImageUrl) -> Self {
        Self {
            session_id,
            name,
            image_url,
            expired_at: OffsetDateTime::now_utc() + super::EXPIRING_DAYS,
        }
    }
}

impl IntoResponseParts for GeneratedSessionToken {
    type Error = GeneratedSessionTokenError;

    fn into_response_parts(self, response: ResponseParts) -> Result<ResponseParts, Self::Error> {
        let token = {
            let header = Header::new(Algorithm::HS256);
            let claim = Claim {
                session_id: self.session_id.to_string(),
                name: self.name.to_string(),
                image_url: self.image_url.to_string(),
                exp: self.expired_at.to_offset(UtcOffset::UTC).unix_timestamp() as usize,
            };
            let encoding_key = EncodingKey::from_secret(crate::SESSION_SECRET.as_bytes());

            match jsonwebtoken::encode(&header, &claim, &encoding_key) {
                Ok(token) => token,
                Err(_) => return Err(GeneratedSessionTokenError::Encoding),
            }
        };
        let cookie = Cookie::build((super::KEY, token))
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Strict)
            .max_age(super::EXPIRING_DAYS)
            .path("/")
            .domain(crate::DOMAIN)
            .build();
        let cookie_jar = CookieJar::new().add(cookie);

        match cookie_jar.into_response_parts(response) {
            Ok(response) => Ok(response),
            Err(_) => Err(GeneratedSessionTokenError::Infallible),
        }
    }
}

impl IntoResponse for GeneratedSessionTokenError {
    fn into_response(self) -> Response<Body> {
        match self {
            Self::Encoding | Self::Infallible => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
