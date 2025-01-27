mod database;
mod postbox;

pub async fn attach(axum: axum::Router) -> anyhow::Result<axum::Router> {
    let axum = database::attach(axum).await?;
    let axum = postbox::attach(axum);

    Ok(axum)
}
