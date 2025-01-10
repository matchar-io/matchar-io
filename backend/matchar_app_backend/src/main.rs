#[macro_use]
extern crate serde;

mod app;
mod controller;
mod domain;
mod shared;

use domain::*;
use shared::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::new().await?;
    app.launch().await?;

    Ok(())
}
