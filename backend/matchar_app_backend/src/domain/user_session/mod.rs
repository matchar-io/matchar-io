use super::{Session, SessionError};
use axum::{
    extract::{rejection::ExtensionRejection, FromRequestParts},
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    Extension,
};
use axum_extra::extract::Cached;
use matchar_app_repository::session::find_one::Repository;
use matchar_app_service::session::find_one::{inbound, outbound, Error, Service, UseCase};
use refinement::UserId;

#[derive(Debug, Clone, Copy)]
pub struct UserSession(pub InnerUserSession);

#[derive(Debug, Clone, Copy)]
pub struct InnerUserSession {
    user_id: UserId,
}

pub enum UserSessionError {
    Session(SessionError),
    Database(ExtensionRejection),
    Service(Error),
}

impl InnerUserSession {
    pub const fn user_id(self) -> UserId {
        self.user_id
    }
}

impl<S> FromRequestParts<S> for UserSession
where
    S: Send + Sync,
{
    type Rejection = UserSessionError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Cached(Session(session)) = Cached::from_request_parts(parts, state)
            .await
            .map_err(UserSessionError::Session)?;
        let Extension(pool) = Extension::from_request_parts(parts, state)
            .await
            .map_err(UserSessionError::Database)?;
        let session_id = session.session_id();
        let now = time::OffsetDateTime::now_utc();
        let data = inbound::Data::new(session_id, now);
        let repository = Repository::new(pool);
        let outbound::Data { user_id } = Service::new(repository)
            .find_one(data)
            .await
            .map_err(UserSessionError::Service)?;

        Ok(Self(InnerUserSession { user_id }))
    }
}

impl IntoResponse for UserSessionError {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        match self {
            Self::Session(error) => error.into_response(),
            Self::Database(rejection) => rejection.into_response(),
            Self::Service(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)).into_response()
            }
        }
    }
}
