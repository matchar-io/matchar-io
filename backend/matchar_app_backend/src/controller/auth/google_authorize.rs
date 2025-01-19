use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Extension,
};
use database::ConnectionPool;
use matchar_app_adapter::auth::google_authorize::Adapter;
use matchar_app_service::auth::google_authorize::{inbound, outbound, Error, Service, UseCase};

#[derive(Deserialize)]
pub struct Parameter {
    from: String,
}

pub enum ErrorKind {
    #[allow(dead_code)]
    Data(inbound::Error),
    Service(Error),
}

pub async fn handler(
    Extension(pool): Extension<ConnectionPool>,
    Query(parameter): Query<Parameter>,
) -> Result<Redirect, ErrorKind> {
    let data = inbound::Data::new(&parameter.from).map_err(ErrorKind::Data)?;
    let adapter = Adapter::new(pool);
    let outbound::Data { redirect_url } = Service::new(adapter)
        .google_authorize(data)
        .await
        .map_err(ErrorKind::Service)?;

    Ok(Redirect::to(redirect_url.as_str()))
}

impl IntoResponse for ErrorKind {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        match self {
            ErrorKind::Data(_) => StatusCode::BAD_REQUEST.into_response(),
            ErrorKind::Service(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)).into_response()
            }
        }
    }
}
