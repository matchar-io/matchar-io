use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Extension,
};
use database::ConnectionPool;
use matchar_app_adapter::auth::google_authorize::Adapter;
use matchar_app_service::auth::google_authorize::{Data, Error, Service};

#[derive(Deserialize)]
pub struct Parameter {
    from: String,
}

pub enum ErrorKind {
    MalformedUrl,
    Service(Error),
}

pub async fn handler(
    Extension(pool): Extension<ConnectionPool>,
    Query(parameter): Query<Parameter>,
) -> Result<Redirect, ErrorKind> {
    let from_url = url::Url::parse(parameter.from.as_str()).map_err(|_| ErrorKind::MalformedUrl)?;

    let adapter = Adapter::new(pool);
    let Data { redirect_url } = Service::new(adapter)
        .execute(from_url)
        .await
        .map_err(ErrorKind::Service)?;

    Ok(Redirect::to(redirect_url.as_str()))
}

impl IntoResponse for ErrorKind {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        match self {
            ErrorKind::MalformedUrl => StatusCode::BAD_REQUEST.into_response(),
            ErrorKind::Service(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)).into_response()
            }
        }
    }
}
