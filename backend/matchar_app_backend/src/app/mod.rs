use std::net::IpAddr;

pub struct App {
    axum: axum::Router,
}

impl App {
    pub fn new() -> Self {
        let axum = axum::Router::new();
        let axum = crate::controller::routes(axum);

        App { axum }
    }

    pub async fn launch(self) -> Result<(), Box<dyn std::error::Error>> {
        let ip_address = IpAddr::from([127, 0, 0, 1]);
        let listener = tokio::net::TcpListener::bind((ip_address, crate::PORT)).await?;
        axum::serve(listener, self.axum).await?;

        Ok(())
    }
}
