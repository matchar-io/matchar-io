use axum::{response::IntoResponse, Extension};
use database::{ConnectionPool, DatabaseDriver};
use matchar_app_adapter::auth::google_callback::Adapter;
use matchar_app_service::auth::google_callback::{Data, Service};

#[derive(Debug)]
pub struct Error(anyhow::Error);

pub async fn handler(Extension(pool): Extension<ConnectionPool>) -> Result<String, Error> {
    let adapter = Adapter::new(pool);
    let service = Service::new(adapter);
    let Data { token } = service
        .execute()
        .await
        .map_err(|error| Error(error.into()))?;

    Ok(token.0)
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        let body = format!("{:?}", self.0);

        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
