use axum::{http::StatusCode, response::Redirect, Extension};
use database::ConnectionPool;
use matchar_app_adapter::auth::google_authorize::Adapter;
use matchar_app_service::auth::google_authorize::{Data, Service};

pub async fn handler(
    Extension(pool): Extension<ConnectionPool>,
) -> Result<Redirect, (StatusCode, String)> {
    let adapter = Adapter::new(pool);
    let Data { redirect_url } = Service::new(adapter)
        .execute()
        .await
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)))?;

    Ok(Redirect::to(redirect_url.as_str()))
}
