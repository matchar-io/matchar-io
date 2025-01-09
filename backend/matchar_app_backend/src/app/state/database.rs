use axum::Extension;

pub async fn attach(axum: axum::Router) -> anyhow::Result<axum::Router> {
    let pool = database::ConnectionPool::new(crate::DATABASE_URL).await?;
    let axum = axum.layer(Extension(pool));

    Ok(axum)
}
