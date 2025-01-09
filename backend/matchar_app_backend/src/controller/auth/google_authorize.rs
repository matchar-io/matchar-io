use axum::response::IntoResponse;
use matchar_app_adapter::auth::google_authorize::Adapter;
use matchar_app_service::auth::google_authorize::{Data, Service};

pub struct Error(anyhow::Error);

pub async fn handler() -> Result<String, Error> {
    let adapter = Adapter::new();
    let service = Service::new(adapter);
    let Data {
        redirect_url,
        code_verifier,
    } = service
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
