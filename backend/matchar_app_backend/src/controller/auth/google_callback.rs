use axum::{extract::Query, http::StatusCode, Extension};
use database::ConnectionPool;
use matchar_app_adapter::auth::google_callback::Adapter;
use matchar_app_service::auth::google_callback::{Data, Error, Service};

#[derive(Deserialize)]
pub struct Parameter {
    code: String,
    state: String,
}

pub async fn handler(
    Extension(pool): Extension<ConnectionPool>,
    Query(parameter): Query<Parameter>,
) -> Result<crate::GeneratedSessionToken, (StatusCode, String)> {
    let adapter = Adapter::new(pool)
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)))?;
    let Data {
        session_id,
        name,
        image_url,
    } = Service::new(adapter)
        .execute(parameter.code, parameter.state)
        .await
        .map_err(|error| match error {
            Error::NoMatched => (StatusCode::UNAUTHORIZED, "NoMatched".to_string()),
            Error::Expired => (StatusCode::BAD_REQUEST, "Expired".to_string()),
            Error::Verify(error)
            | Error::Google(error)
            | Error::Oauth2(error)
            | Error::Database(error) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)),
        })?;
    let session_token = crate::GeneratedSessionToken::new(session_id, name, image_url);

    Ok(session_token)
}
