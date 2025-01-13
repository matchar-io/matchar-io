use crate::Session;
use axum::{http::StatusCode, Extension, Json};
use database::ConnectionPool;
use matchar_app_adapter::me::information::Adapter;
use matchar_app_service::me::information::{Data, Service};
use refinement::{ImageUrl, UserId, UserName};

#[derive(Serialize)]
pub struct Response {
    user: User,
}

#[derive(Serialize)]
pub struct User {
    user_id: UserId,
    name: UserName,
    image_url: ImageUrl,
}

pub async fn handler(
    session: Session,
    Extension(pool): Extension<ConnectionPool>,
) -> Result<Json<Response>, (StatusCode, String)> {
    let adapter = Adapter::new(pool);
    let Data {
        user_id,
        name,
        image_url,
    } = Service::new(adapter)
        .execute(session.session_id())
        .await
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)))?;
    let user = User {
        user_id,
        name,
        image_url,
    };

    Ok(Json(Response { user }))
}
