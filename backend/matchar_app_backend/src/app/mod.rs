mod state;

use std::net::Ipv4Addr;

pub struct App {
    axum: axum::Router,
}

impl App {
    pub async fn new() -> anyhow::Result<Self> {
        let axum = axum::Router::new();
        let axum = crate::controller::routes(axum);
        let axum = state::attach(axum).await?;

        Ok(App { axum })
    }

    pub async fn launch(self) -> Result<(), Box<dyn std::error::Error>> {
        let ip_address = Ipv4Addr::new(127, 0, 0, 1);
        let listener = tokio::net::TcpListener::bind((ip_address, crate::PORT)).await?;
        axum::serve(listener, self.axum).await?;

        Ok(())
    }
}
