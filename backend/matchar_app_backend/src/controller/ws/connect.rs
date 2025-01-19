use axum::{extract::WebSocketUpgrade, response::IntoResponse};

pub async fn handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| async {
        //
    })
}
