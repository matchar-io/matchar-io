use crate::Session;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use database::ConnectionPool;
use matchar_app_repository::me::find_one::Repository;
use matchar_app_service::me::find_one::{inbound, outbound, Error, Service, UseCase};
use refinement::{ImageUrl, UserId, UserName};

#[derive(Serialize)]
pub struct Response {
    user: User,
}

#[derive(Serialize)]
pub struct User {
    user_id: UserId,
    name: UserName,
    image_url: ImageUrl,
}

pub enum ErrorKind {
    Service(Error),
}

pub async fn handler(
    Session(session): Session,
    Extension(pool): Extension<ConnectionPool>,
) -> Result<Json<Response>, ErrorKind> {
    let now = time::OffsetDateTime::now_utc();
    let data = inbound::Data::new(session.session_id(), now);
    let repository = Repository::new(pool);
    let outbound::Data {
        user_id,
        name,
        image_url,
    } = Service::new(repository)
        .find_one(data)
        .await
        .map_err(ErrorKind::Service)?;
    let user = User {
        user_id,
        name,
        image_url,
    };

    Ok(Json(Response { user }))
}

impl IntoResponse for ErrorKind {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        match self {
            ErrorKind::Service(Error::NoMatched) => StatusCode::NOT_FOUND.into_response(),
            ErrorKind::Service(Error::Deactivated) => StatusCode::FORBIDDEN.into_response(),
            ErrorKind::Service(Error::Locked) => StatusCode::FORBIDDEN.into_response(),
            ErrorKind::Service(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)).into_response()
            }
        }
    }
}
