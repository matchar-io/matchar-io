use crate::Session;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use database::ConnectionPool;
use matchar_app_adapter::me::information::Adapter;
use matchar_app_service::me::information::{Data, Error, Service};
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
    session: Session,
    Extension(pool): Extension<ConnectionPool>,
) -> Result<Json<Response>, ErrorKind> {
    let adapter = Adapter::new(pool);
    let Data {
        user_id,
        name,
        image_url,
    } = Service::new(adapter)
        .execute(session.session_id())
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
