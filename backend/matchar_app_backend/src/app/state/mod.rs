mod database;

pub async fn attach(axum: axum::Router) -> anyhow::Result<axum::Router> {
    let axum = database::attach(axum).await?;

    Ok(axum)
}
