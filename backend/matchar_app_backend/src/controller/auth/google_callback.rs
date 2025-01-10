use axum::{extract::Query, response::IntoResponse, Extension};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use database::{ConnectionPool, DatabaseDriver};
use matchar_app_adapter::auth::google_callback::Adapter;
use matchar_app_service::auth::google_callback::{Data, Service};
use time::Duration;

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
) -> Result<(CookieJar, ()), Error> {
    let adapter = Adapter::new(pool);
    let service = Service::new(adapter);
    let Data { session_token } = service
        .execute(parameter.code, parameter.state)
        .await
        .map_err(|error| Error(error.into()))?;

    let session_token = Cookie::build(("session_token", session_token.0))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .max_age(Duration::days(365))
        .path("/")
        .domain(crate::DOMAIN)
        .build();
    let cookie_jar = CookieJar::new().add(session_token);

    Ok((cookie_jar, ()))
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        let body = format!("{:?}", self.0);

        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
