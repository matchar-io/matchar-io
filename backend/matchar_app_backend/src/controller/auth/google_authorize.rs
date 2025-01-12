use axum::{extract::Query, http::StatusCode, response::Redirect, Extension};
use database::ConnectionPool;
use matchar_app_adapter::auth::google_authorize::Adapter;
use matchar_app_service::auth::google_authorize::{Data, Service};

#[derive(Deserialize)]
pub struct Parameter {
    from: String,
}

pub async fn handler(
    Extension(pool): Extension<ConnectionPool>,
    Query(parameter): Query<Parameter>,
) -> Result<Redirect, (StatusCode, String)> {
    let from_url = url::Url::parse(parameter.from.as_str())
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid URL".to_string()))?;

    let adapter = Adapter::new(pool);
    let Data { redirect_url } = Service::new(adapter)
        .execute(from_url)
        .await
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)))?;

    Ok(Redirect::to(redirect_url.as_str()))
}
