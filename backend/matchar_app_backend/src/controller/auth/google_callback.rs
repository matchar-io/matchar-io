use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Extension,
};
use database::ConnectionPool;
use matchar_app_adapter::auth::google_callback::Adapter;
use matchar_app_service::auth::google_callback::{Data, Error, Service};

#[derive(Deserialize)]
pub struct Parameter {
    code: String,
    state: String,
}

pub enum ErrorKind {
    Service(Error),
}

pub async fn handler(
    Extension(pool): Extension<ConnectionPool>,
    Query(parameter): Query<Parameter>,
) -> Result<(crate::GeneratedSessionToken, Redirect), ErrorKind> {
    let adapter = Adapter::new(pool).map_err(ErrorKind::Service)?;
    let Data {
        session_id,
        name,
        image_url,
        from_url,
    } = Service::new(adapter)
        .execute(parameter.code, parameter.state)
        .await
        .map_err(ErrorKind::Service)?;
    let session_token = crate::GeneratedSessionToken::new(session_id, name, image_url);

    Ok((session_token, Redirect::to(from_url.as_str())))
}

impl IntoResponse for ErrorKind {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        match self {
            ErrorKind::Service(Error::NoMatched) => StatusCode::UNAUTHORIZED.into_response(),
            ErrorKind::Service(Error::Expired) => StatusCode::GONE.into_response(),
            ErrorKind::Service(Error::Verify(_)) => StatusCode::BAD_REQUEST.into_response(),
            ErrorKind::Service(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)).into_response()
            }
        }
    }
}
