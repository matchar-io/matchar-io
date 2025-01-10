use axum::{extract::Query, http::StatusCode, Extension};
use database::ConnectionPool;
use matchar_app_adapter::auth::google_callback::Adapter;
use matchar_app_service::auth::google_callback::{Data, Service};

#[derive(Deserialize)]
pub struct Parameter {
    code: String,
    state: String,
}

#[derive(Debug)]
pub struct Error(anyhow::Error);

pub async fn handler(
    Extension(pool): Extension<ConnectionPool>,
    Query(parameter): Query<Parameter>,
) -> Result<crate::SessionToken, (StatusCode, String)> {
    let adapter = Adapter::new(pool);
    let service = Service::new(adapter);
    let Data { session_id } = service
        .execute(parameter.code, parameter.state)
        .await
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)))?;
    let session_token = crate::SessionToken::new(session_id);

    Ok(session_token)
}
