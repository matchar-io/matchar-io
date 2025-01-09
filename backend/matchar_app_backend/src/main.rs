mod app;
mod controller;
mod shared;

use shared::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::new();
    app.launch().await?;

    Ok(())
}
