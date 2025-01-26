use crate::Session;
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Extension, Json};
use database::ConnectionPool;
use matchar_app_repository::user::information::Repository;
use matchar_app_service::user::information::{inbound, outbound, Error, Service, UseCase};
use refinement::{ImageUrl, UserId, UserName};

#[derive(Deserialize)]
pub struct Parameter {
    user_id: String,
}

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
    InvalidUserId(refinement::IdError),
    Service(Error),
}

pub async fn handler(
    Query(query): Query<Parameter>,
    _: Session,
    Extension(pool): Extension<ConnectionPool>,
) -> Result<Json<Response>, ErrorKind> {
    let user_id = query.user_id.parse().map_err(ErrorKind::InvalidUserId)?;
    let now = time::OffsetDateTime::now_utc();
    let data = inbound::Data::new(user_id, now);
    let repository = Repository::new(pool);
    let outbound::Data {
        user_id,
        name,
        image_url,
    } = Service::new(repository)
        .user_information(data)
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
            ErrorKind::InvalidUserId(error) => {
                (StatusCode::BAD_REQUEST, format!("{:?}", error)).into_response()
            }
            ErrorKind::Service(Error::NoMatched) => StatusCode::NOT_FOUND.into_response(),
            ErrorKind::Service(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)).into_response()
            }
        }
    }
}
