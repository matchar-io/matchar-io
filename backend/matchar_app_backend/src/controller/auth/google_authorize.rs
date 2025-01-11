use axum::{response::IntoResponse, Extension};
use database::ConnectionPool;
use matchar_app_adapter::auth::google_authorize::Adapter;
use matchar_app_service::auth::google_authorize::{Data, Service};

pub struct Error(anyhow::Error);

pub async fn handler(Extension(pool): Extension<ConnectionPool>) -> Result<String, Error> {
    let adapter = Adapter::new(pool);
    let Data { redirect_url } = Service::new(adapter)
        .execute()
        .await
        .map_err(|error| Error(error.into()))?;

    Ok(redirect_url.0)
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        let body = format!("{:?}", self.0);

        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
